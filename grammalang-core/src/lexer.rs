// grammalang-core/src/lexer.rs

use crate::error::{Diagnostic, DiagnosticKind, Result};
use crate::token::{Span, Token, TokenKind};

/// Лексер — превращает исходный текст в поток токенов
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
    errors: Vec<Diagnostic>,
    /// Стек уровней отступа
    indent_stack: Vec<usize>,
    /// Начало текущей строки (для расчёта отступа)
    line_start: bool,
}

impl Lexer {
    pub fn new(source: &str) -> Self {
        Lexer {
            source: source.chars().collect(),
            pos: 0,
            line: 1,
            column: 1,
            tokens: Vec::new(),
            errors: Vec::new(),
            indent_stack: vec![0],
            line_start: true,
        }
    }

    /// Главный метод — разобрать весь исходный текст в токены
    pub fn tokenize(&mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        while !self.is_at_end() {
            let c = self.peek();

            match c {
                // Пробелы и отступы
                ' ' if self.line_start => self.handle_indent(),
                ' ' | '\t' | '\r' => {
                    // табуляция в отступах — ошибка
                    if c == '\t' && self.line_start {
                        self.error("Табуляция запрещена. Используйте пробелы для отступов.");
                    }
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                    self.line_start = true;
                }

                // Комментарии
                '/' if self.peek_next() == '/' => self.skip_line_comment(),
                '/' if self.peek_next() == '*' => self.skip_block_comment(),

                // Строки
                '"' => self.read_string(),

                // Числа
                '0'..='9' => self.read_number(),
                '-' if self.peek_next().is_ascii_digit() => self.read_number(),

                // Идентификаторы и ключевые слова
                'а'..='я' | 'ё' | 'А'..='Я' | 'Ё' | 'a'..='z' | 'A'..='Z' | '_' => {
                    self.read_identifier()
                }

                // Операторы и разделители
                '+' => self.single_token(TokenKind::Плюс),
                '*' => self.single_token(TokenKind::Звёздочка),
                '%' => self.single_token(TokenKind::Процент),
                '(' => self.single_token(TokenKind::КруглаяОткрыто),
                ')' => self.single_token(TokenKind::КруглаяЗакрыто),
                '{' => self.single_token(TokenKind::ФигурнаяОткрыто),
                '}' => self.single_token(TokenKind::ФигурнаяЗакрыто),
                '[' => self.single_token(TokenKind::КвадратнаяОткрыто),
                ']' => self.single_token(TokenKind::КвадратнаяЗакрыто),
                ',' => self.single_token(TokenKind::Запятая),
                ':' => self.single_token(TokenKind::Двоеточие),
                ';' => self.single_token(TokenKind::ТочкаСЗапятой),
                '?' => self.single_token(TokenKind::Вопрос),

                '=' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::ДваРавно)
                    } else {
                        self.single_token(TokenKind::Равно)
                    }
                }
                '!' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::НеРавно)
                    } else {
                        self.error("Неожиданный символ '!'");
                        self.advance();
                    }
                }
                '<' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::МеньшеРавно)
                    } else {
                        self.single_token(TokenKind::Меньше)
                    }
                }
                '>' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::БольшеРавно)
                    } else if self.peek_next() == '>' {
                        self.double_token(TokenKind::Композиция)
                    } else {
                        self.single_token(TokenKind::Больше)
                    }
                }
                '-' => {
                    if self.peek_next() == '>' {
                        self.double_token(TokenKind::Стрелка)
                    } else if self.peek_next().is_ascii_digit() {
                        self.read_number()
                    } else {
                        self.single_token(TokenKind::Минус)
                    }
                }
                '|' => {
                    if self.peek_next() == '>' {
                        self.double_token(TokenKind::Конвейер)
                    } else {
                        self.single_token(TokenKind::ВертикальнаяЧерта)
                    }
                }
                '&' => self.single_token(TokenKind::Амперсанд),
                '.' => {
                    if self.peek_next() == '.' && self.peek_n(2) == '.' {
                        self.triple_token(TokenKind::Многоточие)
                    } else {
                        self.single_token(TokenKind::Точка)
                    }
                }
                '_' => self.single_token(TokenKind::Подчёркивание),

                _ => {
                    self.error(&format!("Неожиданный символ: '{}'", c));
                    self.advance();
                }
            }
        }

        // В конце файла — закрыть все оставшиеся отступы
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.push_token(TokenKind::ОтменаОтступа, "");
        }

        self.push_token(TokenKind::КонецФайла, "");
        (std::mem::take(&mut self.tokens), std::mem::take(&mut self.errors))
    }

    // ============ Обработка отступов ============

    fn handle_indent(&mut self) {
        let mut spaces = 0;
        let start = self.current_span();

        while self.peek() == ' ' {
            self.advance();
            spaces += 1;
        }

        // Пустая строка — игнорируем
        if self.peek() == '\n' || self.peek() == '\r' {
            return;
        }

        let current = *self.indent_stack.last().unwrap_or(&0);

        if spaces > current {
            self.indent_stack.push(spaces);
            self.push_token_at(TokenKind::Отступ, "", start);
        } else if spaces < current {
            while *self.indent_stack.last().unwrap() > spaces {
                self.indent_stack.pop();
                self.push_token_at(TokenKind::ОтменаОтступа, "", start);
            }
            if *self.indent_stack.last().unwrap() != spaces {
                self.error_at("Неверный отступ. Ожидался уровень, кратный 4 пробелам.", start);
            }
        }

        self.line_start = false;
    }

    // ============ Чтение токенов ============

    fn read_identifier(&mut self) {
        let start = self.current_span();
        let mut name = String::new();

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            name.push(self.peek());
            self.advance();
        }

        let kind = match name.as_str() {
            "функция" => TokenKind::Функция,
            "вернуть" => TokenKind::Вернуть,
            "если" => TokenKind::Если,
            "иначе" => TokenKind::Иначе,
            "сопоставить" => TokenKind::Сопоставить,
            "структура" => TokenKind::Структура,
            "тип" => TokenKind::Тип,
            "изм" => TokenKind::Изм,
            "внутри" => TokenKind::Внутри,
            "вместе" => TokenKind::Вместе,
            "макрос" => TokenKind::Макрос,
            "открыто" => TokenKind::Открыто,
            "импорт" => TokenKind::Импорт,
            "модуль" => TokenKind::Модуль,
            "ручной" => TokenKind::Ручной,
            "цитировать" => TokenKind::Цитировать,
            "вставить" => TokenKind::Вставить,
            "для" => TokenKind::Для,
            "каждого" => TokenKind::Каждого,
            "из" => TokenKind::Из,
            "пока" => TokenKind::Пока,
            "где" => TokenKind::Где,
            "Истина" => TokenKind::Истина,
            "Ложь" => TokenKind::Ложь,
            "Ничего" => TokenKind::Ничего,
            "Значение" => TokenKind::Значение,
            "Провал" => TokenKind::Провал,
            "Успех" => TokenKind::Успех,
            _ => TokenKind::Идентификатор(name.clone()),
        };

        self.push_token_at(kind, &name, start);
    }

    fn read_number(&mut self) {
        let start = self.current_span();
        let mut num = String::new();
        let mut is_float = false;

        if self.peek() == '-' {
            num.push('-');
            self.advance();
        }

        while !self.is_at_end() && self.peek().is_ascii_digit() {
            num.push(self.peek());
            self.advance();
        }

        if self.peek() == '.' && self.peek_next().is_ascii_digit() {
            is_float = true;
            num.push('.');
            self.advance();
            while !self.is_at_end() && self.peek().is_ascii_digit() {
                num.push(self.peek());
                self.advance();
            }
        }

        let kind = if is_float {
            TokenKind::Десятичное(num.parse().unwrap_or(0.0))
        } else {
            TokenKind::Целое(num.parse().unwrap_or(0))
        };

        self.push_token_at(kind, &num, start);
    }

    fn read_string(&mut self) {
        let start = self.current_span();
        self.advance(); // пропускаем открывающую кавычку
        let mut s = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\\' {
                self.advance();
                match self.peek() {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    c => {
                        s.push('\\');
                        s.push(c);
                    }
                }
            } else {
                s.push(self.peek());
            }
            self.advance();
        }

        if self.peek() == '"' {
            self.advance(); // закрывающая кавычка
        } else {
            self.error_at("Незакрытая строка", start);
        }

        self.push_token_at(TokenKind::Строка(s.clone()), &format!("\"{}\"", s), start);
    }

    fn skip_line_comment(&mut self) {
        let start = self.current_span();
        self.advance(); // /
        self.advance(); // /
        let mut comment = String::new();

        while !self.is_at_end() && self.peek() != '\n' {
            comment.push(self.peek());
            self.advance();
        }

        self.push_token_at(TokenKind::Комментарий(comment.clone()), &format!("//{}", comment), start);
    }

    fn skip_block_comment(&mut self) {
        let start = self.current_span();
        self.advance(); // /
        self.advance(); // *
        let mut depth = 1;
        let mut comment = String::new();

        while !self.is_at_end() && depth > 0 {
            if self.peek() == '*' && self.peek_next() == '/' {
                depth -= 1;
                self.advance();
                self.advance();
            } else if self.peek() == '/' && self.peek_next() == '*' {
                depth += 1;
                comment.push('/');
                comment.push('*');
                self.advance();
                self.advance();
            } else {
                comment.push(self.peek());
                self.advance();
            }
        }

        if depth > 0 {
            self.error_at("Незакрытый блочный комментарий", start);
        }

        self.push_token_at(TokenKind::Комментарий(comment), "", start);
    }

    // ============ Вспомогательные методы ============

    fn peek(&self) -> char {
        self.source.get(self.pos).copied().unwrap_or('\0')
    }

    fn peek_next(&self) -> char {
        self.source.get(self.pos + 1).copied().unwrap_or('\0')
    }

    fn peek_n(&self, n: usize) -> char {
        self.source.get(self.pos + n).copied().unwrap_or('\0')
    }

    fn advance(&mut self) -> char {
        let c = self.peek();
        self.pos += 1;
        self.column += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn current_span(&self) -> Span {
        Span {
            line: self.line,
            column: self.column,
            offset: self.pos,
        }
    }

    fn single_token(&mut self, kind: TokenKind) {
        let start = self.current_span();
        let c = self.advance();
        self.push_token_at(kind, &c.to_string(), start);
    }

    fn double_token(&mut self, kind: TokenKind) {
        let start = self.current_span();
        let c1 = self.advance();
        let c2 = self.advance();
        self.push_token_at(kind, &format!("{}{}", c1, c2), start);
    }

    fn triple_token(&mut self, kind: TokenKind) {
        let start = self.current_span();
        let c1 = self.advance();
        let c2 = self.advance();
        let c3 = self.advance();
        self.push_token_at(kind, &format!("{}{}{}", c1, c2, c3), start);
    }

    fn push_token(&mut self, kind: TokenKind, lexeme: &str) {
        self.tokens.push(Token::new(kind, lexeme, self.current_span()));
    }

    fn push_token_at(&mut self, kind: TokenKind, lexeme: &str, span: Span) {
        self.tokens.push(Token::new(kind, lexeme, span));
    }

    fn error(&mut self, message: &str) {
        self.errors.push(Diagnostic {
            kind: DiagnosticKind::Ошибка,
            message: message.to_string(),
            span: self.current_span(),
            hint: None,
        });
    }

    fn error_at(&mut self, message: &str, span: Span) {
        self.errors.push(Diagnostic {
            kind: DiagnosticKind::Ошибка,
            message: message.to_string(),
            span,
            hint: None,
        });
    }
}
