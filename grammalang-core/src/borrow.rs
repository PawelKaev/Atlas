// grammalang-core/src/borrow.rs

use crate::token::Span;
use std::collections::{HashMap, HashSet};
use crate::ast::*;
use crate::error::{Diagnostic, DiagnosticKind};

/// Access right to a variable
#[derive(Debug, Clone, PartialEq)]
enum Access {
    Owns,
    BorrowedImmutable(HashSet<String>),
    BorrowedMutable(String),
    Moved,
}

/// Borrow checker state
pub struct BorrowChecker {
    access: HashMap<String, Access>,
    regions: Vec<(String, String)>,
    errors: Vec<Diagnostic>,
    next_region_id: usize,
}

impl BorrowChecker {
    pub fn new() -> Self {
        BorrowChecker {
            access: HashMap::new(),
            regions: Vec::new(),
            errors: Vec::new(),
            next_region_id: 0,
        }
    }

    pub fn check(&mut self, ast: &Ast) -> (bool, Vec<Diagnostic>) {
        self.check_node(ast);
        let success = self.errors.is_empty();

        if !self.check_region_cycles() {
            self.errors.push(Diagnostic {
                kind: DiagnosticKind::Error,
                message: "Cyclic lifetime graph detected".to_string(),
                span: Span { line: 0, column: 0, offset: 0 },
                hint: None,
            });
        }

        (success && self.errors.is_empty(), std::mem::take(&mut self.errors))
    }

    fn fresh_region(&mut self) -> String {
        let id = self.next_region_id;
        self.next_region_id += 1;
        format!("'r{}", id)
    }

    // ============ AST traversal ============

