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

// ============ Экспорт в Python ============

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
            errors.iter().map(|e| format!("{} в {}", e.message, e.span)).collect::<Vec<_>>().join("\n")
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
        Err(pyo3::exceptions::PyRuntimeError::new_err("Не удалось разобрать программу"))
    }
}

#[pyfunction]
fn compile_atlas(source: &str) -> PyResult<String> {
    // Лексер
    let mut lex = lexer::Lexer::new(source);
    let (tokens, lex_errors) = lex.tokenize();
    if !lex_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            lex_errors.iter().map(|e| format!("Лексер: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }

    // Парсер
    let mut parser = parser::Parser::new(tokens);
    let (cst, parse_errors) = parser.parse();
    if !parse_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            parse_errors.iter().map(|e| format!("Парсер: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }
    let cst = cst.ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Парсер не вернул CST"))?;

    // Десахаринг
    let mut desugarer = desugar::Desugarer::new();
    let (ast, desugar_errors) = desugarer.desugar(&cst);
    if !desugar_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            desugar_errors.iter().map(|e| format!("Десахаринг: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }
    let ast = ast.ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Десахаринг не удался"))?;

    // Разрешение имён
    let mut resolver = resolve::Resolver::new();
    let (resolved_ast, resolve_errors) = resolver.resolve(&ast);
    if !resolve_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            resolve_errors.iter().map(|e| format!("Разрешение: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }
    let resolved_ast = resolved_ast.ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Разрешение имён не удалось"))?;

    // Вывод типов
    let mut inferrer = infer::Inferrer::new();
    let (typed_ast, infer_errors) = inferrer.infer(&resolved_ast);
    if !infer_errors.is_empty() {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            infer_errors.iter().map(|e| format!("Типизация: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }
    let typed_ast = typed_ast.ok_or_else(|| pyo3::exceptions::PyRuntimeError::new_err("Вывод типов не удался"))?;

    // Проверка заимствований
    let mut checker = borrow::BorrowChecker::new();
    let (success, borrow_errors) = checker.check(&typed_ast);
    if !success {
        return Err(pyo3::exceptions::PySyntaxError::new_err(
            borrow_errors.iter().map(|e| format!("Заимствование: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }

    // Кодогенерация
    let mut codegen = codegen::Codegen::new("main");
    let (ir, codegen_errors) = codegen.generate(&typed_ast);
    if !codegen_errors.is_empty() {
        return Err(pyo3::exceptions::PyRuntimeError::new_err(
            codegen_errors.iter().map(|e| format!("Кодогенерация: {}", e.message)).collect::<Vec<_>>().join("\n")
        ));
    }

    if let Some(ir) = ir {
        let llvm_text = codegen::Codegen::emit_llvm_text(&ir);
        Ok(format!("Компиляция успешна!\n\nСгенерированный LLVM IR:\n\n{}", llvm_text))
    } else {
        Err(pyo3::exceptions::PyRuntimeError::new_err("Кодогенерация не вернула IR"))
    }
}

#[pymodule]
fn grammalang_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze_will, m)?)?;
    m.add_function(wrap_pyfunction!(tokenize_atlas, m)?)?;
    m.add_function(wrap_pyfunction!(parse_atlas, m)?)?;
    m.add_function(wrap_pyfunction!(desugar_atlas, m)?)?;
    m.add_function(wrap_pyfunction!(compile_atlas, m)?)?;
    Ok(())
}
