// grammalang-core/src/lexer.rs

use crate::error::{Diagnostic, DiagnosticKind};
use crate::token::{Span, Token, TokenKind};

/// Lexer — transforms source text into a stream of tokens
pub struct Lexer {
    source: Vec<char>,
    pos: usize,
    line: usize,
    column: usize,
    tokens: Vec<Token>,
    errors: Vec<Diagnostic>,
    /// Stack of indentation levels
    indent_stack: Vec<usize>,
    /// Start of current line (for indentation calculation)
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

    /// Main method — tokenize the entire source text
    pub fn tokenize(&mut self) -> (Vec<Token>, Vec<Diagnostic>) {
        while !self.is_at_end() {
            let c = self.peek();

            match c {
                // Whitespace and indentation
                ' ' if self.line_start => self.handle_indent(),
                ' ' | '\t' | '\r' => {
                    if c == '\t' && self.line_start {
                        self.error("Tabs are forbidden. Use spaces for indentation.");
                    }
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    self.line += 1;
                    self.column = 1;
                    self.line_start = true;
                }

                // Comments
                '/' if self.peek_next() == '/' => self.skip_line_comment(),
                '/' if self.peek_next() == '*' => self.skip_block_comment(),

                // Strings and chars
                '"' => self.read_string(),
                '\'' => self.read_char(),

                // Numbers
                '0'..='9' => self.read_number(),
                '-' if self.peek_next().is_ascii_digit() => self.read_number(),

                // Identifiers and keywords
                'a'..='z' | 'A'..='Z' | '_' => {
                    self.read_identifier()
                }

                // Operators and delimiters
                '+' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::PlusEq)
                    } else {
                        self.single_token(TokenKind::Plus)
                    }
                }
                '*' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::StarEq)
                    } else {
                        self.single_token(TokenKind::Star)
                    }
                }
                '%' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::PercentEq)
                    } else {
                        self.single_token(TokenKind::Percent)
                    }
                }
                '^' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::CaretEq)
                    } else {
                        self.single_token(TokenKind::Caret)
                    }
                }
                '!' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::NotEq)
                    } else {
                        self.single_token(TokenKind::Bang)
                    }
                }
                '(' => self.single_token(TokenKind::LParen),
                ')' => self.single_token(TokenKind::RParen),
                '{' => self.single_token(TokenKind::LBrace),
                '}' => self.single_token(TokenKind::RBrace),
                '[' => self.single_token(TokenKind::LBracket),
                ']' => self.single_token(TokenKind::RBracket),
                ',' => self.single_token(TokenKind::Comma),
                ':' => self.single_token(TokenKind::Colon),
                ';' => self.single_token(TokenKind::Semicolon),
                '?' => self.single_token(TokenKind::Question),
                '@' => self.single_token(TokenKind::At),

                '=' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::EqEq)
                    } else {
                        self.single_token(TokenKind::Eq)
                    }
                }
                '<' => {
                    if self.peek_next() == '<' && self.peek_n(2) == 'e' && self.peek_n(3) == 'x' 
                        && self.peek_n(4) == 'e' && self.peek_n(5) == 'c' && self.peek_n(6) == 'u' 
                        && self.peek_n(7) == 't' && self.peek_n(8) == 'e' && self.peek_n(9) == '>' 
                        && self.peek_n(10) == '>' {
                        // <<execute>>
                        for _ in 0..11 { self.advance(); }
                        self.push_token(TokenKind::ExecuteOp, "<<execute>>");
                    } else if self.peek_next() == '<' && self.peek_n(2) == '+' && self.peek_n(3) == '>' && self.peek_n(4) == '>' {
                        // <<+>>
                        for _ in 0..5 { self.advance(); }
                        self.push_token(TokenKind::AufhebenOp, "<<+>>");
                    } else if self.peek_next() == '=' {
                        self.double_token(TokenKind::Le)
                    } else if self.peek_next() == '<' {
                        if self.peek_n(2) == '=' {
                            self.triple_token(TokenKind::ShlEq)
                        } else {
                            self.double_token(TokenKind::Shl)
                        }
                    } else {
                        self.single_token(TokenKind::Lt)
                    }
                }
                
                '-' => {
                    if self.peek_next() == '>' {
                        self.double_token(TokenKind::Arrow)
                    } else if self.peek_next() == '=' {
                        self.double_token(TokenKind::MinusEq)
                    } else if self.peek_next().is_ascii_digit() {
                        self.read_number()
                    } else {
                        self.single_token(TokenKind::Minus)
                    }
                }
                '|' => {
                    if self.peek_next() == '>' {
                        self.double_token(TokenKind::Pipeline)
                    } else if self.peek_next() == '=' {
                        self.double_token(TokenKind::PipeEq)
                    } else {
                        self.single_token(TokenKind::Pipe)
                    }
                }
                '&' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::AmpersandEq)
                    } else {
                        self.single_token(TokenKind::Ampersand)
                    }
                }
                '.' => {
                    if self.peek_next() == '.' && self.peek_n(2) == '=' {
                        self.triple_token(TokenKind::DotDotEq)
                    } else if self.peek_next() == '.' && self.peek_n(2) == '.' {
                        self.triple_token(TokenKind::Ellipsis)
                    } else if self.peek_next() == '.' {
                        self.double_token(TokenKind::DotDot)
                    } else {
                        self.single_token(TokenKind::Dot)
                    }
                }
                '~' => {
                    if self.peek_next() == ':' && self.peek_n(2) == '~' {
                        self.triple_token(TokenKind::AporeticOp)
                    } else {
                        self.error(&format!("Unexpected character: '{}'", c));
                        self.advance();
                    }
                }
                '>' => {
                    if self.peek_next() == '=' {
                        self.double_token(TokenKind::Ge)
                    } else if self.peek_next() == '>' {
                        if self.peek_n(2) == '=' {
                            self.triple_token(TokenKind::ShrEq)
                        } else {
                            self.double_token(TokenKind::Shr)
                        }
                    } else {
                        self.single_token(TokenKind::Gt)
                    }
                }
                '_' => self.single_token(TokenKind::Underscore),

                _ => {
                    self.error(&format!("Unexpected character: '{}'", c));
                    self.advance();
                }
            }
        }

        // At end of file — close all remaining indentation levels
        while self.indent_stack.len() > 1 {
            self.indent_stack.pop();
            self.push_token(TokenKind::Dedent, "");
        }

        self.push_token(TokenKind::Eof, "");
        (std::mem::take(&mut self.tokens), std::mem::take(&mut self.errors))
    }

    // ============ Indentation handling ============

    fn handle_indent(&mut self) {
        let mut spaces = 0;
        let start = self.current_span();

        while self.peek() == ' ' {
            self.advance();
            spaces += 1;
        }

        // Empty line — ignore
        if self.peek() == '\n' || self.peek() == '\r' {
            return;
        }

        let current = *self.indent_stack.last().unwrap_or(&0);

        if spaces > current {
            self.indent_stack.push(spaces);
            self.push_token_at(TokenKind::Indent, "", start);
        } else if spaces < current {
            while *self.indent_stack.last().unwrap() > spaces {
                self.indent_stack.pop();
                self.push_token_at(TokenKind::Dedent, "", start);
            }
            if *self.indent_stack.last().unwrap() != spaces {
                self.error_at("Invalid indentation. Expected level multiple of 4 spaces.", start);
            }
        }

        self.line_start = false;
    }

    // ============ Token readers ============

    fn read_identifier(&mut self) {
        let start = self.current_span();
        let mut name = String::new();

        while !self.is_at_end() && (self.peek().is_alphanumeric() || self.peek() == '_') {
            name.push(self.peek());
            self.advance();
        }

        let kind = match name.as_str() {
            "fn" => TokenKind::Fn,
            "return" => TokenKind::Return,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "match" => TokenKind::Match,
            "struct" => TokenKind::Struct,
            "type" => TokenKind::Type,
            "mut" => TokenKind::Mut,
            "effect" => TokenKind::Effect,
            "together" => TokenKind::Together,
            "macro" => TokenKind::Macro,
            "public" => TokenKind::Public,
            "import" => TokenKind::Import,
            "module" => TokenKind::Module,
            "unsafe" => TokenKind::Unsafe,
            "quote" => TokenKind::Quote,
            "splice" => TokenKind::Splice,
            "for" => TokenKind::For,
            "each" => TokenKind::Each,
            "from" => TokenKind::From,
            "while" => TokenKind::While,
            "where" => TokenKind::Where,
            "True" => TokenKind::True,
            "False" => TokenKind::False,
            "Nil" => TokenKind::Nil,
            "Value" => TokenKind::Value,
            "Failure" => TokenKind::Failure,
            "Success" => TokenKind::Success,
            "let" => TokenKind::Let,
            "loop" => TokenKind::Loop,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "as" => TokenKind::As,
            "with" => TokenKind::With,
            _ => TokenKind::Identifier(name.clone()),
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
            TokenKind::Float(num.parse().unwrap_or(0.0))
        } else {
            TokenKind::Int(num.parse().unwrap_or(0))
        };

        self.push_token_at(kind, &num, start);
    }

    fn read_string(&mut self) {
        let start = self.current_span();
        self.advance(); // skip opening quote
        let mut s = String::new();

        while !self.is_at_end() && self.peek() != '"' {
            if self.peek() == '\\' {
                self.advance();
                match self.peek() {
                    'n' => s.push('\n'),
                    't' => s.push('\t'),
                    '\\' => s.push('\\'),
                    '"' => s.push('"'),
                    '\'' => s.push('\''),
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
            self.advance(); // closing quote
        } else {
            self.error_at("Unclosed string", start);
        }

        self.push_token_at(TokenKind::String(s.clone()), &format!("\"{}\"", s), start);
    }

    fn read_char(&mut self) {
        let start = self.current_span();
        self.advance(); // skip opening quote '
        let mut ch = '\0';

        if self.is_at_end() {
            self.error_at("Unclosed char literal", start);
            return;
        }

        if self.peek() == '\\' {
            self.advance();
            match self.peek() {
                'n' => ch = '\n',
                't' => ch = '\t',
                '\\' => ch = '\\',
                '\'' => ch = '\'',
                '"' => ch = '"',
                '0' => ch = '\0',
                c => {
                    self.error_at(&format!("Unknown escape sequence: \\{}", c), start);
                    ch = c;
                }
            }
        } else {
            ch = self.peek();
        }
        self.advance();

        if self.peek() == '\'' {
            self.advance(); // closing quote
        } else {
            self.error_at("Expected closing quote '", start);
        }

        self.push_token_at(TokenKind::Char(ch), &format!("'{}'", ch), start);
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

        self.push_token_at(TokenKind::Comment(comment.clone()), &format!("//{}", comment), start);
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
            self.error_at("Unclosed block comment", start);
        }

        self.push_token_at(TokenKind::Comment(comment), "", start);
    }

    // ============ Helper methods ============

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
            kind: DiagnosticKind::Error,
            message: message.to_string(),
            span: self.current_span(),
            hint: None,
        });
    }

    fn error_at(&mut self, message: &str, span: Span) {
        self.errors.push(Diagnostic {
            kind: DiagnosticKind::Error,
            message: message.to_string(),
            span,
            hint: None,
        });
    }
}
