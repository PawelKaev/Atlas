// grammalang-core/src/desugar.rs

use crate::ast::*;
use crate::error::{Diagnostic, DiagnosticKind};
use crate::parser::CstNode;
use crate::token::Span;

pub struct Desugarer {
    errors: Vec<Diagnostic>,
}

impl Desugarer {
    pub fn new() -> Self {
        Desugarer { errors: Vec::new() }
    }

    pub fn desugar(&mut self, cst: &CstNode) -> (Option<Ast>, Vec<Diagnostic>) {
        let ast = self.desugar_node(cst);
        (ast, std::mem::take(&mut self.errors))
    }

    fn desugar_node(&mut self, node: &CstNode) -> Option<Ast> {
        match node {
            CstNode::Module { declarations, .. } => {
                let decls: Vec<Ast> = declarations.iter().filter_map(|d| self.desugar_node(d)).collect();
                Some(Ast::Module {
                    name: "".to_string(),
                    declarations: decls,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Fn { name, params, return_type, body, modifiers, .. } => {
                let params: Vec<Parameter> = params
                    .iter()
                    .map(|p| {
                        let typ = self.desugar_type(&p.llvm_type);
                        Parameter {
                            name: p.name.clone(),
                            llvm_type: typ.unwrap_or(Type::Void),
                            mutable: p.mutable,
                        }
                    })
                    .collect();
                let ret = return_type.as_ref().and_then(|t| self.desugar_type(t));
                let body = self.desugar_node(body)?;
                Some(Ast::FnDecl {
                    name: name.clone(),
                    type_params: Vec::new(),
                    params,
                    return_type: ret,
                    body: Box::new(body),
                    public: modifiers.contains(&"public".to_string()),
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Block { expressions, .. } => {
                let exprs: Vec<Ast> = expressions.iter().filter_map(|e| self.desugar_node(e)).collect();
                Some(Ast::Block {
                    expressions: exprs,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::BinExpr { left, operator, right } => {
                let left = self.desugar_node(left)?;
                let right = self.desugar_node(right)?;
                let op = match operator.as_str() {
                    "+" => BinOp::Add,
                    "-" => BinOp::Sub,
                    "*" => BinOp::Mul,
                    "/" => BinOp::Div,
                    "%" => BinOp::Rem,
                    "==" | "EqEq" => BinOp::Eq,
                    "!=" | "NotEq" => BinOp::Neq,
                    "<" | "Lt" => BinOp::Lt,
                    ">" | "Gt" => BinOp::Gt,
                    "<=" | "Le" => BinOp::Le,
                    ">=" | "Ge" => BinOp::Ge,
                    "and" => BinOp::And,
                    "or" => BinOp::Or,
                    "++" => BinOp::Concat,
                    _ => {
                        self.error(&format!("Unknown operator: {}", operator));
                        return None;
                    }
                };
                Some(Ast::BinExpr {
                    left: Box::new(left),
                    operator: op,
                    right: Box::new(right),
                    llvm_type: None,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Pipeline { left, right } => {
                let left_ast = self.desugar_node(left)?;
                match right.as_ref() {
                    CstNode::Call { function, arguments } => {
                        let func = self.desugar_node(function)?;
                        let mut args = vec![left_ast];
                        for arg in arguments {
                            if let Some(a) = self.desugar_node(arg) {
                                args.push(a);
                            }
                        }
                        Some(Ast::Call {
                            function: Box::new(func),
                            arguments: args,
                            llvm_type: None,
                            span: Span { line: 1, column: 1, offset: 0 },
                        })
                    }
                    _ => {
                        let right_ast = self.desugar_node(right)?;
                        Some(Ast::Call {
                            function: Box::new(right_ast),
                            arguments: vec![left_ast],
                            llvm_type: None,
                            span: Span { line: 1, column: 1, offset: 0 },
                        })
                    }
                }
            }
            CstNode::AporeticBinding { left, right } => {
                let l = self.desugar_node(left)?;
                let r = self.desugar_node(right)?;
                Some(Ast::AporeticBinding {
                    left: Box::new(l),
                    right: Box::new(r),
                    source_span: Span { line: 1, column: 1, offset: 0 },
                })
            }
            CstNode::AufhebenBinding { left, right } => {
                let l = self.desugar_node(left)?;
                let r = self.desugar_node(right)?;
                Some(Ast::AufhebenBinding {
                    left: Box::new(l),
                    right: Box::new(r),
                    source_span: Span { line: 1, column: 1, offset: 0 },
                })
            }
            CstNode::ExecuteBinding { schema, args } => {
                let s = self.desugar_node(schema)?;
                let a: Vec<Ast> = args.iter().filter_map(|arg| self.desugar_node(arg)).collect();
                Some(Ast::ExecuteBinding {
                    schema: Box::new(s),
                    arguments: a,
                    source_span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::ReflexiveCascade { subject, ethics_override, context } => {
                let subj = self.desugar_node(subject)?;
                let ctx = self.desugar_node(context)?;
                let override_ethics = ethics_override.as_ref().and_then(|name| {
                    match name.as_str() {
                        "First" | "FirstEthics" => Some(EthicalSystem::First),
                        "Second" | "SecondEthics" => Some(EthicalSystem::Second),
                        _ => None,
                    }
                });
                let default_ethics = EthicalSystem::Second;
                Some(Ast::ReflexiveCascade {
                    subject: Box::new(subj),
                    ethics_override: override_ethics,
                    context: Box::new(ctx),
                    ethics: override_ethics.unwrap_or(default_ethics),
                    depth: 3,
                    source_span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Match { value, arms, .. } => {
                let val = self.desugar_node(value)?;
                let branches: Vec<MatchArm> = arms
                    .iter()
                    .filter_map(|b| {
                        let pattern = self.desugar_pattern(&b.pattern)?;
                        let guard = b.condition.as_ref().and_then(|g| self.desugar_node(g)).map(Box::new);
                        let body = self.desugar_node(&b.body)?;
                        Some(MatchArm {
                            pattern,
                            condition: guard,
                            body: Box::new(body),
                        })
                    })
                    .collect();
                Some(Ast::Match {
                    value: Box::new(val),
                    arms: branches,
                    llvm_type: None,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::If { condition, then, else_arm } => {
                let cond = self.desugar_node(condition)?;
                let then_branch = self.desugar_node(then)?;
                let else_branch = else_arm.as_ref().and_then(|e| self.desugar_node(e)).map(Box::new);
                Some(Ast::If {
                    condition: Box::new(cond),
                    then: Box::new(then_branch),
                    else_arm: else_branch,
                    llvm_type: None,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Return(value) => {
                let val = value.as_ref().and_then(|v| self.desugar_node(v)).map(Box::new);
                Some(Ast::Return {
                    value: val,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Assign { name, mutable, value } => {
                let val = self.desugar_node(value)?;
                Some(Ast::Assign {
                    name: name.clone(),
                    type_annotation: None,
                    mutable: *mutable,
                    value: Box::new(val),
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Borrow { mutable, value } => {
                let val = self.desugar_node(value)?;
                Some(Ast::Borrow {
                    mutable: *mutable,
                    value: Box::new(val),
                    llvm_type: None,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::EffectBlock { effects, body } => {
                let body = self.desugar_node(body)?;
                let mut result = body;
                for effect in effects.iter().rev() {
                    result = Ast::EffectBlock {
                        effects: vec![effect.clone()],
                        body: Box::new(result),
                        span: Span { line: 1, column: 1, offset: 0 },
                    };
                }
                Some(result)
            }

            CstNode::ParallelBlock { body } => {
                let body = self.desugar_node(body)?;
                Some(Ast::ParallelBlock {
                    strategy: ParallelStrategy::FailFast,
                    body: Box::new(body),
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Call { function, arguments } => {
                let func = self.desugar_node(function)?;
                let args: Vec<Ast> = arguments.iter().filter_map(|a| self.desugar_node(a)).collect();
                Some(Ast::Call {
                    function: Box::new(func),
                    arguments: args,
                    llvm_type: None,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::FieldAccess { object, field } => {
                let obj = self.desugar_node(object)?;
                Some(Ast::FieldAccess {
                    object: Box::new(obj),
                    field: field.clone(),
                    llvm_type: None,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::StructCons { name, fields } => {
                let fields: Vec<(String, Ast)> = fields
                    .iter()
                    .filter_map(|(n, v)| self.desugar_node(v).map(|ast| (n.clone(), ast)))
                    .collect();
                Some(Ast::StructCons {
                    name: name.clone(),
                    fields,
                    llvm_type: None,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::SumCons { name, value } => {
                let val = value.as_ref().and_then(|v| self.desugar_node(v)).map(Box::new);
                Some(Ast::SumCons {
                    name: name.clone(),
                    value: val,
                    llvm_type: None,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::UnaryExpr { operator, operand } => {
                let operand = self.desugar_node(operand)?;
                let op = match operator.as_str() {
                    "-" => UnaryOp::Negate,
                    "not" => UnaryOp::Not,
                    "?" => UnaryOp::Question,
                    _ => {
                        self.error(&format!("Unknown unary operator: {}", operator));
                        return None;
                    }
                };
                Some(Ast::UnaryExpr {
                    operator: op,
                    operand: Box::new(operand),
                    llvm_type: None,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Variable(name) => Some(Ast::Variable {
                name: name.clone(),
                llvm_type: None,
                span: Span { line: 1, column: 1, offset: 0 },
            }),

            CstNode::Literal(lit) => {
                let value = match lit {
                    crate::token::TokenKind::Int(n) => Value::Int(*n),
                    crate::token::TokenKind::Float(f) => Value::Float(*f),
                    crate::token::TokenKind::String(s) => Value::String(s.clone()),
                    crate::token::TokenKind::True => Value::Bool(true),
                    crate::token::TokenKind::False => Value::Bool(false),
                    crate::token::TokenKind::Nil => Value::Nil,
                    _ => {
                        self.error("Unknown literal");
                        return None;
                    }
                };
                Some(Ast::Literal {
                    value,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::StructDecl { name, fields } => {
                let fields: Vec<(String, Type)> = fields
                    .iter()
                    .filter_map(|(n, t)| self.desugar_type(t).map(|typ| (n.clone(), typ)))
                    .collect();
                Some(Ast::StructDecl {
                    name: name.clone(),
                    type_params: Vec::new(),
                    fields,
                    public: false,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::SumDecl { name, variants } => {
                let variants: Vec<SumVariant> = variants
                    .iter()
                    .map(|(n, t)| SumVariant {
                        name: n.clone(),
                        data_type: t.as_ref().and_then(|typ| self.desugar_type(typ)),
                    })
                    .collect();
                Some(Ast::SumDecl {
                    name: name.clone(),
                    type_params: Vec::new(),
                    variants,
                    public: false,
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::ImportDecl { path, names } => {
                Some(Ast::ImportDecl {
                    path: path.clone(),
                    names: names.iter().map(|n| (n.clone(), None)).collect(),
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            _ => {
                self.error(&format!("Desugaring not implemented for: {:?}", node));
                None
            }
        }
    }

    fn desugar_type(&self, node: &CstNode) -> Option<Type> {
        match node {
            CstNode::TypeName(name) => match name.as_str() {
                "Int" => Some(Type::Primitive(PrimitiveType::Int)),
                "Float" => Some(Type::Primitive(PrimitiveType::Float)),
                "Bool" => Some(Type::Primitive(PrimitiveType::Bool)),
                "String" => Some(Type::Primitive(PrimitiveType::String)),
                _ => Some(Type::Variable(name.clone())),
            },
            CstNode::TypeParameterized { name, params } => {
                let params: Vec<Type> = params.iter().filter_map(|p| self.desugar_type(p)).collect();
                Some(Type::Parameterized {
                    name: name.clone(),
                    params,
                })
            }
            CstNode::TypeFn { arguments, result } => {
                let args: Vec<Type> = arguments.iter().filter_map(|a| self.desugar_type(a)).collect();
                let ret = self.desugar_type(result)?;
                Some(Type::Fn {
                    arguments: args,
                    result: Box::new(ret),
                })
            }
            CstNode::TypeRecord { fields } => {
                let fields: Vec<(String, Type)> = fields
                    .iter()
                    .filter_map(|(n, t)| self.desugar_type(t).map(|typ| (n.clone(), typ)))
                    .collect();
                Some(Type::Record(fields))
            }
            CstNode::TypeRef { mutable, llvm_type } => {
                let typ = self.desugar_type(llvm_type)?;
                Some(Type::Ref {
                    mutable: *mutable,
                    llvm_type: Box::new(typ),
                })
            }
            _ => Some(Type::Void),
        }
    }

    fn desugar_pattern(&mut self, node: &CstNode) -> Option<Pattern> {
        match node {
            CstNode::PatternWildcard => Some(Pattern::Wildcard),
            CstNode::PatternVariable(name) => {
                if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                    Some(Pattern::Constructor {
                        name: name.clone(),
                        nested: None,
                    })
                } else {
                    Some(Pattern::Variable(name.clone()))
                }
            }
            CstNode::PatternLiteral(lit) => Some(Pattern::Literal(Value::String(lit.clone()))),
            CstNode::PatternConstructor { name, nested } => {
                let inner = nested.as_ref().and_then(|p| self.desugar_pattern(p)).map(Box::new);
                Some(Pattern::Constructor {
                    name: name.clone(),
                    nested: inner,
                })
            }
            CstNode::PatternOr(left, right) => {
                let l = self.desugar_pattern(left)?;
                let r = self.desugar_pattern(right)?;
                Some(Pattern::Or(Box::new(l), Box::new(r)))
            }
            CstNode::PatternBinding { name, pattern } => {
                let inner = self.desugar_pattern(pattern)?;
                Some(Pattern::Binding {
                    name: name.clone(),
                    pattern: Box::new(inner),
                })
            }
            CstNode::PatternStruct { name, fields, open } => {
                let fields: Vec<(String, Pattern)> = fields
                    .iter()
                    .filter_map(|(n, p)| self.desugar_pattern(p).map(|pat| (n.clone(), pat)))
                    .collect();
                Some(Pattern::Struct {
                    name: name.clone(),
                    fields,
                    open: *open,
                })
            }
            _ => {
                self.error(&format!("Unknown pattern: {:?}", node));
                None
            }
        }
    }

    fn error(&mut self, message: &str) {
        self.errors.push(Diagnostic {
            kind: DiagnosticKind::Error,
            message: message.to_string(),
            span: Span { line: 1, column: 1, offset: 0 },
            hint: None,
        });
    }
}
