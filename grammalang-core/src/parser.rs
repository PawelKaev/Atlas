// grammalang-core/src/parser.rs

use crate::error::{Diagnostic, DiagnosticKind};
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum CstNode {
    Модуль { объявления: Vec<CstNode>, span: (usize, usize) },
    Функция { имя: String, параметры: Vec<Параметр>, возвращаемый_тип: Option<Box<CstNode>>, тело: Box<CstNode>, модификаторы: Vec<String>, span: (usize, usize) },
    Параметр { имя: String, тип: Box<CstNode>, изменяемый: bool },
    Блок { выражения: Vec<CstNode>, span: (usize, usize) },
    ДвоичноеВыражение { левое: Box<CstNode>, оператор: String, правое: Box<CstNode> },
    УнарноеВыражение { оператор: String, операнд: Box<CstNode> },
    Вызов { функция: Box<CstNode>, аргументы: Vec<CstNode> },
    Конвейер { левое: Box<CstNode>, правое: Box<CstNode> },
    ДоступКПолю { объект: Box<CstNode>, поле: String },
    Сопоставление { значение: Box<CstNode>, ветки: Vec<Ветка> },
    Ветка { образец: Box<CstNode>, условие: Option<Box<CstNode>>, тело: Box<CstNode> },
    КонструкторСтруктуры { имя: String, поля: Vec<(String, CstNode)> },
    КонструкторСуммы { имя: String, значение: Option<Box<CstNode>> },
    ОбъявлениеСтруктуры { имя: String, поля: Vec<(String, CstNode)> },
    ОбъявлениеСуммы { имя: String, варианты: Vec<(String, Option<CstNode>)> },
    ОбъявлениеИмпорта { путь: Vec<String>, имена: Vec<String> },
    Если { условие: Box<CstNode>, то: Box<CstNode>, иначе: Option<Box<CstNode>> },
    Возврат(Option<Box<CstNode>>),
    Присваивание { имя: String, изменяемая: bool, значение: Box<CstNode> },
    Заимствование { изменяемое: bool, значение: Box<CstNode> },
    ВнутриЭффекта { эффекты: Vec<String>, тело: Box<CstNode> },
    ВместеБлок { тело: Box<CstNode> },
    ОбразецПеременная(String),
    ОбразецПодчёркивание,
    ОбразецЛитерал(String),
    ОбразецКонструктор { имя: String, вложенный: Option<Box<CstNode>> },
    ОбразецИли(Box<CstNode>, Box<CstNode>),
    ОбразецПривязка { имя: String, образец: Box<CstNode> },
    ОбразецСтруктура { имя: String, поля: Vec<(String, CstNode)>, открытый: bool },
    ОбразецСписок { элементы: Vec<CstNode>, хвост: Option<Box<CstNode>> },
    ТипИмя(String),
    ТипПараметризованный { имя: String, параметры: Vec<CstNode> },
    ТипФункция { аргументы: Vec<CstNode>, результат: Box<CstNode> },
    ТипЗапись { поля: Vec<(String, CstNode)> },
    ТипСсылка { изменяемая: bool, тип: Box<CstNode> },
    Переменная(String),
    Литерал(TokenKind),
}

#[derive(Debug, Clone)]
pub struct Параметр { pub имя: String, pub тип: CstNode, pub изменяемый: bool }

#[derive(Debug, Clone)]
pub struct Ветка { pub образец: Box<CstNode>, pub условие: Option<Box<CstNode>>, pub тело: Box<CstNode> }

pub struct Parser {
    tokens: Vec<Token>,
    pos: usize,
    errors: Vec<Diagnostic>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self { Parser { tokens, pos: 0, errors: Vec::new() } }

    pub fn parse(&mut self) -> (Option<CstNode>, Vec<Diagnostic>) {
        let module = self.parse_module();
        (module, std::mem::take(&mut self.errors))
    }

