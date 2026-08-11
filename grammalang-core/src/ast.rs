// grammalang-core/src/ast.rs

use serde::{Deserialize, Serialize};
use crate::token::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum EthicalSystem {
    First,
    Second,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Ast {
    Module { name: String, declarations: Vec<Ast>, span: Span },
    FnDecl {
        name: String, type_params: Vec<TypeParam>, params: Vec<Parameter>,
        return_type: Option<Type>, body: Box<Ast>, public: bool, span: Span,
    },
    StructDecl {
        name: String, type_params: Vec<TypeParam>, fields: Vec<(String, Type)>,
        public: bool, span: Span,
    },
    SumDecl {
        name: String, type_params: Vec<TypeParam>, variants: Vec<SumVariant>,
        public: bool, span: Span,
    },
    ImportDecl { path: Vec<String>, names: Vec<(String, Option<String>)>, span: Span },
    ExternFnDecl { language: String, name: String, params: Vec<Type>, return_type: Option<Type>, span: Span },
    
    ForLoop {
        variable: String,
        iterator: Box<Ast>,
        body: Box<Ast>,
        span: Span,
    },
    
    LoopWhile {
        condition: Box<Ast>,
        body: Box<Ast>,
        label: Option<String>,
        span: Span,
    },
    
    Loop {
        body: Box<Ast>,
        label: Option<String>,
        span: Span,
    },
    
    Break {
        label: Option<String>,
        value: Option<Box<Ast>>,
        span: Span,
    },
    
    Continue {
        label: Option<String>,
        span: Span,
    },
    
    Let {
        name: String,
        type_annotation: Option<Type>,
        mutable: bool,
        value: Box<Ast>,
        span: Span,
    },
    
    ScopeBlock {
        expressions: Vec<Ast>,
        last: Option<Box<Ast>>,
        captures: Vec<Capture>,
        span: Span,
    },
    
    OpAssign {
        name: String,
        operator: BinOp,
        value: Box<Ast>,
        span: Span,
    },
    
    PatternAssign {
        pattern: Pattern,
        value: Box<Ast>,
        span: Span,
    },
    
    StructUpdate {
        object: Box<Ast>,
        fields: Vec<(String, Ast)>,
        llvm_type: Option<Type>,
        span: Span,
    },
    AporeticBinding {
        left: Box<Ast>,
        right: Box<Ast>,
        source_span: Span,
    },
    AufhebenBinding {
        left: Box<Ast>,
        right: Box<Ast>,
        source_span: Span,
    },
    ExecuteBinding {
        schema: Box<Ast>,
        arguments: Vec<Ast>,
        source_span: Span,
    },
    EncodeBinding {
        schema: Box<Ast>,
        form: Box<Ast>,
        source_span: Span,
    },
    DecodeBinding {
        symbol: Box<Ast>,
        source_span: Span,
    },

    ReflexiveCascade {
        subject: Box<Ast>,
        ethics_override: Option<EthicalSystem>,
        context: Box<Ast>,
        ethics: EthicalSystem,
        depth: usize,
        source_span: Span,
    },
    
    Block { expressions: Vec<Ast>, span: Span },
    Assign { name: String, type_annotation: Option<Type>, mutable: bool, value: Box<Ast>, span: Span },
    BinExpr { left: Box<Ast>, operator: BinOp, right: Box<Ast>, llvm_type: Option<Type>, span: Span },
    UnaryExpr { operator: UnaryOp, operand: Box<Ast>, llvm_type: Option<Type>, span: Span },
    Call { function: Box<Ast>, arguments: Vec<Ast>, llvm_type: Option<Type>, span: Span },
    Lambda { params: Vec<Parameter>, return_type: Option<Type>, body: Box<Ast>, span: Span },
    Match { value: Box<Ast>, arms: Vec<MatchArm>, llvm_type: Option<Type>, span: Span },
    If { condition: Box<Ast>, then: Box<Ast>, else_arm: Option<Box<Ast>>, llvm_type: Option<Type>, span: Span },
    While { condition: Box<Ast>, body: Box<Ast>, span: Span },
    Return { value: Option<Box<Ast>>, span: Span },
    StructCons { name: String, fields: Vec<(String, Ast)>, llvm_type: Option<Type>, span: Span },
    SumCons { name: String, value: Option<Box<Ast>>, llvm_type: Option<Type>, span: Span },
    FieldAccess { object: Box<Ast>, field: String, llvm_type: Option<Type>, span: Span },
    Borrow { mutable: bool, value: Box<Ast>, llvm_type: Option<Type>, span: Span },
    Move { value: Box<Ast>, span: Span },
    Quote { body: Box<Ast>, span: Span },
    Splice { value: Box<Ast>, span: Span },
    EffectBlock { effects: Vec<String>, body: Box<Ast>, span: Span },
    ParallelBlock { strategy: ParallelStrategy, body: Box<Ast>, span: Span },
    UnsafeBlock { body: Box<Ast>, span: Span },
    Variable { name: String, llvm_type: Option<Type>, span: Span },
    Literal { value: Value, span: Span },
    MacroCall { name: String, arguments: Vec<MacroArg>, span: Span },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Capture {
    pub name: String,
    pub by_ref: bool,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Range {
    pub start: Box<Ast>,
    pub end: Box<Ast>,
    pub inclusive: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TypeParam {
    pub name: String,
    pub constraints: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub llvm_type: Type,
    pub mutable: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SumVariant {
    pub name: String,
    pub data_type: Option<Type>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MatchArm {
    pub pattern: Pattern,
    pub condition: Option<Box<Ast>>,
    pub body: Box<Ast>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Pattern {
    Variable(String),
    Wildcard,
    Literal(Value),
    Constructor { name: String, nested: Option<Box<Pattern>> },
    Tuple(Vec<Pattern>),
    List { elements: Vec<Pattern>, tail: Option<Box<Pattern>> },
    Struct { name: String, fields: Vec<(String, Pattern)>, open: bool },
    Range { start: Value, end: Value },
    Or(Box<Pattern>, Box<Pattern>),
    Binding { name: String, pattern: Box<Pattern> },
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinOp {
    Add, Sub, Mul, Div, Rem,
    Eq, Neq, Lt, Gt, Le, Ge,
    And, Or, Concat,
    BitAnd, BitOr, BitXor,
    Shl, Shr,
    Assign,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum UnaryOp {
    Negate, Not, Question,
    Ref, Deref,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Value {
    Int(i64), Float(f64), String(String), Bool(bool), Nil,
    Char(char),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ParallelStrategy {
    FailFast,
    CollectAll,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MacroArg {
    Expression(Box<Ast>),
    Type(Type),
    Block(Box<Ast>),
    Identifier(String),
    Pattern(Pattern),
    Declaration(Box<Ast>),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PrimitiveType {
    Int, Float, Bool, String,
    Char, Byte, UnsignedInt,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Type {
    Variable(String),
    Primitive(PrimitiveType),
    Parameterized { name: String, params: Vec<Type> },
    Fn { arguments: Vec<Type>, result: Box<Type> },
    Record(Vec<(String, Type)>),
    Sum(Vec<(String, Option<Type>)>),
    Ref { mutable: bool, llvm_type: Box<Type> },
    Effect { effect: String, llvm_type: Box<Type> },
    Refined { base: Box<Type>, condition: Box<Ast> },
    Unit,
    Void,
    Array { llvm_type: Box<Type>, size: Option<usize> },
    Slice { llvm_type: Box<Type> },
    Range,
    Tuple(Vec<Type>),
    Pointer { mutable: bool, llvm_type: Box<Type> },
}

impl Ast {
    pub fn span(&self) -> &Span {
        match self {
            Ast::Module { span, .. } => span,
            Ast::FnDecl { span, .. } => span,
            Ast::StructDecl { span, .. } => span,
            Ast::SumDecl { span, .. } => span,
            Ast::ImportDecl { span, .. } => span,
            Ast::ExternFnDecl { span, .. } => span,
            Ast::ForLoop { span, .. } => span,
            Ast::LoopWhile { span, .. } => span,
            Ast::Loop { span, .. } => span,
            Ast::Break { span, .. } => span,
            Ast::Continue { span, .. } => span,
            Ast::Let { span, .. } => span,
            Ast::ScopeBlock { span, .. } => span,
            Ast::OpAssign { span, .. } => span,
            Ast::PatternAssign { span, .. } => span,
            Ast::StructUpdate { span, .. } => span,
            Ast::AporeticBinding { source_span, .. } => source_span,
            Ast::AufhebenBinding { source_span, .. } => source_span,
            Ast::ExecuteBinding { source_span, .. } => source_span,
            Ast::EncodeBinding { source_span, .. } => source_span,
            Ast::DecodeBinding { source_span, .. } => source_span,
            Ast::ReflexiveCascade { source_span, .. } => source_span,
            Ast::Block { span, .. } => span,
            Ast::Assign { span, .. } => span,
            Ast::BinExpr { span, .. } => span,
            Ast::UnaryExpr { span, .. } => span,
            Ast::Call { span, .. } => span,
            Ast::Lambda { span, .. } => span,
            Ast::Match { span, .. } => span,
            Ast::If { span, .. } => span,
            Ast::While { span, .. } => span,
            Ast::Return { span, .. } => span,
            Ast::StructCons { span, .. } => span,
            Ast::SumCons { span, .. } => span,
            Ast::FieldAccess { span, .. } => span,
            Ast::Borrow { span, .. } => span,
            Ast::Move { span, .. } => span,
            Ast::Quote { span, .. } => span,
            Ast::Splice { span, .. } => span,
            Ast::EffectBlock { span, .. } => span,
            Ast::ParallelBlock { span, .. } => span,
            Ast::UnsafeBlock { span, .. } => span,
            Ast::Variable { span, .. } => span,
            Ast::Literal { span, .. } => span,
            Ast::MacroCall { span, .. } => span,
        }
    }
    
    pub fn is_expression(&self) -> bool {
        matches!(self,
            Ast::AporeticBinding { .. } |
            Ast::AufhebenBinding { .. } |
            Ast::ExecuteBinding { .. } |
            Ast::EncodeBinding { .. } |
            Ast::DecodeBinding { .. } |
            Ast::Literal { .. } |
            Ast::Variable { .. } |
            Ast::BinExpr { .. } |
            Ast::UnaryExpr { .. } |
            Ast::Call { .. } |
            Ast::Lambda { .. } |
            Ast::Match { .. } |
            Ast::If { .. } |
            Ast::Block { .. } |
            Ast::ScopeBlock { .. } |
            Ast::Let { .. } |
            Ast::Loop { .. } |
            Ast::LoopWhile { .. } |
            Ast::ForLoop { .. } |
            Ast::StructCons { .. } |
            Ast::SumCons { .. } |
            Ast::FieldAccess { .. } |
            Ast::StructUpdate { .. } |
            Ast::ReflexiveCascade { .. }
        )
    }

    pub fn clone_operator(&self) -> Option<BinOp> {
        match self {
            Ast::OpAssign { operator, .. } => Some(operator.clone()),
            _ => None,
        }
    }
}
