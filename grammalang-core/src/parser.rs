// grammalang-core/src/parser.rs

use crate::error::{Diagnostic, DiagnosticKind};
use crate::token::{Token, TokenKind};

#[derive(Debug, Clone)]
pub enum CstNode {
    Module { declarations: Vec<CstNode>, span: (usize, usize) },
    Fn { name: String, params: Vec<Param>, return_type: Option<Box<CstNode>>, body: Box<CstNode>, modifiers: Vec<String>, span: (usize, usize) },
    Param { name: String, llvm_type: Box<CstNode>, mutable: bool },
    Block { expressions: Vec<CstNode>, span: (usize, usize) },
    BinExpr { left: Box<CstNode>, operator: String, right: Box<CstNode> },
    UnaryExpr { operator: String, operand: Box<CstNode> },
    Call { function: Box<CstNode>, arguments: Vec<CstNode> },
    Pipeline { left: Box<CstNode>, right: Box<CstNode> },
    ReflexiveCascade { subject: Box<CstNode>, ethics_override: Option<String>, context: Box<CstNode> },
    AufhebenBinding { left: Box<CstNode>, right: Box<CstNode> },
    ExecuteBinding { schema: Box<CstNode>, args: Vec<CstNode> },
    AporeticBinding { left: Box<CstNode>, right: Box<CstNode> },
    FieldAccess { object: Box<CstNode>, field: String },
    Match { value: Box<CstNode>, arms: Vec<Arm> },
    Arm { pattern: Box<CstNode>, condition: Option<Box<CstNode>>, body: Box<CstNode> },
    StructCons { name: String, fields: Vec<(String, CstNode)> },
    SumCons { name: String, value: Option<Box<CstNode>> },
    StructDecl { name: String, fields: Vec<(String, CstNode)> },
    SumDecl { name: String, variants: Vec<(String, Option<CstNode>)> },
    ImportDecl { path: Vec<String>, names: Vec<String> },
    If { condition: Box<CstNode>, then: Box<CstNode>, else_arm: Option<Box<CstNode>> },
    Return(Option<Box<CstNode>>),
    Assign { name: String, mutable: bool, value: Box<CstNode> },
    Borrow { mutable: bool, value: Box<CstNode> },
    EffectBlock { effects: Vec<String>, body: Box<CstNode> },
    ParallelBlock { body: Box<CstNode> },
    PatternVariable(String),
    PatternWildcard,
    PatternLiteral(String),
    PatternConstructor { name: String, nested: Option<Box<CstNode>> },
    PatternOr(Box<CstNode>, Box<CstNode>),
    PatternBinding { name: String, pattern: Box<CstNode> },
    PatternStruct { name: String, fields: Vec<(String, CstNode)>, open: bool },
    PatternList { elements: Vec<CstNode>, tail: Option<Box<CstNode>> },
    TypeName(String),
    TypeParameterized { name: String, params: Vec<CstNode> },
    TypeFn { arguments: Vec<CstNode>, result: Box<CstNode> },
    TypeRecord { fields: Vec<(String, CstNode)> },
    TypeRef { mutable: bool, llvm_type: Box<CstNode> },
    Variable(String),
    Literal(TokenKind),
}

#[derive(Debug, Clone)]
pub struct Param { pub name: String, pub llvm_type: CstNode, pub mutable: bool }

