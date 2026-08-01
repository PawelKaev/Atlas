// grammalang-core/src/borrow.rs

use std::collections::{HashMap, HashSet};
use crate::ast::*;
use crate::error::{Diagnostic, DiagnosticKind};
use crate::token::Span;

/// Право доступа к переменной
#[derive(Debug, Clone, PartialEq)]
enum Access {
    Владеет,
    ЗаимствованаНеизменяемо(HashSet<String>),
    ЗаимствованаИзменяемо(String),
    Перемещена,
}

/// Состояние borrow checker'а
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
                kind: DiagnosticKind::Ошибка,
                message: "Обнаружен циклический граф времён жизни".to_string(),
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

    // ============ Обход AST ============

    fn check_node(&mut self, node: &Ast) {
        match node {
            Ast::Модуль { объявления, .. } => {
                for decl in объявления {
                    self.check_node(decl);
                }
            }

            Ast::ОбъявлениеФункции { параметры, тело, .. } => {
                let saved = self.access.clone();

                for param in параметры {
                    self.access.insert(param.имя.clone(), Access::Владеет);
                }

                self.check_node(тело);
                self.access = saved;
            }

            Ast::Блок { выражения, .. } => {
                let saved = self.access.clone();

                for expr in выражения {
                    self.check_node(expr);
                }

                self.access = saved;
            }

            Ast::Присваивание { имя, изменяемая, значение, .. } => {
                self.check_node(значение);
                self.access.insert(имя.clone(), Access::Владеет);
            }

            Ast::Заимствование { изменяемое, значение, span, .. } => {
                if let Ast::Переменная { имя, .. } = значение.as_ref() {
                    let region = self.fresh_region();

                    match self.access.get(имя) {
                        Some(Access::Владеет) => {
                            if *изменяемое {
                                self.access.insert(имя.clone(), Access::ЗаимствованаИзменяемо(region.clone()));
                            } else {
                                let mut refs = HashSet::new();
                                refs.insert(region.clone());
                                self.access.insert(имя.clone(), Access::ЗаимствованаНеизменяемо(refs));
                            }
                        }
                        Some(Access::ЗаимствованаНеизменяемо(refs)) if !изменяемое => {
                            let mut refs = refs.clone();
                            refs.insert(region.clone());
                            self.access.insert(имя.clone(), Access::ЗаимствованаНеизменяемо(refs));
                        }
                        Some(Access::ЗаимствованаНеизменяемо(_)) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Ошибка,
                                message: format!("Нельзя создать изменяемую ссылку на '{}', потому что она уже заимствована неизменяемо", имя),
                                span: *span,
                                hint: None,
                            });
                        }
                        Some(Access::ЗаимствованаИзменяемо(existing)) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Ошибка,
                                message: format!("Нельзя заимствовать '{}', потому что она уже заимствована изменяемо (регион {})", имя, existing),
                                span: *span,
                                hint: None,
                            });
                        }
                        Some(Access::Перемещена) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Ошибка,
                                message: format!("Нельзя заимствовать '{}', потому что она была перемещена", имя),
                                span: *span,
                                hint: None,
                            });
                        }
                        None => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Ошибка,
                                message: format!("Неизвестная переменная: '{}'", имя),
                                span: *span,
                                hint: None,
                            });
                        }
                    }
                }
            }

            Ast::Перемещение { значение, span } => {
                if let Ast::Переменная { имя, .. } = значение.as_ref() {
                    match self.access.get(имя) {
                        Some(Access::Владеет) => {
                            self.access.insert(имя.clone(), Access::Перемещена);
                        }
                        Some(Access::ЗаимствованаНеизменяемо(_)) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Ошибка,
                                message: format!("Нельзя переместить '{}', потому что она заимствована", имя),
                                span: *span,
                                hint: None,
                            });
                        }
                        Some(Access::ЗаимствованаИзменяемо(_)) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Ошибка,
                                message: format!("Нельзя переместить '{}', потому что она заимствована изменяемо", имя),
                                span: *span,
                                hint: None,
                            });
                        }
                        Some(Access::Перемещена) => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Ошибка,
                                message: format!("'{}' уже была перемещена", имя),
                                span: *span,
                                hint: None,
                            });
                        }
                        None => {
                            self.errors.push(Diagnostic {
                                kind: DiagnosticKind::Ошибка,
                                message: format!("Неизвестная переменная: '{}'", имя),
                                span: *span,
                                hint: None,
                            });
                        }
                    }
                }
            }

            Ast::Вызов { функция, аргументы, .. } => {
                self.check_node(функция);
                for arg in аргументы {
                    self.check_node(arg);
                }
            }

            Ast::ДвоичноеВыражение { левое, правое, .. } => {
                self.check_node(левое);
                self.check_node(правое);
            }

            Ast::УнарноеВыражение { операнд, .. } => {
                self.check_node(операнд);
            }

            Ast::Сопоставление { значение, ветки, .. } => {
                self.check_node(значение);
                for ветка in ветки {
                    let saved = self.access.clone();
                    self.add_pattern_vars(&ветка.образец);

                    if let Some(ref условие) = ветка.условие {
                        self.check_node(условие);
                    }
                    self.check_node(&ветка.тело);

                    self.access = saved;
                }
            }

            Ast::Если { условие, то, иначе, .. } => {
                self.check_node(условие);
                self.check_node(то);
                if let Some(else_branch) = иначе {
                    self.check_node(else_branch);
                }
            }

            Ast::Возврат { значение, .. } => {
                if let Some(val) = значение {
                    self.check_node(val);
                }
            }

            Ast::БлокЭффекта { тело, .. } => {
                self.check_node(тело);
            }

            Ast::ПараллельныйБлок { тело, .. } => {
                self.check_node(тело);
            }

            Ast::РучнойБлок { тело, .. } => {
                self.check_node(тело);
            }

            Ast::ДоступКПолю { объект, .. } => {
                self.check_node(объект);
            }

            Ast::КонструкторСтруктуры { поля, .. } => {
                for (_, value) in поля {
                    self.check_node(value);
                }
            }

            Ast::КонструкторСуммы { значение, .. } => {
                if let Some(val) = значение {
                    self.check_node(val);
                }
            }

            Ast::Переменная { имя, span, .. } => {
                if let Some(Access::Перемещена) = self.access.get(имя) {
                    self.errors.push(Diagnostic {
                        kind: DiagnosticKind::Ошибка,
                        message: format!("Переменная '{}' была перемещена и больше не доступна", имя),
                        span: *span,
                        hint: None,
                    });
                }
            }

            _ => {}
        }
    }

    fn add_pattern_vars(&mut self, pattern: &Образец) {
        match pattern {
            Образец::Переменная(name) => {
                self.access.insert(name.clone(), Access::Владеет);
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

    // ============ Проверка графа регионов ============

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