    fn parse_module(&mut self) -> Option<CstNode> {
        let start = self.pos;
        let mut declarations = Vec::new();
        while !self.is_at_end() {
            self.skip_insignificant();
            if self.is_at_end() { break; }
            if let Some(decl) = self.parse_declaration() {
                declarations.push(decl);
            } else if !self.is_at_end() {
                self.advance();
            }
        }
        Some(CstNode::Модуль { объявления: declarations, span: (start, self.pos) })
    }

    fn skip_insignificant(&mut self) {
        while self.check(&TokenKind::Отступ) || self.check(&TokenKind::ОтменаОтступа) {
            self.advance();
        }
    }

    fn parse_declaration(&mut self) -> Option<CstNode> {
        self.skip_insignificant();
        if self.check(&TokenKind::Открыто) || self.check(&TokenKind::Функция) { return self.parse_function(); }
        if self.check(&TokenKind::Структура) { return self.parse_struct_declaration(); }
        if self.check(&TokenKind::Тип) { return self.parse_sum_declaration(); }
        if self.check(&TokenKind::Импорт) { return self.parse_import(); }
        if self.check(&TokenKind::Модуль) { return self.parse_module_declaration(); }
        None
    }

    fn parse_function(&mut self) -> Option<CstNode> {
        let start = self.pos;
        let mut modifiers = Vec::new();
        self.skip_insignificant();
        if self.eat(&TokenKind::Открыто) { modifiers.push("открыто".to_string()); self.skip_insignificant(); }
        self.expect(&TokenKind::Функция)?;
        self.skip_insignificant();
        let name = self.expect_identifier()?;
        self.skip_insignificant();
        let parameters = self.parse_parameters()?;
        self.skip_insignificant();
        let return_type = if self.eat(&TokenKind::Стрелка) { self.skip_insignificant(); self.parse_type().map(Box::new) } else { None };
        self.skip_insignificant();
        self.expect(&TokenKind::Двоеточие)?;
        let body = self.parse_block()?;
        Some(CstNode::Функция { имя: name, параметры: parameters, возвращаемый_тип: return_type, тело: Box::new(body), модификаторы: modifiers, span: (start, self.pos) })
    }

    fn parse_parameters(&mut self) -> Option<Vec<Параметр>> {
        self.expect(&TokenKind::КруглаяОткрыто)?;
        let mut params = Vec::new();
        if !self.check(&TokenKind::КруглаяЗакрыто) {
            loop {
                let изменяемый = self.eat(&TokenKind::Изм);
                let name = self.expect_identifier()?;
                self.expect(&TokenKind::Двоеточие)?;
                let typ = self.parse_type()?;
                params.push(Параметр { имя: name, тип: typ, изменяемый });
                if !self.eat(&TokenKind::Запятая) { break; }
            }
        }
        self.expect(&TokenKind::КруглаяЗакрыто)?;
        Some(params)
    }

