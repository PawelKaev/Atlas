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
    pub публичный: bool,
}

#[derive(Debug, Clone)]
pub enum SymbolKind {
    Функция {
        параметры_типа: Vec<ПараметрТипа>,
        параметры: Vec<Параметр>,
        возвращаемый_тип: Option<Тип>,
    },
    Структура {
        поля: Vec<(String, Тип)>,
    },
    Сумма {
        варианты: Vec<(String, Option<Тип>)>,
    },
    Переменная {
        тип: Option<Тип>,
        изменяемая: bool,
    },
    Модуль {
        символы: HashMap<String, Symbol>,
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
            Ast::Модуль { имя, объявления, .. } => {
                self.current_module.push(имя.clone());
                let mut module_syms = HashMap::new();

                for decl in объявления {
                    match decl {
                        Ast::ОбъявлениеФункции {
                            имя, параметры_типа, параметры, возвращаемый_тип, открыто, ..
                        } => {
                            module_syms.insert(имя.clone(), Symbol {
                                kind: SymbolKind::Функция {
                                    параметры_типа: параметры_типа.clone(),
                                    параметры: параметры.clone(),
                                    возвращаемый_тип: возвращаемый_тип.clone(),
                                },
                                span: Span { line: 1, column: 1, offset: 0 },
                                публичный: *открыто,
                            });
                        }
                        Ast::ОбъявлениеСтруктуры { имя, поля, открыто, .. } => {
                            module_syms.insert(имя.clone(), Symbol {
                                kind: SymbolKind::Структура { поля: поля.clone() },
                                span: Span { line: 1, column: 1, offset: 0 },
                                публичный: *открыто,
                            });
                        }
                        Ast::ОбъявлениеСуммы { имя, варианты, открыто, .. } => {
                            module_syms.insert(имя.clone(), Symbol {
                                kind: SymbolKind::Сумма {
                                    варианты: варианты.iter().map(|v| (v.имя.clone(), v.тип_данных.clone())).collect()
                                },
                                span: Span { line: 1, column: 1, offset: 0 },
                                публичный: *открыто,
                            });
                        }
                        _ => {}
                    }
                }

                let module_name = self.current_module.join(".");
                self.symbols.module_symbols.insert(module_name, Symbol {
                    kind: SymbolKind::Модуль { символы: module_syms },
                    span: Span { line: 1, column: 1, offset: 0 },
                    публичный: true,
                });

                self.current_module.pop();
            }
            _ => {}
        }
    }

    fn resolve_node(&mut self, node: &Ast) -> Option<Ast> {
        match node {
            Ast::Модуль { имя, объявления, span } => {
                let resolved_decls: Vec<Ast> = объявления.iter().filter_map(|d| self.resolve_node(d)).collect();
                Some(Ast::Модуль { имя: имя.clone(), объявления: resolved_decls, span: *span })
            }

            Ast::ОбъявлениеФункции { имя, параметры_типа, параметры, возвращаемый_тип, тело, открыто, span } => {
                self.push_scope();
                for param in параметры {
                    self.add_symbol(&param.имя, Symbol {
                        kind: SymbolKind::Переменная { тип: Some(param.тип.clone()), изменяемая: param.изменяемый },
                        span: *span, публичный: false,
                    });
                }
                let resolved_body = self.resolve_node(тело)?;
                self.pop_scope();
                Some(Ast::ОбъявлениеФункции {
                    имя: имя.clone(), параметры_типа: параметры_типа.clone(),
                    параметры: параметры.clone(), возвращаемый_тип: возвращаемый_тип.clone(),
                    тело: Box::new(resolved_body), открыто: *открыто, span: *span,
                })
            }

            Ast::Блок { выражения, span } => {
                self.push_scope();
                let exprs: Vec<Ast> = выражения.iter().filter_map(|e| self.resolve_node(e)).collect();
                self.pop_scope();
                Some(Ast::Блок { выражения: exprs, span: *span })
            }

            Ast::Присваивание { имя, тип_аннотация, изменяемая, значение, span } => {
                let resolved_value = self.resolve_node(значение)?;
                self.add_symbol(имя, Symbol {
                    kind: SymbolKind::Переменная { тип: тип_аннотация.clone(), изменяемая: *изменяемая },
                    span: *span, публичный: false,
                });
                Some(Ast::Присваивание {
                    имя: имя.clone(), тип_аннотация: тип_аннотация.clone(),
                    изменяемая: *изменяемая, значение: Box::new(resolved_value), span: *span,
                })
            }

            Ast::Переменная { имя, тип, span } => {
                if self.lookup_symbol(имя).is_none() {
                    self.error(&format!("Неизвестное имя: '{}'", имя), *span);
                }
                Some(Ast::Переменная { имя: имя.clone(), тип: тип.clone(), span: *span })
            }

            Ast::Сопоставление { значение, ветки, тип, span } => {
                let val = self.resolve_node(значение)?;
                let resolved_branches: Vec<ВеткаСопоставления> = ветки.iter().filter_map(|b| {
                    self.push_scope();
                    self.add_pattern_vars(&b.образец);
                    let body = self.resolve_node(&b.тело);
                    self.pop_scope();
                    body.map(|resolved_body| ВеткаСопоставления {
                        образец: b.образец.clone(),
                        условие: b.условие.clone(),
                        тело: Box::new(resolved_body),
                    })
                }).collect();
                Some(Ast::Сопоставление { значение: Box::new(val), ветки: resolved_branches, тип: тип.clone(), span: *span })
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
        None
    }

    fn add_pattern_vars(&mut self, pattern: &Образец) {
        match pattern {
            Образец::Переменная(name) => {
                self.add_symbol(name, Symbol {
                    kind: SymbolKind::Переменная { тип: None, изменяемая: false },
                    span: Span { line: 1, column: 1, offset: 0 },
                    публичный: false,
                });
            }
            Образец::Конструктор { вложенный, .. } => {
                if let Some(inner) = вложенный {
                    self.add_pattern_vars(inner);
                }
            }
            Образец::Кортеж(элементы) | Образец::Список { элементы: элементы, .. } => {
                for elem in элементы {
                    self.add_pattern_vars(elem);
                }
            }
            _ => {}
        }
    }

    fn error(&mut self, message: &str, span: Span) {
        self.errors.push(Diagnostic {
            kind: DiagnosticKind::Ошибка,
            message: message.to_string(),
            span,
            hint: None,
        });
    }
}
