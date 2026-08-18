// grammalang-core/src/lib.rs

use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod ontology;
pub mod modes;
pub mod social;
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

pub use ontology::*;
pub use modes::*;

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

#[pymodule]
fn grammalang_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(analyze_will, m)?)?;
    m.add_function(wrap_pyfunction!(tokenize_atlas, m)?)?;
    m.add_function(wrap_pyfunction!(parse_atlas, m)?)?;
    Ok(())
}