#[derive(Debug, Clone)]
pub struct Arm { pub pattern: Box<CstNode>, pub condition: Option<Box<CstNode>>, pub body: Box<CstNode> }

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
        Some(CstNode::Module { declarations, span: (start, self.pos) })
    }

    fn skip_insignificant(&mut self) {
        while self.check(&TokenKind::Indent) || self.check(&TokenKind::Dedent) {
            self.advance();
        }
    }

    fn parse_declaration(&mut self) -> Option<CstNode> {
        self.skip_insignificant();
        if self.check(&TokenKind::Public) || self.check(&TokenKind::Fn) { return self.parse_function(); }
        if self.check(&TokenKind::Struct) { return self.parse_struct_declaration(); }
        if self.check(&TokenKind::Type) { return self.parse_sum_declaration(); }
        if self.check(&TokenKind::Import) { return self.parse_import(); }
        if self.check(&TokenKind::Module) { return self.parse_module_declaration(); }
        None
    }

    fn parse_function(&mut self) -> Option<CstNode> {
        let start = self.pos;
        let mut modifiers = Vec::new();
        self.skip_insignificant();
        if self.eat(&TokenKind::Public) { modifiers.push("public".to_string()); self.skip_insignificant(); }
        self.expect(&TokenKind::Fn)?;
        self.skip_insignificant();
        let name = self.expect_identifier()?;
        
        let _type_params = if self.eat(&TokenKind::Lt) {
            let mut params = Vec::new();
            loop {
                self.skip_insignificant();
                params.push(self.expect_identifier()?);
                self.skip_insignificant();
                if !self.eat(&TokenKind::Comma) { break; }
            }
            self.expect(&TokenKind::Gt)?;
            params
        } else {
            Vec::new()
        };
        
        self.skip_insignificant();
        let parameters = self.parse_parameters()?;
        self.skip_insignificant();
        let return_type = if self.eat(&TokenKind::Arrow) { self.skip_insignificant(); self.parse_type().map(Box::new) } else { None };
        self.skip_insignificant();
        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;
        Some(CstNode::Fn { name, params: parameters, return_type, body: Box::new(body), modifiers, span: (start, self.pos) })
    }

    fn parse_parameters(&mut self) -> Option<Vec<Param>> {
        self.expect(&TokenKind::LParen)?;
        let mut params = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                let mutable = self.eat(&TokenKind::Mut);
                let name = self.expect_identifier()?;
                self.expect(&TokenKind::Colon)?;
                let typ = self.parse_type()?;
                params.push(Param { name, llvm_type: typ, mutable });
                if !self.eat(&TokenKind::Comma) { break; }
            }
        }
        self.expect(&TokenKind::RParen)?;
        Some(params)
    }

    fn parse_struct_declaration(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Struct)?;
        self.skip_insignificant();
        let name = self.expect_identifier()?;
        self.skip_insignificant();
        self.expect(&TokenKind::Colon)?;
        let mut fields = Vec::new();
        while !self.is_at_end()
            && !self.check(&TokenKind::Fn)
            && !self.check(&TokenKind::Struct)
            && !self.check(&TokenKind::Type)
            && !self.check(&TokenKind::Public)
            && !self.check(&TokenKind::Import)
            && !self.check(&TokenKind::Eof)
            && !self.check(&TokenKind::Dedent)
        {
            if let Some(ident) = self.eat_identifier() {
                self.expect(&TokenKind::Colon)?;
                let typ = self.parse_type()?;
                fields.push((ident, typ));
            } else {
                self.advance();
            }
        }
        Some(CstNode::StructDecl { name, fields })
    }

    fn parse_sum_declaration(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Type)?;
        self.skip_insignificant();
        let name = self.expect_identifier()?;
        
        let _type_params = if self.eat(&TokenKind::Lt) {
            let mut params = Vec::new();
            loop {
                self.skip_insignificant();
                params.push(self.expect_identifier()?);
                self.skip_insignificant();
                if !self.eat(&TokenKind::Comma) { break; }
            }
            self.expect(&TokenKind::Gt)?;
            params
        } else {
            Vec::new()
        };
        
        self.skip_insignificant();
        self.expect(&TokenKind::Eq)?;
        self.skip_insignificant();
        let mut variants = Vec::new();
        loop {
            let variant_name = self.expect_identifier()?;
            let data = if self.eat(&TokenKind::LParen) {
                let typ = self.parse_type();
                self.expect(&TokenKind::RParen)?;
                typ
            } else {
                None
            };
            variants.push((variant_name, data));
            if !self.eat(&TokenKind::Pipe) { break; }
        }
        Some(CstNode::SumDecl { name, variants })
    }

    fn parse_import(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Import)?;
        let mut path = Vec::new();
        let mut names = Vec::new();
        path.push(self.expect_identifier()?);
        while self.eat(&TokenKind::Dot) {
            if self.eat(&TokenKind::Star) {
                names.push("*".to_string());
                break;
            }
            path.push(self.expect_identifier()?);
        }
        Some(CstNode::ImportDecl { path, names })
    }

    fn parse_module_declaration(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Module)?;
        self.skip_insignificant();
        let _name = self.expect_identifier()?;
        self.skip_insignificant();
        self.expect(&TokenKind::Colon)?;
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
        Some(CstNode::Module { declarations, span: (0, self.pos) })
    }

    fn parse_expression(&mut self) -> Option<CstNode> {
        if self.check(&TokenKind::Let) { return self.parse_let(); }
        self.parse_assignment()
    }

    fn parse_let(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Let)?;
        let name = self.expect_identifier()?;
        self.expect(&TokenKind::Eq)?;
        let value = Box::new(self.parse_expression()?);
        Some(CstNode::Assign { name, mutable: false, value })
    }

    fn parse_assignment(&mut self) -> Option<CstNode> {
        let left = self.parse_pipeline()?;
        self.skip_insignificant();
        if self.check(&TokenKind::Eq) {
            self.advance();
            self.skip_insignificant();
            let right = self.parse_assignment()?;
            if let CstNode::Variable(name) = left {
                return Some(CstNode::Assign { name, mutable: false, value: Box::new(right) });
            }
        }
        Some(left)
    }

    fn parse_pipeline(&mut self) -> Option<CstNode> {
        let mut left = self.parse_reflexive()?;
        while self.eat(&TokenKind::Pipeline) {
            let right = self.parse_reflexive()?;
            left = CstNode::Pipeline { left: Box::new(left), right: Box::new(right) };
        }
        Some(left)
    }

    fn parse_reflexive(&mut self) -> Option<CstNode> {
        let left = self.parse_or()?;
        
        // AporeticBinding: left ~::~ right
        if self.eat(&TokenKind::AporeticOp) {
            let right = self.parse_reflexive()?;
            return Some(CstNode::AporeticBinding {
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        // AufhebenBinding: left <<+>> right
        if self.eat(&TokenKind::AufhebenOp) {
            let right = self.parse_reflexive()?;
            return Some(CstNode::AufhebenBinding {
                left: Box::new(left),
                right: Box::new(right),
            });
        }
         // ExecuteBinding: schema <<execute>> (args)
        if self.eat(&TokenKind::ExecuteOp) {
            let args = if self.check(&TokenKind::LParen) {
                self.parse_arguments()?
            } else {
                Vec::new()
            };
            return Some(CstNode::ExecuteBinding {
                schema: Box::new(left),
                args,
            });
        }
        
        if self.eat(&TokenKind::ColonColonColon) {
            // Check for ethics override: Identifier ::: EthicsName ::: context
            let ethics_override = self.try_parse_ethics_override();
            
            if ethics_override.is_some() {
                self.eat(&TokenKind::ColonColonColon); // consume second :::
            }

            let right = self.parse_reflexive()?;
            return Some(CstNode::ReflexiveCascade {
                subject: Box::new(left),
                ethics_override,
                context: Box::new(right),
            });
        }
        Some(left)
    }

    /// Attempts to parse an ethics override after the first :::.
    /// Returns Some(name) if the pattern "UppercaseId :::" is found,
    /// otherwise None (and leaves the token stream unchanged).
    fn try_parse_ethics_override(&mut self) -> Option<String> {
        let saved_pos = self.pos;
        
        // Need an uppercase identifier
        let name = match self.peek() {
            Some(t) if matches!(&t.kind, TokenKind::Identifier(n) if n.chars().next().map_or(false, |c| c.is_uppercase())) => {
                let n = match &t.kind {
                    TokenKind::Identifier(name) => name.clone(),
                    _ => return None,
                };
                self.advance();
                n
            }
            _ => return None,
        };
        
        // Followed by :::
        if self.check(&TokenKind::ColonColonColon) {
            Some(name)
        } else {
            // Not an override, rewind
            self.pos = saved_pos;
            None
        }
    }

    fn parse_or(&mut self) -> Option<CstNode> {
        let mut left = self.parse_and()?;
        while self.eat_identifier_is("or") {
            let right = self.parse_and()?;
            left = CstNode::BinExpr { left: Box::new(left), operator: "or".to_string(), right: Box::new(right) };
        }
        Some(left)
    }

    fn parse_and(&mut self) -> Option<CstNode> {
        let mut left = self.parse_comparison()?;
        while self.eat_identifier_is("and") {
            let right = self.parse_comparison()?;
            left = CstNode::BinExpr { left: Box::new(left), operator: "and".to_string(), right: Box::new(right) };
        }
        Some(left)
    }

    fn parse_comparison(&mut self) -> Option<CstNode> {
        let mut left = self.parse_addition()?;
        let ops = [TokenKind::EqEq, TokenKind::NotEq, TokenKind::Lt, TokenKind::Gt, TokenKind::Le, TokenKind::Ge];
        if self.check_any(&ops) {
            let op_str = format!("{:?}", self.advance().kind);
            let right = self.parse_addition()?;
            left = CstNode::BinExpr { left: Box::new(left), operator: op_str, right: Box::new(right) };
        }
        Some(left)
    }

    fn parse_addition(&mut self) -> Option<CstNode> {
        let mut left = self.parse_multiplication()?;
        self.skip_insignificant();
        while self.check(&TokenKind::Plus) || self.check(&TokenKind::Minus) {
            let op = self.advance().lexeme.clone();
            self.skip_insignificant();
            let right = self.parse_multiplication()?;
            left = CstNode::BinExpr { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        Some(left)
    }

    fn parse_multiplication(&mut self) -> Option<CstNode> {
        let mut left = self.parse_unary()?;
        self.skip_insignificant();
        while self.check(&TokenKind::Star) || self.check(&TokenKind::Slash) || self.check(&TokenKind::Percent) {
            let op = self.advance().lexeme.clone();
            self.skip_insignificant();
            let right = self.parse_unary()?;
            left = CstNode::BinExpr { left: Box::new(left), operator: op, right: Box::new(right) };
        }
        Some(left)
    }

    fn parse_unary(&mut self) -> Option<CstNode> {
        if self.eat_identifier_is("not") || self.check(&TokenKind::Minus) {
            let op = self.advance().lexeme.clone();
            let operand = self.parse_unary()?;
            return Some(CstNode::UnaryExpr { operator: op, operand: Box::new(operand) });
        }
        self.parse_postfix()
    }

    fn parse_postfix(&mut self) -> Option<CstNode> {
        let mut expr = self.parse_primary()?;
        loop {
            self.skip_insignificant();
            if self.eat(&TokenKind::Dot) {
                let field = self.expect_identifier()?;
                expr = CstNode::FieldAccess { object: Box::new(expr), field };
            }
            else if self.check(&TokenKind::LParen) {
                let args = self.parse_arguments()?;
                expr = CstNode::Call { function: Box::new(expr), arguments: args };
            }
            else if self.eat(&TokenKind::Question) {
                expr = CstNode::UnaryExpr { operator: "?".to_string(), operand: Box::new(expr) };
            }
            else { break; }
        }
        Some(expr)
    }

    fn parse_primary(&mut self) -> Option<CstNode> {
        let token = self.peek()?.clone();
        match &token.kind {
            TokenKind::Int(_) | TokenKind::Float(_) | TokenKind::String(_) => {
                self.advance();
                Some(CstNode::Literal(token.kind.clone()))
            }
            TokenKind::True | TokenKind::False | TokenKind::Nil => {
                self.advance();
                Some(CstNode::Literal(token.kind.clone()))
            }
            TokenKind::Identifier(_) => {
                let name = token.lexeme.clone();
                let is_uppercase = name.chars().next().map_or(false, |c| c.is_uppercase());
                self.advance();
                self.skip_insignificant();
                
                if self.check(&TokenKind::LParen) {
                    let args = self.parse_arguments()?;
                    if is_uppercase {
                        if args.len() == 1 {
                            return Some(CstNode::SumCons { name, value: Some(Box::new(args.into_iter().next().unwrap())) });
                        } else {
                            return Some(CstNode::SumCons { name, value: None });
                        }
                    } else {
                        return Some(CstNode::Call {
                            function: Box::new(CstNode::Variable(name)),
                            arguments: args,
                        });
                    }
                }
                
                if self.check(&TokenKind::LBrace) && is_uppercase {
                    return self.parse_struct_init(&name);
                }
                
                Some(CstNode::Variable(name))
            }
            TokenKind::LBrace => self.parse_block(),
            TokenKind::If => self.parse_if(),
            TokenKind::Match => self.parse_match(),
            TokenKind::Return => self.parse_return(),
            TokenKind::Effect => self.parse_inside_effect(),
            TokenKind::Together => self.parse_together(),
            TokenKind::Ampersand => self.parse_borrow(),
            TokenKind::LParen => {
                self.advance();
                let expr = self.parse_expression();
                self.expect(&TokenKind::RParen)?;
                expr
            }
            _ => {
                self.error(&format!("Unexpected token: {}", token));
                self.advance();
                None
            }
        }
    }

    fn parse_struct_init(&mut self, name: &str) -> Option<CstNode> {
        self.expect(&TokenKind::LBrace)?;
        let mut fields = Vec::new();
        if !self.check(&TokenKind::RBrace) {
            loop {
                let field_name = self.expect_identifier()?;
                let field_value = if self.eat(&TokenKind::Colon) {
                    self.parse_expression()?
                } else {
                    CstNode::Variable(field_name.clone())
                };
                fields.push((field_name, field_value));
                if !self.eat(&TokenKind::Comma) { break; }
            }
        }
        self.expect(&TokenKind::RBrace)?;
        Some(CstNode::StructCons { name: name.to_string(), fields })
    }

    fn parse_arguments(&mut self) -> Option<Vec<CstNode>> {
        self.expect(&TokenKind::LParen)?;
        let mut args = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                if let Some(expr) = self.parse_expression() { args.push(expr); }
                if !self.eat(&TokenKind::Comma) { break; }
            }
        }
        self.expect(&TokenKind::RParen)?;
        Some(args)
    }

    fn parse_block(&mut self) -> Option<CstNode> {
        let start = self.pos;
        let mut expressions = Vec::new();
        if self.eat(&TokenKind::LBrace) {
            while !self.check(&TokenKind::RBrace) && !self.is_at_end() {
                if let Some(expr) = self.parse_expression() { expressions.push(expr); }
            }
            self.expect(&TokenKind::RBrace)?;
        } else if self.eat(&TokenKind::Indent) {
            loop {
                if self.is_at_end() { break; }
                if self.check(&TokenKind::Dedent) { self.advance(); break; }
                if self.check(&TokenKind::Indent) { self.advance(); continue; }
                if let Some(expr) = self.parse_expression() { expressions.push(expr); }
                else { self.advance(); }
            }
        } else {
            if let Some(expr) = self.parse_expression() { expressions.push(expr); }
        }
        Some(CstNode::Block { expressions, span: (start, self.pos) })
    }

    fn parse_if(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::If)?;
        let condition = self.parse_expression()?;
        self.expect(&TokenKind::Colon)?;
        let then_branch = self.parse_block()?;
        let else_branch = if self.eat(&TokenKind::Else) {
            if self.check(&TokenKind::If) {
                Some(Box::new(self.parse_if()?))
            } else {
                self.expect(&TokenKind::Colon)?;
                Some(Box::new(self.parse_block()?))
            }
        } else {
            None
        };
        Some(CstNode::If { condition: Box::new(condition), then: Box::new(then_branch), else_arm: else_branch })
    }

    fn parse_match(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Match)?;
        let value = self.parse_expression()?;
        self.expect(&TokenKind::Colon)?;
        self.skip_insignificant();
        let mut branches = Vec::new();
        loop {
            self.skip_insignificant();
            if self.is_at_end() { break; }
            if self.check(&TokenKind::Dedent) { break; }
            if !self.can_start_pattern() { break; }
            
            let pattern = self.parse_pattern()?;
            let guard = if self.eat_identifier_is("if") {
                Some(Box::new(self.parse_expression()?))
            } else {
                None
            };
            self.expect(&TokenKind::Arrow)?;
            self.skip_insignificant();
            let body = self.parse_expression()?;
            branches.push(Arm {
                pattern: Box::new(pattern),
                condition: guard,
                body: Box::new(body),
            });
        }
        while !self.is_at_end() && !self.check(&TokenKind::Dedent) {
            self.advance();
        }
        Some(CstNode::Match { value: Box::new(value), arms: branches })
    }

    fn can_start_pattern(&self) -> bool {
        match self.peek() {
            Some(t) => matches!(&t.kind,
                TokenKind::Identifier(_) |
                TokenKind::Int(_) |
                TokenKind::Float(_) |
                TokenKind::String(_) |
                TokenKind::Underscore |
                TokenKind::Nil |
                TokenKind::True |
                TokenKind::False
            ),
            None => false,
        }
    }

    fn parse_pattern(&mut self) -> Option<CstNode> {
        self.parse_pattern_or()
    }

    fn parse_pattern_or(&mut self) -> Option<CstNode> {
        let mut left = self.parse_pattern_atom()?;
        while self.eat(&TokenKind::Pipe) {
            let right = self.parse_pattern_atom()?;
            left = CstNode::PatternOr(Box::new(left), Box::new(right));
        }
        Some(left)
    }

    fn parse_pattern_atom(&mut self) -> Option<CstNode> {
        if self.eat(&TokenKind::Underscore) { return Some(CstNode::PatternWildcard); }
        if let Some(token) = self.peek() {
            match &token.kind {
                TokenKind::Identifier(name) => {
                    let name = name.clone();
                    let is_uppercase = name.chars().next().map_or(false, |c| c.is_uppercase());
                    self.advance();
                    if self.check(&TokenKind::At) {
                        self.advance();
                        let inner = self.parse_pattern_atom()?;
                        return Some(CstNode::PatternBinding { name, pattern: Box::new(inner) });
                    }
                    if self.check(&TokenKind::LParen) {
                        self.advance();
                        let inner = self.parse_pattern();
                        self.expect(&TokenKind::RParen)?;
                        return Some(CstNode::PatternConstructor { name, nested: inner.map(Box::new) });
                    }
                    if self.check(&TokenKind::LBrace) && is_uppercase {
                        self.advance();
                        let mut fields = Vec::new();
                        let mut open = false;
                        if !self.check(&TokenKind::RBrace) {
                            loop {
                                if self.eat(&TokenKind::Ellipsis) { open = true; break; }
                                let field_name = self.expect_identifier()?;
                                let field_pattern = if self.eat(&TokenKind::Colon) {
                                    self.parse_pattern()?
                                } else {
                                    CstNode::PatternVariable(field_name.clone())
                                };
                                fields.push((field_name, field_pattern));
                                if !self.eat(&TokenKind::Comma) {
                                    if self.eat(&TokenKind::Ellipsis) { open = true; }
                                    break;
                                }
                            }
                        }
                        self.expect(&TokenKind::RBrace)?;
                        return Some(CstNode::PatternStruct { name, fields, open });
                    }
                    if is_uppercase { return Some(CstNode::PatternConstructor { name, nested: None }); }
                    Some(CstNode::PatternVariable(name))
                }
                TokenKind::Int(_) | TokenKind::String(_) => {
                    let lit = token.lexeme.clone();
                    self.advance();
                    Some(CstNode::PatternLiteral(lit))
                }
                TokenKind::Nil => {
                    self.advance();
                    Some(CstNode::PatternConstructor { name: "Nil".to_string(), nested: None })
                }
                _ => {
                    self.error("Expected pattern");
                    None
                }
            }
        } else {
            None
        }
    }

    fn parse_return(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Return)?;
        if self.check(&TokenKind::Dedent) || self.is_at_end() {
            return Some(CstNode::Return(None));
        }
        let expr = self.parse_expression();
        Some(CstNode::Return(expr.map(Box::new)))
    }

    fn parse_inside_effect(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Effect)?;
        let mut effects = Vec::new();
        loop {
            if let Some(name) = self.eat_identifier() { effects.push(name); } else { break; }
            if !self.eat(&TokenKind::Comma) { break; }
        }
        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;
        Some(CstNode::EffectBlock { effects, body: Box::new(body) })
    }

    fn parse_together(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Together)?;
        self.expect(&TokenKind::Colon)?;
        let body = self.parse_block()?;
        Some(CstNode::ParallelBlock { body: Box::new(body) })
    }

    fn parse_borrow(&mut self) -> Option<CstNode> {
        self.expect(&TokenKind::Ampersand)?;
        let mutable = self.eat(&TokenKind::Mut);
        let value = self.parse_primary()?;
        Some(CstNode::Borrow { mutable, value: Box::new(value) })
    }

    fn parse_type(&mut self) -> Option<CstNode> {
        let typ = self.parse_type_primary()?;
        if self.eat(&TokenKind::Arrow) {
            self.skip_insignificant();
            let result = self.parse_type()?;
            return Some(CstNode::TypeFn { arguments: vec![typ], result: Box::new(result) });
        }
        Some(typ)
    }

    fn parse_type_primary(&mut self) -> Option<CstNode> {
        if self.eat(&TokenKind::Ampersand) {
            let mutable = self.eat(&TokenKind::Mut);
            let typ = self.parse_type_primary()?;
            return Some(CstNode::TypeRef { mutable, llvm_type: Box::new(typ) });
        }
        if self.eat(&TokenKind::LBrace) {
            let mut fields = Vec::new();
            if !self.check(&TokenKind::RBrace) {
                loop {
                    let name = self.expect_identifier()?;
                    self.expect(&TokenKind::Colon)?;
                    let typ = self.parse_type()?;
                    fields.push((name, typ));
                    if !self.eat(&TokenKind::Comma) { break; }
                }
            }
            self.expect(&TokenKind::RBrace)?;
            return Some(CstNode::TypeRecord { fields });
        }
        let name = self.expect_identifier()?;
        if self.eat(&TokenKind::Lt) {
            let mut params = Vec::new();
            loop {
                if let Some(t) = self.parse_type() { params.push(t); }
                if !self.eat(&TokenKind::Comma) { break; }
            }
            self.expect(&TokenKind::Gt)?;
            Some(CstNode::TypeParameterized { name, params })
        } else {
            Some(CstNode::TypeName(name))
        }
    }

    fn peek(&self) -> Option<&Token> { self.tokens.get(self.pos) }
    fn advance(&mut self) -> &Token { let token = &self.tokens[self.pos]; self.pos += 1; token }
    fn is_at_end(&self) -> bool {
        self.pos >= self.tokens.len() || matches!(self.peek(), Some(t) if matches!(t.kind, TokenKind::Eof))
    }
    fn check(&self, kind: &TokenKind) -> bool {
        self.peek().map_or(false, |t| std::mem::discriminant(&t.kind) == std::mem::discriminant(kind))
    }
    fn check_any(&self, kinds: &[TokenKind]) -> bool { kinds.iter().any(|k| self.check(k)) }
    fn eat(&mut self, kind: &TokenKind) -> bool {
        if self.check(kind) { self.advance(); true } else { false }
    }
    fn eat_identifier(&mut self) -> Option<String> {
        if let Some(token) = self.peek() {
            if let TokenKind::Identifier(name) = &token.kind {
                let name = name.clone(); self.advance(); return Some(name);
            }
        }
        None
    }
    fn eat_identifier_is(&mut self, text: &str) -> bool {
        if let Some(token) = self.peek() {
            if let TokenKind::Identifier(name) = &token.kind {
                if name == text { self.advance(); return true; }
            }
        }
        false
    }
    fn expect(&mut self, kind: &TokenKind) -> Option<&Token> {
        if self.check(kind) {
            Some(self.advance())
        } else {
            let found = self.peek().map(|t| t.lexeme.clone()).unwrap_or_default();
            self.error(&format!("Expected {:?}, found '{}'", kind, found));
            None
        }
    }
    fn expect_identifier(&mut self) -> Option<String> {
        if let Some(name) = self.eat_identifier() {
            Some(name)
        } else {
            let found = self.peek().map(|t| t.lexeme.clone()).unwrap_or_default();
            self.error(&format!("Expected identifier, found '{}'", found));
            None
        }
    }
    fn error(&mut self, message: &str) {
        let token = self.peek().cloned().unwrap_or(Token {
            kind: TokenKind::Eof,
            lexeme: "".to_string(),
            span: crate::token::Span { line: 0, column: 0, offset: 0 },
        });
        let span = token.span;
        
        let context_start = if self.pos > 5 { self.pos - 5 } else { 0 };
        let context_end = usize::min(self.pos + 5, self.tokens.len());
        let snippet: Vec<String> = self.tokens[context_start..context_end]
            .iter()
            .map(|t| format!("{:?}", t.kind))
            .collect();
        
        eprintln!("--- GRAMMALANG PARSER CRASH DEBUG ---");
        eprintln!("Message: {}", message);
        eprintln!("Lexeme: '{}' at line {}, column {}", token.lexeme, span.line, span.column);
        eprintln!("Token context: {:?}", snippet);
        eprintln!("-------------------------------------");
        
        self.errors.push(Diagnostic {
            kind: DiagnosticKind::Error,
            message: message.to_string(),
            span,
            hint: None,
        });
    }
}