    fn parse_struct_declaration(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Структура)?;
        self.skip_insignificant();
        let name = self.expect_identifier()?;
        self.skip_insignificant();
        self.expect(&TokenKind::Двоеточие)?;
        let mut fields = Vec::new();
        while !self.is_at_end() && !self.check(&TokenKind::Функция) && !self.check(&TokenKind::Структура) && !self.check(&TokenKind::Тип) && !self.check(&TokenKind::Открыто) && !self.check(&TokenKind::Импорт) && !self.check(&TokenKind::КонецФайла) && !self.check(&TokenKind::ОтменаОтступа) {
            if let Some(ident) = self.eat_identifier() {
                self.expect(&TokenKind::Двоеточие)?;
                let typ = self.parse_type()?;
                fields.push((ident, typ));
            } else { self.advance(); }
        }
        Some(CstNode::ОбъявлениеСтруктуры { имя: name, поля: fields })
    }

    fn parse_sum_declaration(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Тип)?;
        self.skip_insignificant();
        let name = self.expect_identifier()?;
        self.skip_insignificant();
        self.expect(&TokenKind::Равно)?;
        self.skip_insignificant();
        let mut variants = Vec::new();
        loop {
            let variant_name = self.expect_identifier()?;
            let data = if self.eat(&TokenKind::КруглаяОткрыто) { let typ = self.parse_type(); self.expect(&TokenKind::КруглаяЗакрыто)?; typ } else { None };
            variants.push((variant_name, data));
            if !self.eat(&TokenKind::ВертикальнаяЧерта) { break; }
        }
        Some(CstNode::ОбъявлениеСуммы { имя: name, варианты: variants })
    }

    fn parse_import(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Импорт)?;
        let mut path = Vec::new();
        let mut names = Vec::new();
        path.push(self.expect_identifier()?);
        while self.eat(&TokenKind::Точка) {
            if self.eat(&TokenKind::Звёздочка) { names.push("*".to_string()); break; }
            path.push(self.expect_identifier()?);
        }
        Some(CstNode::ОбъявлениеИмпорта { путь: path, имена: names })
    }

    fn parse_module_declaration(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Модуль)?;
        self.skip_insignificant();
        let _name = self.expect_identifier()?;
        self.skip_insignificant();
        self.expect(&TokenKind::Двоеточие)?;
        let mut declarations = Vec::new();
        while !self.is_at_end() {
            self.skip_insignificant();
            if self.is_at_end() { break; }
            if let Some(decl) = self.parse_declaration() { declarations.push(decl); } else if !self.is_at_end() { self.advance(); }
        }
        Some(CstNode::Модуль { объявления: declarations, span: (0, self.pos) })
    }

    fn parse_expression(&mut self) -> Option<CstNode> {
        if self.check(&TokenKind::Пусть) { return self.parse_let(); }
        self.parse_assignment()
    }

    fn parse_let(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Пусть)?;
        let имя = self.expect_identifier()?;
        self.expect(&TokenKind::Равно)?;
        let значение = Box::new(self.parse_expression()?);
        Some(CstNode::Присваивание { имя, изменяемая: false, значение })
    }

    fn parse_assignment(&mut self) -> Option<CstNode> {
        let left = self.parse_pipeline()?;
        self.skip_insignificant();
        if self.check(&TokenKind::Равно) {
            self.advance();
            self.skip_insignificant();
            let right = self.parse_assignment()?;
            if let CstNode::Переменная(name) = left { return Some(CstNode::Присваивание { имя: name, изменяемая: false, значение: Box::new(right) }); }
        }
        Some(left)
    }

    fn parse_pipeline(&mut self) -> Option<CstNode> {
        let mut left = self.parse_or()?;
        while self.eat(&TokenKind::Конвейер) { let right = self.parse_pipeline()?; left = CstNode::Конвейер { левое: Box::new(left), правое: Box::new(right) }; }
        Some(left)
    }

    fn parse_or(&mut self) -> Option<CstNode> {
        let mut left = self.parse_and()?;
        while self.eat_identifier_is("или") { let right = self.parse_and()?; left = CstNode::ДвоичноеВыражение { левое: Box::new(left), оператор: "или".to_string(), правое: Box::new(right) }; }
        Some(left)
    }

    fn parse_and(&mut self) -> Option<CstNode> {
        let mut left = self.parse_comparison()?;
        while self.eat_identifier_is("и") { let right = self.parse_comparison()?; left = CstNode::ДвоичноеВыражение { левое: Box::new(left), оператор: "и".to_string(), правое: Box::new(right) }; }
        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<CstNode> {
        let mut left = self.parse_addition()?;
        let ops = [TokenKind::ДваРавно, TokenKind::НеРавно, TokenKind::Меньше, TokenKind::Больше, TokenKind::МеньшеРавно, TokenKind::БольшеРавно];
        if self.check_any(&ops) { let op_str = format!("{:?}", self.advance().kind); let right = self.parse_addition()?; left = CstNode::ДвоичноеВыражение { левое: Box::new(left), оператор: op_str, правое: Box::new(right) }; }
        Some(left)
    }

    fn parse_addition(&mut self) -> Option<CstNode> {
        let mut left = self.parse_multiplication()?;
        self.skip_insignificant();
        while self.check(&TokenKind::Плюс) || self.check(&TokenKind::Минус) { let op = self.advance().lexeme.clone(); self.skip_insignificant(); let right = self.parse_multiplication()?; left = CstNode::ДвоичноеВыражение { левое: Box::new(left), оператор: op, правое: Box::new(right) }; }
        Some(left)
    }

    fn parse_multiplication(&mut self) -> Option<CstNode> {
        let mut left = self.parse_unary()?;
        self.skip_insignificant();
        while self.check(&TokenKind::Звёздочка) || self.check(&TokenKind::Слэш) || self.check(&TokenKind::Процент) { let op = self.advance().lexeme.clone(); self.skip_insignificant(); let right = self.parse_unary()?; left = CstNode::ДвоичноеВыражение { левое: Box::new(left), оператор: op, правое: Box::new(right) }; }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<CstNode> {
        if self.eat_identifier_is("не") || self.check(&TokenKind::Минус) {
            let op = self.advance().lexeme.clone();
            let operand = self.parse_unary()?;
            return Some(CstNode::УнарноеВыражение { оператор: op, операнд: Box::new(operand) });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Option<CstNode> {
        let mut expr = self.parse_primary()?;
        loop {
            self.skip_insignificant();
            if self.eat(&TokenKind::Точка) { let field = self.expect_identifier()?; expr = CstNode::ДоступКПолю { объект: Box::new(expr), поле: field }; }
            else if self.check(&TokenKind::КруглаяОткрыто) { let args = self.parse_arguments()?; expr = CstNode::Вызов { функция: Box::new(expr), аргументы: args }; }
            else if self.eat(&TokenKind::Вопрос) { expr = CstNode::УнарноеВыражение { оператор: "?".to_string(), операнд: Box::new(expr) }; }
            else { break; }
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<CstNode> {
        let token = self.peek()?.clone();
        match &token.kind {
            TokenKind::Целое(_) | TokenKind::Десятичное(_) | TokenKind::Строка(_) => { self.advance(); Some(CstNode::Литерал(token.kind.clone())) }
            TokenKind::Истина | TokenKind::Ложь | TokenKind::Ничего => { self.advance(); Some(CstNode::Литерал(token.kind.clone())) }
            TokenKind::Идентификатор(_) => {
                let name = token.lexeme.clone();
                let is_uppercase = name.chars().next().map_or(false, |c| c.is_uppercase());
                self.advance();
                self.skip_insignificant();
                
                if self.check(&TokenKind::КруглаяОткрыто) {
                    let args = self.parse_arguments()?;
                    if is_uppercase {
                        if args.len() == 1 {
                            return Some(CstNode::КонструкторСуммы { имя: name, значение: Some(Box::new(args.into_iter().next().unwrap())) });
                        } else {
                            return Some(CstNode::КонструкторСуммы { имя: name, значение: None });
                        }
                    } else {
                        return Some(CstNode::Вызов {
                            функция: Box::new(CstNode::Переменная(name)),
                            аргументы: args,
                        });
                    }
                }
                
                if self.check(&TokenKind::ФигурнаяОткрыто) && is_uppercase {
                    return self.parse_struct_init(&name);
                }
                
                Some(CstNode::Переменная(name))
            }
            TokenKind::ФигурнаяОткрыто => self.parse_block(),
            TokenKind::Если => self.parse_if(),
            TokenKind::Сопоставить => self.parse_match(),
            TokenKind::Вернуть => self.parse_return(),
            TokenKind::Внутри => self.parse_inside_effect(),
            TokenKind::Вместе => self.parse_together(),
            TokenKind::Амперсанд => self.parse_borrow(),
            TokenKind::КруглаяОткрыто => { self.advance(); let expr = self.parse_expression(); self.expect(&TokenKind::КруглаяЗакрыто)?; expr }
            _ => { self.error(&format!("Неожиданный токен: {}", token)); self.advance(); None }
        }
    }
    fn parse_struct_init(&mut self, name: &str) -> Option<CstNode> {
        self.expect(&TokenKind::ФигурнаяОткрыто)?;
        let mut fields = Vec::new();
        if !self.check(&TokenKind::ФигурнаяЗакрыто) {
            loop {
                let field_name = self.expect_identifier()?;
                let field_value = if self.eat(&TokenKind::Двоеточие) {
                    self.parse_expression()?
                } else {
                    CstNode::Переменная(field_name.clone())
                };
                fields.push((field_name, field_value));
                if !self.eat(&TokenKind::Запятая) { break; }
            }
        }
        self.expect(&TokenKind::ФигурнаяЗакрыто)?;
        Some(CstNode::КонструкторСтруктуры { имя: name.to_string(), поля: fields })
    }

    fn parse_arguments(&mut self) -> Option<Vec<CstNode>> {
        self.expect(&TokenKind::КруглаяОткрыто)?;
        let mut args = Vec::new();
        if !self.check(&TokenKind::КруглаяЗакрыто) {
            loop {
                if let Some(expr) = self.parse_expression() { args.push(expr); }
                if !self.eat(&TokenKind::Запятая) { break; }
            }
        }
        self.expect(&TokenKind::КруглаяЗакрыто)?;
        Some(args)
    }

    fn parse_block(&mut self) -> Option<CstNode> {
        let start = self.pos;
        let mut expressions = Vec::new();
        if self.eat(&TokenKind::ФигурнаяОткрыто) {
            while !self.check(&TokenKind::ФигурнаяЗакрыто) && !self.is_at_end() {
                if let Some(expr) = self.parse_expression() { expressions.push(expr); }
            }
            self.expect(&TokenKind::ФигурнаяЗакрыто)?;
        } else if self.eat(&TokenKind::Отступ) {
            loop {
                if self.is_at_end() { break; }
                if self.check(&TokenKind::ОтменаОтступа) { self.advance(); break; }
                if self.check(&TokenKind::Отступ) { self.advance(); continue; }
                if let Some(expr) = self.parse_expression() { expressions.push(expr); }
                else { self.advance(); }
            }
        } else {
            if let Some(expr) = self.parse_expression() { expressions.push(expr); }
        }
        Some(CstNode::Блок { выражения: expressions, span: (start, self.pos) })
    }

    fn parse_if(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Если)?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::Двоеточие)?;
        let then_branch = self.parse_block()?;
        let else_branch = if self.eat(&TokenKind::Иначе) {
            if self.check(&TokenKind::Если) { Some(Box::new(self.parse_if()?)) }
            else { self.expect(&TokenKind::Двоеточие)?; Some(Box::new(self.parse_block()?)) }
        } else { None };
        Some(CstNode::Если { условие: Box::new(condition), то: Box::new(then_branch), иначе: else_branch })
    }

    fn parse_match(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Сопоставить)?;
        let value = self.parse_expression()?;
        self.expect(&TokenKind::Двоеточие)?;
        let mut branches = Vec::new();
        loop {
            if self.is_at_end() { break; }
            if self.check(&TokenKind::ОтменаОтступа) { self.advance(); break; }
            let pattern = self.parse_pattern()?;
            let guard = if self.eat_identifier_is("если") { Some(Box::new(self.parse_expression()?)) } else { None };
            self.expect(&TokenKind::Стрелка)?;
            self.skip_insignificant();
            let body = self.parse_expression()?;
            branches.push(Ветка { образец: Box::new(pattern), условие: guard, тело: Box::new(body) });
        }
        Some(CstNode::Сопоставление { значение: Box::new(value), ветки: branches })
    }

    fn parse_pattern(&mut self) -> Option<CstNode> {
        self.parse_pattern_or()
    }

    fn parse_pattern_or(&mut self) -> Option<CstNode> {
        let mut left = self.parse_pattern_atom()?;
        while self.eat(&TokenKind::ВертикальнаяЧерта) {
            let right = self.parse_pattern_atom()?;
            left = CstNode::ОбразецИли(Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_pattern_atom(&mut self) -> Option<CstNode> {
        if self.eat(&TokenKind::Подчёркивание) { return Some(CstNode::ОбразецПодчёркивание); }
        if let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Идентификатор(name) => {
                    let name = name.clone();
                    let is_uppercase = name.chars().next().map_or(false, |c| c.is_uppercase());
                    self.advance();
                    if self.check(&TokenKind::Собака) {
                        self.advance();
                        let inner = self.parse_pattern_atom()?;
                        return Some(CstNode::ОбразецПривязка { имя: name, образец: Box::new(inner) });
                    }
                    if self.check(&TokenKind::КруглаяОткрыто) {
                        self.advance();
                        let inner = self.parse_pattern();
                        self.expect(&TokenKind::КруглаяЗакрыто)?;
                        return Some(CstNode::ОбразецКонструктор { имя: name, вложенный: inner.map(Box::new) });
                    }
                    if self.check(&TokenKind::ФигурнаяОткрыто) && is_uppercase {
                        self.advance();
                        let mut fields = Vec::new();
                        let mut открытый = false;
                        if !self.check(&TokenKind::ФигурнаяЗакрыто) {
                            loop {
                                if self.eat(&TokenKind::Многоточие) { открытый = true; break; }
                                let field_name = self.expect_identifier()?;
                                let field_pattern = if self.eat(&TokenKind::Двоеточие) {
                                    self.parse_pattern()?
                                } else {
                                    CstNode::ОбразецПеременная(field_name.clone())
                                };
                                fields.push((field_name, field_pattern));
                                if !self.eat(&TokenKind::Запятая) {
                                    if self.eat(&TokenKind::Многоточие) { открытый = true; }
                                    break;
                                }
                            }
                        }
                        self.expect(&TokenKind::ФигурнаяЗакрыто)?;
                        return Some(CstNode::ОбразецСтруктура { имя: name, поля: fields, открытый });
                    }
                    if is_uppercase { return Some(CstNode::ОбразецКонструктор { имя: name, вложенный: None }); }
                    Some(CstNode::ОбразецПеременная(name))
                }
                TokenKind::Целое(_) | TokenKind::Строка(_) => { let lit = token.lexeme.clone(); self.advance(); Some(CstNode::ОбразецЛитерал(lit)) }
                TokenKind::Ничего => {
                    self.advance();
                    Some(CstNode::ОбразецКонструктор { имя: "Ничего".to_string(), вложенный: None })
                }
                _ => { self.error("Ожидался образец"); None }
            }
        } else { None }
    }

    fn parse_return(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Вернуть)?;
        if self.check(&TokenKind::ОтменаОтступа) || self.is_at_end() { return Some(CstNode::Возврат(None)); }
        let expr = self.parse_expression();
        Some(CstNode::Возврат(expr.map(Box::new)))
    }

    fn parse_inside_effect(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Внутри)?;
        let mut effects = Vec::new();
        loop {
            if let Some(name) = self.eat_identifier() { effects.push(name); } else { break; }
            if !self.eat(&TokenKind::Запятая) { break; }
        }
        self.expect(&TokenKind::Двоеточие)?;
        let body = self.parse_block()?;
        Some(CstNode::ВнутриЭффекта { эффекты: effects, тело: Box::new(body) })
    }

    fn parse_together(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Вместе)?;
        self.expect(&TokenKind::Двоеточие)?;
        let body = self.parse_block()?;
        Some(CstNode::ВместеБлок { тело: Box::new(body) })
    }

    fn parse_borrow(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Амперсанд)?;
        let изменяемое = self.eat(&TokenKind::Изм);
        let value = self.parse_primary()?;
        Some(CstNode::Заимствование { изменяемое, значение: Box::new(value) })
    }

    fn parse_type(&mut self) -> Option<CstNode> {
        let typ = self.parse_type_primary()?;
        if self.eat(&TokenKind::Стрелка) { self.skip_insignificant(); let result = self.parse_type()?; return Some(CstNode::ТипФункция { аргументы: vec![typ], результат: Box::new(result) }); }
        Some(typ)
    }

    fn parse_type_primary(&mut self) -> Option<CstNode> {
        if self.eat(&TokenKind::Амперсанд) {
            let изменяемая = self.eat(&TokenKind::Изм);
            let typ = self.parse_type_primary()?;
            return Some(CstNode::ТипСсылка { изменяемая, тип: Box::new(typ) });
        }
        if self.eat(&TokenKind::ФигурнаяОткрыто) {
            let mut fields = Vec::new();
            if !self.check(&TokenKind::ФигурнаяЗакрыто) {
                loop {
                    let name = self.expect_identifier()?;
                    self.expect(&TokenKind::Двоеточие)?;
                    let typ = self.parse_type()?;
                    fields.push((name, typ));
                    if !self.eat(&TokenKind::Запятая) { break; }
                }
            }
            self.expect(&TokenKind::ФигурнаяЗакрыто)?;
            return Some(CstNode::ТипЗапись { поля: fields });
        }
        let name = self.expect_identifier()?;
        if self.eat(&TokenKind::Меньше) {
            let mut params = Vec::new();
            loop {
                if let Some(t) = self.parse_type() { params.push(t); }
                if !self.eat(&TokenKind::Запятая) { break; }
            }
            self.expect(&TokenKind::Больше)?;
            Some(CstNode::ТипПараметризованный { имя: name, параметры: params })
        } else {
            Some(CstNode::ТипИмя(name))
        }
    }

    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn advance(&mut self) -> &Token { let token = &self.tokens[self.pos]; self.pos += 1; token }
    fn is_at_end(&self) -> bool { self.pos >= self.tokens.len() || matches!(self.peek(), Some(t) if matches!(t.kind, TokenKind::КонецФайла)) }
    fn check(&self, kind: &TokenKind) -> bool { self.peek().map_or(false, |t| std::mem::discriminant(&t.kind) == std::mem::discriminant(kind)) }
    fn check_any(&self, kinds: &[TokenKind]) -> bool { kinds.iter().any(|k| self.check(k)) }
    fn eat(&mut self, kind: &TokenKind) -> bool { if self.check(kind) { self.advance(); true } else { false } }
    fn eat_identifier(&mut self) -> Option<String> {
        if let Some(token) = self.peek() {
            if let TokenKind::Идентификатор(name) = &token.kind { let name = name.clone(); self.advance(); return Some(name); }
        }
        None
    }
    fn eat_identifier_is(&mut self, text: &str) -> bool {
        if let Some(token) = self.peek() {
            if let TokenKind::Идентификатор(name) = &token.kind { if name == text { self.advance(); return true; } }
        }
        false
    }
    fn expect(&mut self, kind: &TokenKind) -> Option<&Token> {
        if self.check(kind) { Some(self.advance()) }
        else { let found = self.peek().map(|t| t.lexeme.clone()).unwrap_or_default(); self.error(&format!("Ожидался {:?}, найдено '{}'", kind, found)); None }
    }
    fn expect_identifier(&mut self) -> Option<String> {
        if let Some(name) = self.eat_identifier() { Some(name) }
        else { let found = self.peek().map(|t| t.lexeme.clone()).unwrap_or_default(); self.error(&format!("Ожидался идентификатор, найдено '{}'", found)); None }
    }
    fn error(&mut self, message: &str) {
        let token = self.peek().cloned().unwrap_or(Token { kind: TokenKind::КонецФайла, lexeme: "".to_string(), span: crate::token::Span { line: 0, column: 0, offset: 0 } });
        let span = token.span;
        
        let context_start = if self.pos > 5 { self.pos - 5 } else { 0 };
        let context_end = usize::min(self.pos + 5, self.tokens.len());
        let snippet: Vec<String> = self.tokens[context_start..context_end]
            .iter()
            .map(|t| format!("{:?}", t.kind))
            .collect();
        
        eprintln!("--- GRAMMALANG PARSER CRASH DEBUG ---");
        eprintln!("Сообщение: {}", message);
        eprintln!("Лексема: '{}' в строке {}, колонка {}", token.lexeme, span.line, span.column);
        eprintln!("Контекст токенов: {:?}", snippet);
        eprintln!("-------------------------------------");
        
        self.errors.push(Diagnostic { kind: DiagnosticKind::Ошибка, message: message.to_string(), span, hint: None });
    }
}
