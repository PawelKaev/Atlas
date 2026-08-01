// grammalang-core/src/infer.rs

use std::collections::HashMap;
use crate::ast::*;
use crate::error::{Diagnostic, DiagnosticKind};
use crate::token::Span;
use crate::types::{Constraint, Substitution, fresh_var};

type TypeContext = HashMap<String, Тип>;

pub struct Inferrer {
    context: TypeContext,
    constraints: Vec<Constraint>,
    errors: Vec<Diagnostic>,
}

impl Inferrer {
    pub fn new() -> Self {
        Inferrer {
            context: HashMap::new(),
            constraints: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn infer(&mut self, ast: &Ast) -> (Option<Ast>, Vec<Diagnostic>) {
        let typed = self.infer_node(ast);
        (typed, std::mem::take(&mut self.errors))
    }

    fn infer_node(&mut self, node: &Ast) -> Option<Ast> {
        match node {
            Ast::Модуль { имя, объявления, span } => {
                let decls: Vec<Ast> = объявления.iter().filter_map(|d| self.infer_node(d)).collect();
                Some(Ast::Модуль { имя: имя.clone(), объявления: decls, span: *span })
            }

            Ast::ОбъявлениеФункции { имя, параметры_типа, параметры, возвращаемый_тип, тело, открыто, span } => {
                let mut saved = Vec::new();
                for param in параметры {
                    let var_typ = fresh_var();
                    saved.push((param.имя.clone(), self.context.insert(param.имя.clone(), var_typ)));
                }
                let typed_body = self.infer_node(тело)?;
                for (name, old) in saved {
                    if let Some(t) = old { self.context.insert(name, t); }
                    else { self.context.remove(&name); }
                }
                Some(Ast::ОбъявлениеФункции {
                    имя: имя.clone(), параметры_типа: параметры_типа.clone(),
                    параметры: параметры.clone(), возвращаемый_тип: возвращаемый_тип.clone(),
                    тело: Box::new(typed_body), открыто: *открыто, span: *span,
                })
            }

            Ast::Блок { выражения, span } => {
                let exprs: Vec<Ast> = выражения.iter().filter_map(|e| self.infer_node(e)).collect();
                Some(Ast::Блок { выражения: exprs, span: *span })
            }

            Ast::Присваивание { имя, тип_аннотация, изменяемая, значение, span } => {
                let typed_value = self.infer_node(значение)?;
                let value_type = self.get_type(&typed_value);
                if let (Some(ref annot), Some(ref vt)) = (тип_аннотация, &value_type) {
                    self.constraints.push(Constraint::Равенство(vt.clone(), annot.clone()));
                }
                let final_type = тип_аннотация.clone().or(value_type);
                self.context.insert(имя.clone(), final_type.clone().unwrap_or(fresh_var()));
                Some(Ast::Присваивание {
                    имя: имя.clone(), тип_аннотация: final_type,
                    изменяемая: *изменяемая, значение: Box::new(typed_value), span: *span,
                })
            }

            Ast::ДвоичноеВыражение { левое, оператор, правое, тип, span } => {
                let typed_left = self.infer_node(левое)?;
                let typed_right = self.infer_node(правое)?;
                let left_type = self.get_type(&typed_left).unwrap_or(fresh_var());
                let right_type = self.get_type(&typed_right).unwrap_or(fresh_var());
                let result_type = match оператор {
                    БинарныйОператор::Сложение | БинарныйОператор::Вычитание
                    | БинарныйОператор::Умножение | БинарныйОператор::Деление
                    | БинарныйОператор::Остаток => {
                        self.constraints.push(Constraint::Равенство(left_type.clone(), right_type.clone()));
                        left_type.clone()
                    }
                    БинарныйОператор::Равно | БинарныйОператор::НеРавно
                    | БинарныйОператор::Меньше | БинарныйОператор::Больше
                    | БинарныйОператор::МеньшеРавно | БинарныйОператор::БольшеРавно => {
                        self.constraints.push(Constraint::Равенство(left_type.clone(), right_type.clone()));
                        Тип::Примитивный(ПримитивныйТип::Булево)
                    }
                    БинарныйОператор::И | БинарныйОператор::Или => Тип::Примитивный(ПримитивныйТип::Булево),
                    БинарныйОператор::Конкатенация => Тип::Примитивный(ПримитивныйТип::Строка),
                };
                Some(Ast::ДвоичноеВыражение {
                    левое: Box::new(typed_left), оператор: оператор.clone(),
                    правое: Box::new(typed_right), тип: Some(result_type), span: *span,
                })
            }

            Ast::Литерал { значение, span } => {
                let typ = match значение {
                    Значение::Целое(_) => Тип::Примитивный(ПримитивныйТип::Целое),
                    Значение::Десятичное(_) => Тип::Примитивный(ПримитивныйТип::Десятичное),
                    Значение::Строка(_) => Тип::Примитивный(ПримитивныйТип::Строка),
                    Значение::Булево(_) => Тип::Примитивный(ПримитивныйТип::Булево),
                    Значение::Ничего => Тип::Пустой,
                };
                Some(Ast::Литерал { значение: значение.clone(), span: *span })
            }

            Ast::Переменная { имя, тип: _, span } => {
                let typ = self.context.get(имя).cloned().or_else(|| Some(fresh_var()));
                Some(Ast::Переменная { имя: имя.clone(), тип: typ, span: *span })
            }

            Ast::Возврат { значение, span } => {
                let val = значение.as_ref().and_then(|v| self.infer_node(v)).map(Box::new);
                Some(Ast::Возврат { значение: val, span: *span })
            }

            _ => Some(node.clone()),
        }
    }

    fn get_type(&self, node: &Ast) -> Option<Тип> {
        match node {
            Ast::ДвоичноеВыражение { тип, .. } => тип.clone(),
            Ast::Переменная { тип, .. } => тип.clone(),
            _ => None,
        }
    }
}

