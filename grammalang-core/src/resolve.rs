// grammalang-core/src/resolve.rs

use std::collections::HashMap;
use crate::ast::*;
use crate::error::{Diagnostic, DiagnosticKind};
use crate::token::Span;

#[derive(Debug, Clone)]
pub struct SymbolTable {
    scopes: Vec<Scope>,
    module_symbols: HashMap<String, Symbol>,
}

#[derive(Debug, Clone)]
struct Scope {
    symbols: HashMap<String, Symbol>,
}

#[derive(Debug, Clone)]
pub struct Symbol {
    pub kind: SymbolKind,
    pub span: Span,
    pub public: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Fn {
        type_params: Vec<TypeParam>,
        params: Vec<Parameter>,
        return_type: Option<Type>,
    },
    Struct {
        fields: Vec<(String, Type)>,
    },
    Sum {
        variants: Vec<(String, Option<Type>)>,
    },
    Variable {
        llvm_type: Option<Type>,
        mutable: bool,
    },
    Module {
        symbols: HashMap<String, Symbol>,
    },
}

pub struct Resolver {
    symbols: SymbolTable,
    errors: Vec<Diagnostic>,
    current_module: Vec<String>,
}

impl Resolver {
    pub fn new() -> Self {
        Resolver {
            symbols: SymbolTable {
                scopes: vec![Scope { symbols: HashMap::new() }],
                module_symbols: HashMap::new(),
            },
            errors: Vec::new(),
            current_module: Vec::new(),
        }
    }

    pub fn resolve(&mut self, ast: &Ast) -> (Option<Ast>, Vec<Diagnostic>) {
        self.collect_declarations(ast);
        let resolved = self.resolve_node(ast);
        (resolved, std::mem::take(&mut self.errors))
    }

    fn collect_declarations(&mut self, ast: &Ast) {
        match ast {
            Ast::Module { name, declarations, .. } => {
                self.current_module.push(name.clone());
                let mut module_syms = HashMap::new();

                for decl in declarations {
                    match decl {
                        Ast::FnDecl {
                            name, type_params, params, return_type, public, ..
                        } => {
                            module_syms.insert(name.clone(), Symbol {
                                kind: SymbolKind::Fn {
                                    type_params: type_params.clone(),
                                    params: params.clone(),
                                    return_type: return_type.clone(),
                                },
                                span: Span { line: 1, column: 1, offset: 0 },
                                public: *public,
                            });
                        }
                        Ast::StructDecl { name, fields, public, .. } => {
                            module_syms.insert(name.clone(), Symbol {
                                kind: SymbolKind::Struct { fields: fields.clone() },
                                span: Span { line: 1, column: 1, offset: 0 },
                                public: *public,
                            });
                        }
                        Ast::SumDecl { name, variants, public, .. } => {
                            module_syms.insert(name.clone(), Symbol {
                                kind: SymbolKind::Sum {
                                    variants: variants.iter().map(|v| (v.name.clone(), v.data_type.clone())).collect()
                                },
                                span: Span { line: 1, column: 1, offset: 0 },
                                public: *public,
                            });
                            for variant in variants {
                                module_syms.insert(variant.name.clone(), Symbol {
                                    kind: SymbolKind::Sum {
                                        variants: vec![(variant.name.clone(), variant.data_type.clone())]
                                    },
                                    span: Span { line: 1, column: 1, offset: 0 },
                                    public: *public,
                                });
                            }
                        }
                        _ => {}
                    }
                }

                let module_name = self.current_module.join(".");
                self.symbols.module_symbols.insert(module_name, Symbol {
                    kind: SymbolKind::Module { symbols: module_syms },
                    span: Span { line: 1, column: 1, offset: 0 },
                    public: true,
                });

                self.current_module.pop();
            }
            _ => {}
        }
    }

