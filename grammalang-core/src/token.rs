// grammalang-core/src/token.rs

use serde::{Deserialize, Serialize};
use std::fmt;

/// Position in source file
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Span {
    pub line: usize,   // line, starting from 1
    pub column: usize, // column, starting from 1
    pub offset: usize, // byte offset from start of file
}

impl fmt::Display for Span {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line {}, column {}", self.line, self.column)
    }
}

/// Token kind
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TokenKind {
    // Keywords
    Fn,
    Return,
    If,
    Else,
    Match,
    Struct,
    Type,
    Mut,
    Effect,
    Together,
    Macro,
    Public,
    Import,
    Module,
    Unsafe,
    Quote,
    Splice,
    For,
    Each,
    From,
    While,
    Where,
    True,
    False,
    Nil,
    Value,
    Failure,
    Success,
    
    // New keywords
    Let,
    Loop,
    Break,
    Continue,
    As,       // as (type casting)
    With,     // with (struct update)
    
    // Identifiers and literals
    Identifier(String),
    Int(i64),
    Float(f64),
    String(String),
    Char(char),

    // Operators
    Plus,        // +
    Minus,       // -
    Star,        // *
    Slash,       // /
    Percent,     // %
    Eq,          // =
    EqEq,        // ==
    NotEq,       // !=
    Lt,          // <
    Gt,          // >
    Le,          // <=
    Ge,          // >=
    Arrow,       // ->
    Pipeline,    // |>
    Shr,         // >>
    Ampersand,   // &
    At,          // @
    Pipe,        // |
    Question,    // ?
    Underscore,  // _
    Dot,         // .
    Colon,       // :
    ColonColonColon,  // :::
    AufhebenOp,       // <<+>>
    ExecuteOp,        // <<execute>>
    EncodeOp,         // <<encode>>
    DecodeOp,         // <<decode>>
    PraxisOp,         // <<praxis>>
    RevolutionOp,      // <<revolution>>
    AporeticOp,       // ~:~
    Comma,       // ,
    Semicolon,   // ;
    DotDot,      // ..
    DotDotEq,    // ..=
    Ellipsis,    // ...
    
    // Compound operators
    PlusEq,      // +=
    MinusEq,     // -=
    StarEq,      // *=
    SlashEq,     // /=
    PercentEq,   // %=
    AmpersandEq, // &=
    PipeEq,      // |=
    Shl,         // <<
    ShlEq,       // <<=
    ShrEq,       // >>=
    Caret,       // ^
    CaretEq,     // ^=
    Bang,        // !

    // Brackets
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Special
    Indent,          // increase indentation
    Dedent,          // decrease indentation
    Eof,
    Comment(String),
    Documentation(String),
    Error(String),
}

/// Token — atomic unit of the language
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub kind: TokenKind,
    pub lexeme: String,  // original token text
    pub span: Span,
}

impl Token {
    pub fn new(kind: TokenKind, lexeme: impl Into<String>, span: Span) -> Self {
        Token {
            kind,
            lexeme: lexeme.into(),
            span,
        }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} ('{}')", self.kind, self.lexeme)
    }
}
