// grammalang-core/src/lib.rs

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

mod ontology;
mod error;
mod trace;
mod will_markers;

pub mod token;
pub mod lexer;
pub mod parser;
pub mod ast;
pub mod desugar;
pub mod resolve;
pub mod types;
pub mod infer;
pub mod borrow;
pub mod codegen;
pub mod monomorphize;
pub mod syntax_modifiers;
pub mod lefebvre;
pub mod cascade;
pub mod entity;
pub mod context;
pub mod llm_resolver;
pub mod evaluator;

// ============ Python export ============

#[pyfunction]
fn analyze_will(sentences: Vec<String>) -> PyResult<Vec<f64>> {
    Ok(will_markers::analyze_text(&sentences))
}

#[pyfunction]
fn tokenize_atlas(source: &str) -> PyResult<String> {
    let mut lex = lexer::Lexer::new(source);
    let (tokens, errors) = lex.tokenize();
    if !errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            errors.iter().map(|e| format!("{} at {}", e.message, e.span)).collect::<Vec<_>>().join("\n")
        ));
    }
    Ok(tokens.iter()
        .map(|t| format!("{:?} '{}' @ {}", t.kind, t.lexeme, t.span))
        .collect::<Vec<_>>()
        .join("\n"))
}

#[pyfunction]
fn parse_atlas(source: &str) -> PyResult<String> {
    let mut lex = lexer::Lexer::new(source);
    let (tokens, lex_errors) = lex.tokenize();
    if !lex_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            lex_errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join("\n")
        ));
    }
    let mut parser = parser::Parser::new(tokens);
    let (cst, parse_errors) = parser.parse();
    if !parse_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            parse_errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join("\n")
        ));
    }
    Ok(format!("{:#?}", cst))
}

#[pyfunction]
fn desugar_atlas(source: &str) -> PyResult<String> {
    let mut lex = lexer::Lexer::new(source);
    let (tokens, lex_errors) = lex.tokenize();
    if !lex_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            lex_errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join("\n")
        ));
    }
    let mut parser = parser::Parser::new(tokens);
    let (cst, parse_errors) = parser.parse();
    if !parse_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            parse_errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join("\n")
        ));
    }
    if let Some(cst) = cst {
        let mut desugarer = desugar::Desugarer::new();
        let (ast, desugar_errors) = desugarer.desugar(&cst);
        if !desugar_errors.is_empty() {
            return Err(pyo3::exceptions::PySyntaxError::new_err(
                desugar_errors.iter().map(|e| e.message.clone()).collect::<Vec<_>>().join("\n")
            ));
        }
        Ok(format!("{:#?}", ast))
    } else {
        Err(pyo3::exceptions::PyRuntimeError::new_err("Failed to parse program"))
    }
}

#[pyfunction]
fn compile_atlas(source: &str) -> PyResult<String> {
    // Lexer
    let mut lex = lexer::Lexer::new(source);
    let (tokens, lex_errors) = lex.tokenize();
    if !lex_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            lex_errors.iter().map(|e| format!("Lexer: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }

    // Parser
    let mut parser = parser::Parser::new(tokens);
    let (cst, parse_errors) = parser.parse();
    if !parse_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            parse_errors.iter().map(|e| format!("Parser: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }
    let cst = cst.ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Parser returned no CST"))?;

    // Desugaring
    let mut desugarer = desugar::Desugarer::new();
    let (ast, desugar_errors) = desugarer.desugar(&cst);
    if !desugar_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            desugar_errors.iter().map(|e| format!("Desugar: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }
    let ast = ast.ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Desugaring failed"))?;

    // Name resolution
    let mut resolver = resolve::Resolver::new();
    let (resolved_ast, resolve_errors) = resolver.resolve(&ast);
    if !resolve_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            resolve_errors.iter().map(|e| format!("Resolution: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }
    let resolved_ast = resolved_ast.ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Name resolution failed"))?;

    // Type inference
    let mut inferrer = infer::Inferrer::new();
    let (typed_ast, infer_errors) = inferrer.infer(&resolved_ast);
    if !infer_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            infer_errors.iter().map(|e| format!("Typing: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }
    let typed_ast = typed_ast.ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Type inference failed"))?;

    // Monomorphization of generics
    let mut monomorphizer = monomorphize::Monomorphizer::new();
    let mono_ast = monomorphizer.monomorphize(&typed_ast);

    // Borrow checking
    let mut checker = borrow::BorrowChecker::new();
    let (success, borrow_errors) = checker.check(&mono_ast);
    if !success {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            borrow_errors.iter().map(|e| format!("Borrow: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }

    // Code generation
    let mut codegen = codegen::Codegen::new("main");
    let (ir, codegen_errors) = codegen.generate(&mono_ast);
    if !codegen_errors.is_empty() {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(
            codegen_errors.iter().map(|e| format!("Codegen: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }

    if let Some(ir) = ir {
        let llvm_text = codegen::Codegen::emit_llvm_text(&ir);
        Ok(format!("Compilation successful!\n\nGenerated LLVM IR:\n\n{}", llvm_text))
    } else {
        Err(pyo3::exceptions::PyRuntimeError::new_err("Codegen returned no IR"))
    }
}

#[pyfunction]
fn apply_syntax_modifiers(
    lexical_indices: Vec<f64>,
    syntax_types: Vec<String>,
) -> PyResult<Vec<f64>> {
    syntax_modifiers::apply_syntax_modifiers(lexical_indices, syntax_types)
}

#[pymodule]
fn grammalang_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze_will, m)?)?;
    m.add_function(wrap_pyfunction!(tokenize_atlas, m)?)?;
    m.add_function(wrap_pyfunction!(parse_atlas, m)?)?;
    m.add_function(wrap_pyfunction!(desugar_atlas, m)?)?;
    m.add_function(wrap_pyfunction!(compile_atlas, m)?)?;
    m.add_function(wrap_pyfunction!(apply_syntax_modifiers, m)?)?;
    Ok(())
}