    fn resolve_node(&mut self, node: &Ast) -> Option<Ast> {
        match node {
            Ast::Module { name, declarations, span } => {
                let resolved_decls: Vec<Ast> = declarations.iter().filter_map(|d| self.resolve_node(d)).collect();
                Some(Ast::Module { name: name.clone(), declarations: resolved_decls, span: *span })
            }

            Ast::FnDecl { name, type_params, params, return_type, body, public, span } => {
                self.push_scope();
                for param in params {
                    self.add_symbol(&param.name, Symbol {
                        kind: SymbolKind::Variable {
                            llvm_type: Some(param.llvm_type.clone()),
                            mutable: param.mutable,
                        },
                        span: *span,
                        public: false,
                    });
                }
                let resolved_body = self.resolve_node(body)?;
                self.pop_scope();
                Some(Ast::FnDecl {
                    name: name.clone(),
                    type_params: type_params.clone(),
                    params: params.clone(),
                    return_type: return_type.clone(),
                    body: Box::new(resolved_body),
                    public: *public,
                    span: *span,
                })
            }

            Ast::Block { expressions, span } => {
                self.push_scope();
                let exprs: Vec<Ast> = expressions.iter().filter_map(|e| self.resolve_node(e)).collect();
                self.pop_scope();
                Some(Ast::Block { expressions: exprs, span: *span })
            }

            Ast::Assign { name, type_annotation, mutable, value, span } => {
                let resolved_value = self.resolve_node(value)?;
                self.add_symbol(name, Symbol {
                    kind: SymbolKind::Variable {
                        llvm_type: type_annotation.clone(),
                        mutable: *mutable,
                    },
                    span: *span,
                    public: false,
                });
                Some(Ast::Assign {
                    name: name.clone(),
                    type_annotation: type_annotation.clone(),
                    mutable: *mutable,
                    value: Box::new(resolved_value),
                    span: *span,
                })
            }

            Ast::Variable { name, llvm_type, span } => {
                if self.lookup_symbol(name).is_none() {
                    self.error(&format!("Unknown name: '{}'", name), *span);
                }
                Some(Ast::Variable { name: name.clone(), llvm_type: llvm_type.clone(), span: *span })
            }

            Ast::Match { value, arms, llvm_type, span } => {
                let val = self.resolve_node(value)?;
                let resolved_branches: Vec<MatchArm> = arms.iter().filter_map(|b| {
                    self.push_scope();
                    self.add_pattern_vars(&b.pattern);
                    let body = self.resolve_node(&b.body);
                    self.pop_scope();
                    body.map(|resolved_body| MatchArm {
                        pattern: b.pattern.clone(),
                        condition: b.condition.clone(),
                        body: Box::new(resolved_body),
                    })
                }).collect();
                Some(Ast::Match {
                    value: Box::new(val),
                    arms: resolved_branches,
                    llvm_type: llvm_type.clone(),
                    span: *span,
                })
            }

            _ => Some(node.clone()),
        }
    }

    fn push_scope(&mut self) {
        self.symbols.scopes.push(Scope { symbols: HashMap::new() });
    }

    fn pop_scope(&mut self) {
        self.symbols.scopes.pop();
    }

    fn add_symbol(&mut self, name: &str, symbol: Symbol) {
        if let Some(scope) = self.symbols.scopes.last_mut() {
            scope.symbols.insert(name.to_string(), symbol);
        }
    }

    fn lookup_symbol(&self, name: &str) -> Option<&Symbol> {
        for scope in self.symbols.scopes.iter().rev() {
            if let Some(sym) = scope.symbols.get(name) {
                return Some(sym);
            }
        }
        self.symbols.module_symbols.get(name)
    }

    fn add_pattern_vars(&mut self, pattern: &Pattern) {
        match pattern {
            Pattern::Variable(name) => {
                self.add_symbol(name, Symbol {
                    kind: SymbolKind::Variable { llvm_type: None, mutable: false },
                    span: Span { line: 1, column: 1, offset: 0 },
                    public: false,
                });
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
            _ => {}
        }
    }

    fn error(&mut self, message: &str, span: Span) {
        self.errors.push(Diagnostic {
            kind: DiagnosticKind::Error,
            message: message.to_string(),
            span,
            hint: None,
        });
    }
}