    fn check_node(&mut self, node: &Ast) {
        match node {
            Ast::Module { declarations, .. } => {
                for decl in declarations {
                    self.check_node(decl);
                }
            }

            Ast::FnDecl { params, body, .. } => {
                let saved = self.access.clone();

                for param in params {
                    self.access.insert(param.name.clone(), Access::Owns);
                }

                self.check_node(body);
                self.access = saved;
            }

            Ast::Block { expressions, .. } => {
                let saved = self.access.clone();

                for expr in expressions {
                    self.check_node(expr);
                }

                self.access = saved;
            }

            Ast::ScopeBlock { expressions, last, captures, .. } => {
                let saved = self.access.clone();

                for expr in expressions {
                    self.check_node(expr);
                }
                if let Some(last) = last {
                    self.check_node(last);
                }
                
                for capture in captures {
                    match self.access.get(&capture.name) {
                        Some(Access::Owns) => {
                            if capture.by_ref {
                                let region = self.fresh_region();
                                let mut refs = HashSet::new();
                                refs.insert(region.clone());
                                self.access.insert(
                                    capture.name.clone(),
                                    Access::BorrowedImmutable(refs),
                                );
                            } else {
                                self.access.insert(capture.name.clone(), Access::Moved);
                            }
                        }
                        Some(Access::BorrowedImmutable(refs)) if !capture.mutable => {
                            let mut new_refs = refs.clone();
                            new_refs.insert(self.fresh_region());
                            self.access.insert(capture.name.clone(), Access::BorrowedImmutable(new_refs));
                        }
                        _ => {}
                    }
                }

                self.access = saved;
            }

            Ast::Let { name, value, .. } => {
                self.check_node(value);
                self.access.insert(name.clone(), Access::Owns);
            }

            Ast::Assign { name, value, .. } => {
                self.check_node(value);
                self.access.insert(name.clone(), Access::Owns);
            }

            Ast::OpAssign { name, value, .. } => {
                self.check_node(value);
                if let Some(Access::Moved) = self.access.get(name) {
                    self.errors.push(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!("Cannot modify '{}' — variable has been moved", name),
                        span: *node.span(),
                        hint: None,
                    });
                }
            }

            Ast::PatternAssign { pattern, value, .. } => {
                self.check_node(value);
                self.add_pattern_vars(pattern);
            }

            Ast::StructUpdate { object, fields, .. } => {
                self.check_node(object);
                for (_, value) in fields {
                    self.check_node(value);
                }
            }

            Ast::Borrow { mutable, value, span, .. } => {
                if let Ast::Variable { name, .. } = value.as_ref() {
                    let region = self.fresh_region();

                    match self.access.get(name) {
                        Some(Access::Owns) => {
                            if *mutable {
                                self.access.insert(name.clone(), Access::BorrowedMutable(region.clone()));
                            } else {
                                let mut refs = HashSet::new();
                                refs.insert(region.clone());
                                self.access.insert(name.clone(), Access::BorrowedImmutable(refs));
                            }
                        }
                        Some(Access::BorrowedImmutable(refs)) if !mutable => {
                            let mut refs = refs.clone();
                            refs.insert(region.clone());
                            self.access.insert(name.clone(), Access::BorrowedImmutable(refs));
                        }
                        Some(Access::BorrowedImmutable(_)) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("Cannot mutably borrow '{}' — it is already immutably borrowed", name),
                                span: *span,
                                hint: None,
                            });
                        }
                        Some(Access::BorrowedMutable(existing)) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("Cannot borrow '{}' — it is already mutably borrowed (region {})", name, existing),
                                span: *span,
                                hint: None,
                            });
                        }
                        Some(Access::Moved) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("Cannot borrow '{}' — it has been moved", name),
                                span: *span,
                                hint: None,
                            });
                        }
                        None => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("Unknown variable: '{}'", name),
                                span: *span,
                                hint: None,
                            });
                        }
                    }
                }
            }

            Ast::Move { value, span } => {
                if let Ast::Variable { name, .. } = value.as_ref() {
                    match self.access.get(name) {
                        Some(Access::Owns) => {
                            self.access.insert(name.clone(), Access::Moved);
                        }
                        Some(Access::BorrowedImmutable(_)) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("Cannot move '{}' — it is borrowed", name),
                                span: *span,
                                hint: None,
                            });
                        }
                        Some(Access::BorrowedMutable(_)) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("Cannot move '{}' — it is mutably borrowed", name),
                                span: *span,
                                hint: None,
                            });
                        }
                        Some(Access::Moved) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("'{}' has already been moved", name),
                                span: *span,
                                hint: None,
                            });
                        }
                        None => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Error,
                                message: format!("Unknown variable: '{}'", name),
                                span: *span,
                                hint: None,
                            });
                        }
                    }
                }
            }

            Ast::Call { function, arguments, .. } => {
                self.check_node(function);
                for arg in arguments {
                    self.check_node(arg);
                }
            }

            Ast::BinExpr { left, right, .. } => {
                self.check_node(left);
                self.check_node(right);
            }

            Ast::UnaryExpr { operand, .. } => {
                self.check_node(operand);
            }

            Ast::Match { value, arms, .. } => {
                self.check_node(value);
                for arm in arms {
                    let saved = self.access.clone();
                    self.add_pattern_vars(&arm.pattern);

                    if let Some(ref condition) = arm.condition {
                        self.check_node(condition);
                    }
                    self.check_node(&arm.body);

                    self.access = saved;
                }
            }

            Ast::If { condition, then, else_arm, .. } => {
                self.check_node(condition);
                self.check_node(then);
                if let Some(else_branch) = else_arm {
                    self.check_node(else_branch);
                }
            }

            Ast::While { condition, body, .. } => {
                self.check_node(condition);
                self.check_node(body);
            }

            Ast::LoopWhile { condition, body, .. } => {
                self.check_node(condition);
                self.check_node(body);
            }

            Ast::ForLoop { variable, iterator, body, .. } => {
                self.check_node(iterator);
                let saved = self.access.clone();
                self.access.insert(variable.clone(), Access::Owns);
                self.check_node(body);
                self.access = saved;
            }

            Ast::Loop { body, .. } => {
                self.check_node(body);
            }

            Ast::Break { value, .. } => {
                if let Some(val) = value {
                    self.check_node(val);
                }
            }

            Ast::Continue { .. } => {}

            Ast::Return { value, .. } => {
                if let Some(val) = value {
                    self.check_node(val);
                }
            }

            Ast::EffectBlock { body, .. } => {
                self.check_node(body);
            }

            Ast::ParallelBlock { body, .. } => {
                self.check_node(body);
            }

            Ast::UnsafeBlock { body, .. } => {
                self.check_node(body);
            }

            Ast::FieldAccess { object, .. } => {
                self.check_node(object);
            }

            Ast::StructCons { fields, .. } => {
                for (_, value) in fields {
                    self.check_node(value);
                }
            }

            Ast::SumCons { value, .. } => {
                if let Some(val) = value {
                    self.check_node(val);
                }
            }

            Ast::Lambda { params, body, .. } => {
                let saved = self.access.clone();
                for param in params {
                    self.access.insert(param.name.clone(), Access::Owns);
                }
                self.check_node(body);
                self.access = saved;
            }

            Ast::Variable { name, span, .. } => {
                if let Some(Access::Moved) = self.access.get(name) {
                    self.errors.push(Diagnostic {
                        kind: DiagnosticKind::Error,
                        message: format!("Variable '{}' was moved and is no longer available", name),
                        span: *span,
                        hint: None,
                    });
                }
            }

            Ast::Literal { .. } => {}
            Ast::Quote { .. } => {}
            Ast::Splice { .. } => {}
            Ast::MacroCall { .. } => {}
            Ast::ImportDecl { .. } => {}
            Ast::ExternFnDecl { .. } => {}
            Ast::StructDecl { .. } => {}
            Ast::AporeticBinding { left, right, .. } => {
                self.check_node(left);
                self.check_node(right);
            }
            Ast::AufhebenBinding { left, right, .. } => {
                self.check_node(left);
                self.check_node(right);
            }
            Ast::ExecuteBinding { schema, arguments, .. } => {
                self.check_node(schema);
                for arg in arguments {
                    self.check_node(arg);
                }
            }
            Ast::EncodeBinding { schema, form, .. } => {
                self.check_node(schema);
                self.check_node(form);
            }
            Ast::DecodeBinding { symbol, .. } => {
                self.check_node(symbol);
            }
            Ast::ReflexiveCascade { subject, context, .. } => {
                self.check_node(subject);
                self.check_node(context);
            }
            Ast::SumDecl { .. } => {}
        }
    }

    fn add_pattern_vars(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Variable(name) => {
                self.access.insert(name.clone(), Access::Owns);
            }
            Pattern::Constructor { nested, .. } => {
                if let Some(inner) = nested {
                    self.add_pattern_vars(inner);
                }
            }
            Pattern::Tuple(elements) | Pattern::List { elements, .. } => {
                for elem in elements {
                    self.add_pattern_vars(elem);
                }
            }
            Pattern::Struct { fields, .. } => {
                for (_, pattern) in fields {
                    self.add_pattern_vars(pattern);
                }
            }
            _ => {}
        }
    }

    // ============ Region graph cycle check ============

    fn check_region_cycles(&self) -> bool {
        let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
        for (from, to) in &self.regions {
            graph.entry(from).or_default().push(to);
        }

        let mut visited = HashSet::new();
        let mut in_stack = HashSet::new();

        for node in graph.keys() {
            if !visited.contains(node) {
                if self.dfs_cycle(&graph, node, &mut visited, &mut in_stack) {
                    return false;
                }
            }
        }
        true
    }

    fn dfs_cycle<'a>(
        &self,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        node: &'a str,
        visited: &mut HashSet<&'a str>,
        in_stack: &mut HashSet<&'a str>,
    ) -> bool {
        visited.insert(node);
        in_stack.insert(node);

        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                if !visited.contains(neighbor) {
                    if self.dfs_cycle(graph, neighbor, visited, in_stack) {
                        return true;
                    }
                } else if in_stack.contains(neighbor) {
                    return true;
                }
            }
        }

        in_stack.remove(node);
        false
    }
}
