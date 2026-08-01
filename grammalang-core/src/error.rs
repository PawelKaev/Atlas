// grammalang-core/src/error.rs

use serde::{Deserialize, Serialize};
use crate::token::Span;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DiagnosticKind {
    Ошибка,
    Предупреждение,
    Подсказка,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Diagnostic {
    pub kind: DiagnosticKind,
    pub message: String,
    pub span: Span,
    pub hint: Option<String>,
}

pub type Result<T> = std::result::Result<T, Diagnostic>;
