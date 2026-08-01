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
            CstNode::Модуль { объявления, .. } => {
                let decls: Vec<Ast> = объявления.iter().filter_map(|d| self.desugar_node(d)).collect();
                Some(Ast::Модуль { имя: "".to_string(), объявления: decls, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::Функция { имя, параметры, возвращаемый_тип, тело, модификаторы, .. } => {
                let params: Vec<Параметр> = параметры.iter().map(|p| {
                    let typ = self.desugar_type(&p.тип);
                    Параметр { имя: p.имя.clone(), тип: typ.unwrap_or(Тип::Пустой), изменяемый: p.изменяемый }
                }).collect();
                let ret = возвращаемый_тип.as_ref().and_then(|t| self.desugar_type(t));
                let body = self.desugar_node(тело)?;
                Some(Ast::ОбъявлениеФункции {
                    имя: имя.clone(), параметры_типа: Vec::new(), параметры: params,
                    возвращаемый_тип: ret, тело: Box::new(body),
                    открыто: модификаторы.contains(&"открыто".to_string()),
                    span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Блок { выражения, .. } => {
                let exprs: Vec<Ast> = выражения.iter().filter_map(|e| self.desugar_node(e)).collect();
                Some(Ast::Блок { выражения: exprs, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::ДвоичноеВыражение { левое, оператор, правое } => {
                let left = self.desugar_node(левое)?;
                let right = self.desugar_node(правое)?;
                let op = match оператор.as_str() {
                    "+" => БинарныйОператор::Сложение,
                    "-" => БинарныйОператор::Вычитание,
                    "*" => БинарныйОператор::Умножение,
                    "/" => БинарныйОператор::Деление,
                    "%" => БинарныйОператор::Остаток,
                    "==" | "ДваРавно" => БинарныйОператор::Равно,
                    "!=" | "НеРавно" => БинарныйОператор::НеРавно,
                    "<" | "Меньше" => БинарныйОператор::Меньше,
                    ">" | "Больше" => БинарныйОператор::Больше,
                    "<=" | "МеньшеРавно" => БинарныйОператор::МеньшеРавно,
                    ">=" | "БольшеРавно" => БинарныйОператор::БольшеРавно,
                    "и" => БинарныйОператор::И,
                    "или" => БинарныйОператор::Или,
                    "++" => БинарныйОператор::Конкатенация,
                    _ => { self.error(&format!("Неизвестный оператор: {}", оператор)); return None; }
                };
                Some(Ast::ДвоичноеВыражение {
                    левое: Box::new(left), оператор: op, правое: Box::new(right),
                    тип: None, span: Span { line: 1, column: 1, offset: 0 },
                })
            }

            CstNode::Конвейер { левое, правое } => {
                let left_ast = self.desugar_node(левое)?;
                match правое.as_ref() {
                    CstNode::Вызов { функция, аргументы } => {
                        let func = self.desugar_node(функция)?;
                        let mut args = vec![left_ast];
                        for arg in аргументы {
                            if let Some(a) = self.desugar_node(arg) { args.push(a); }
                        }
                        Some(Ast::Вызов { функция: Box::new(func), аргументы: args, тип: None, span: Span { line: 1, column: 1, offset: 0 } })
                    }
                    _ => {
                        let right_ast = self.desugar_node(правое)?;
                        Some(Ast::Вызов { функция: Box::new(right_ast), аргументы: vec![left_ast], тип: None, span: Span { line: 1, column: 1, offset: 0 } })
                    }
                }
            }

            CstNode::Сопоставление { значение, ветки, .. } => {
                let val = self.desugar_node(значение)?;
                let branches: Vec<ВеткаСопоставления> = ветки.iter().filter_map(|b| {
                    let pattern = self.desugar_pattern(&b.образец)?;
                    let guard = b.условие.as_ref().and_then(|g| self.desugar_node(g)).map(Box::new);
                    let body = self.desugar_node(&b.тело)?;
                    Some(ВеткаСопоставления { образец: pattern, условие: guard, тело: Box::new(body) })
                }).collect();
                Some(Ast::Сопоставление { значение: Box::new(val), ветки: branches, тип: None, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::Если { условие, то, иначе } => {
                let cond = self.desugar_node(условие)?;
                let then_branch = self.desugar_node(то)?;
                let else_branch = иначе.as_ref().and_then(|e| self.desugar_node(e)).map(Box::new);
                Some(Ast::Если { условие: Box::new(cond), то: Box::new(then_branch), иначе: else_branch, тип: None, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::Возврат(значение) => {
                let val = значение.as_ref().and_then(|v| self.desugar_node(v)).map(Box::new);
                Some(Ast::Возврат { значение: val, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::Присваивание { имя, изменяемая, значение } => {
                let val = self.desugar_node(значение)?;
                Some(Ast::Присваивание { имя: имя.clone(), тип_аннотация: None, изменяемая: *изменяемая, значение: Box::new(val), span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::Заимствование { изменяемое, значение } => {
                let val = self.desugar_node(значение)?;
                Some(Ast::Заимствование { изменяемое: *изменяемое, значение: Box::new(val), тип: None, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::ВнутриЭффекта { эффекты, тело } => {
                let body = self.desugar_node(тело)?;
                let mut result = body;
                for эффект in эффекты.iter().rev() {
                    result = Ast::БлокЭффекта { эффекты: vec![эффект.clone()], тело: Box::new(result), span: Span { line: 1, column: 1, offset: 0 } };
                }
                Some(result)
            }

            CstNode::ВместеБлок { тело } => {
                let body = self.desugar_node(тело)?;
                Some(Ast::ПараллельныйБлок { стратегия: СтратегияПараллельности::БыстрыйОтказ, тело: Box::new(body), span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::Вызов { функция, аргументы } => {
                let func = self.desugar_node(функция)?;
                let args: Vec<Ast> = аргументы.iter().filter_map(|a| self.desugar_node(a)).collect();
                Some(Ast::Вызов { функция: Box::new(func), аргументы: args, тип: None, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::ДоступКПолю { объект, поле } => {
                let obj = self.desugar_node(объект)?;
                Some(Ast::ДоступКПолю { объект: Box::new(obj), поле: поле.clone(), тип: None, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::КонструкторСтруктуры { имя, поля } => {
                let fields: Vec<(String, Ast)> = поля.iter().filter_map(|(n, v)| self.desugar_node(v).map(|ast| (n.clone(), ast))).collect();
                Some(Ast::КонструкторСтруктуры { имя: имя.clone(), поля: fields, тип: None, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::КонструкторСуммы { имя, значение } => {
                let val = значение.as_ref().and_then(|v| self.desugar_node(v)).map(Box::new);
                Some(Ast::КонструкторСуммы { имя: имя.clone(), значение: val, тип: None, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::УнарноеВыражение { оператор, операнд } => {
                let operand = self.desugar_node(операнд)?;
                let op = match оператор.as_str() {
                    "-" => УнарныйОператор::Отрицание,
                    "не" => УнарныйОператор::Не,
                    "?" => УнарныйОператор::Вопрос,
                    _ => { self.error(&format!("Неизвестный унарный оператор: {}", оператор)); return None; }
                };
                Some(Ast::УнарноеВыражение { оператор: op, операнд: Box::new(operand), тип: None, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::Переменная(имя) => Some(Ast::Переменная { имя: имя.clone(), тип: None, span: Span { line: 1, column: 1, offset: 0 } }),

            CstNode::Литерал(lit) => {
                let value = match lit {
                    crate::token::TokenKind::Целое(n) => Значение::Целое(*n),
                    crate::token::TokenKind::Десятичное(f) => Значение::Десятичное(*f),
                    crate::token::TokenKind::Строка(s) => Значение::Строка(s.clone()),
                    crate::token::TokenKind::Истина => Значение::Булево(true),
                    crate::token::TokenKind::Ложь => Значение::Булево(false),
                    crate::token::TokenKind::Ничего => Значение::Ничего,
                    _ => { self.error("Неизвестный литерал"); return None; }
                };
                Some(Ast::Литерал { значение: value, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::ОбъявлениеСтруктуры { имя, поля } => {
                let fields: Vec<(String, Тип)> = поля.iter().filter_map(|(n, t)| self.desugar_type(t).map(|typ| (n.clone(), typ))).collect();
                Some(Ast::ОбъявлениеСтруктуры { имя: имя.clone(), параметры_типа: Vec::new(), поля: fields, открыто: false, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::ОбъявлениеСуммы { имя, варианты } => {
                let variants: Vec<ВариантСуммы> = варианты.iter().map(|(n, t)| ВариантСуммы { имя: n.clone(), тип_данных: t.as_ref().and_then(|typ| self.desugar_type(typ)) }).collect();
                Some(Ast::ОбъявлениеСуммы { имя: имя.clone(), параметры_типа: Vec::new(), варианты: variants, открыто: false, span: Span { line: 1, column: 1, offset: 0 } })
            }

            CstNode::ОбъявлениеИмпорта { путь, имена } => {
                Some(Ast::ОбъявлениеИмпорта { путь: путь.clone(), имена: имена.iter().map(|n| (n.clone(), None)).collect(), span: Span { line: 1, column: 1, offset: 0 } })
            }

            _ => { self.error(&format!("Десахаринг не реализован для: {:?}", node)); None }
        }
    }

    fn desugar_type(&self, node: &CstNode) -> Option<Тип> {
        match node {
            CstNode::ТипИмя(name) => match name.as_str() {
                "Целое" => Some(Тип::Примитивный(ПримитивныйТип::Целое)),
                "Десятичное" => Some(Тип::Примитивный(ПримитивныйТип::Десятичное)),
                "Булево" => Some(Тип::Примитивный(ПримитивныйТип::Булево)),
                "Строка" => Some(Тип::Примитивный(ПримитивныйТип::Строка)),
                _ => Some(Тип::Переменная(name.clone())),
            },
            CstNode::ТипПараметризованный { имя, параметры } => {
                let params: Vec<Тип> = параметры.iter().filter_map(|p| self.desugar_type(p)).collect();
                Some(Тип::Параметризованный { имя: имя.clone(), параметры: params })
            }
            CstNode::ТипФункция { аргументы, результат } => {
                let args: Vec<Тип> = аргументы.iter().filter_map(|a| self.desugar_type(a)).collect();
                let ret = self.desugar_type(результат)?;
                Some(Тип::Функция { аргументы: args, результат: Box::new(ret) })
            }
            CstNode::ТипЗапись { поля } => {
                let fields: Vec<(String, Тип)> = поля.iter().filter_map(|(n, t)| self.desugar_type(t).map(|typ| (n.clone(), typ))).collect();
                Some(Тип::Запись(fields))
            }
            CstNode::ТипСсылка { изменяемая, тип } => {
                let typ = self.desugar_type(тип)?;
                Some(Тип::Ссылка { изменяемая: *изменяемая, тип: Box::new(typ) })
            }
            _ => Some(Тип::Пустой),
        }
    }

    fn desugar_pattern(&mut self, node: &CstNode) -> Option<Образец> {
        match node {
            CstNode::ОбразецПодчёркивание => Some(Образец::Подчёркивание),
            CstNode::ОбразецПеременная(name) => {
                if name.chars().next().map_or(false, |c| c.is_uppercase()) {
                    Some(Образец::Конструктор { имя: name.clone(), вложенный: None })
                } else {
                    Some(Образец::Переменная(name.clone()))
                }
            }
            CstNode::ОбразецЛитерал(lit) => Some(Образец::Литерал(Значение::Строка(lit.clone()))),
            CstNode::ОбразецКонструктор { имя, вложенный } => {
                let inner = вложенный.as_ref().and_then(|p| self.desugar_pattern(p)).map(Box::new);
                Some(Образец::Конструктор { имя: имя.clone(), вложенный: inner })
            }
            _ => { self.error(&format!("Неизвестный образец: {:?}", node)); None }
        }
    }

    fn error(&mut self, message: &str) {
        self.errors.push(Diagnostic {
            kind: DiagnosticKind::Ошибка,
            message: message.to_string(),
            span: Span { line: 1, column: 1, offset: 0 },
            hint: None,
        });
    }
}
