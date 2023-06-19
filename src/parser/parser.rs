#![allow(unused)]

use std::any::Any;
use anyhow::bail;
use crate::ast::expression::{Boolean, CallExpression, Expression, FunctionExpression, Identifier, IfExpression, InfixExpression, Integer, PrefixExpression, StringExpression};
use crate::ast::precedences::Precedences;
use crate::parser::program::Program;
use crate::ast::statement::{BlockStatement, ExpressionStatement, LetStatement, ReturnStatement, Statement};
use crate::evaluate::environment::Environment;
use crate::lexer::token::Token;
use crate::parser::error::ParseError::UnexpectedToken;
use crate::parser::error::ParseError::*;

pub struct Parser {
    tokens: Vec<Token>,
    pointer_position: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pointer_position: 0 }
    }

    pub fn parse(&mut self) -> anyhow::Result<Program> {
        let mut program = Program::default();

        while !self.out_of_tokens() && !Token::variant_is_equal(self.current_token().unwrap(), &Token::Eof) {
            program.add_statement(self.parse_statement()?);
        }

        Ok(program)
    }

    fn parse_statement(&mut self) -> anyhow::Result<Box<dyn Statement + Send + Sync>> {
        match self.current_token().unwrap() {
            Token::Let => self.parse_let_statement(),
            Token::Return => self.parse_return_statement(),
            _ => self.parse_expression_statement(),
        }
    }

    fn parse_expression_statement(&mut self) -> anyhow::Result<Box<dyn Statement + Send + Sync>> {
        let token = self.current_token().unwrap().clone();

        let expr = self.parse_expression(Precedences::Lowest as u8)?;

        if self.assert_current_token(&Token::Semicolon).is_ok() {
            self.move_pointer();
        }

        Ok(Box::new(ExpressionStatement::new(token, expr)))
    }

    fn parse_let_statement(&mut self) -> anyhow::Result<Box<dyn Statement + Send + Sync>> {
        self.move_pointer();
        let identifier = self.parse_identifier()?;

        self.assert_current_token(&Token::Assign)?;
        self.move_pointer();

        let expression = self.parse_expression(Precedences::Lowest as u8)?;

        self.assert_current_token(&Token::Semicolon)?;
        self.move_pointer();

        Ok(Box::new(LetStatement::new(identifier, expression)))
    }

    fn parse_return_statement(&mut self) -> anyhow::Result<Box<dyn Statement + Send + Sync>> {
        self.move_pointer();
        let expression = self.parse_expression(Precedences::Lowest as u8)?;

        self.assert_current_token(&Token::Semicolon)?;
        self.move_pointer();

        Ok(Box::new(ReturnStatement::new(expression)))
    }

    fn parse_identifier(&mut self) -> anyhow::Result<Identifier> {
        if self.out_of_tokens() { bail!(RanOutOfTokens) }

        if !Token::variant_is_equal(self.current_token().unwrap(), &Token::Ident(String::new())) { bail!(UnexpectedToken(self.current_token().unwrap().clone())) }

        let identifier = Identifier::new(self.current_token().unwrap().value());

        self.move_pointer();

        Ok(identifier)
    }

    fn parse_integer(&mut self) -> anyhow::Result<Integer> {
        if self.out_of_tokens() { bail!(RanOutOfTokens) }

        if self.assert_current_token(&Token::Int("".to_string())).is_err() { bail!(UnexpectedToken(self.current_token().unwrap().clone())) }

        let int = Integer::new(self.current_token().unwrap().value());

        self.move_pointer();

        Ok(int)
    }

    fn parse_string(&mut self) -> anyhow::Result<StringExpression> {
        if self.out_of_tokens() { bail!(RanOutOfTokens) }

        if self.assert_current_token(&Token::String("".to_string())).is_err() { bail!(UnexpectedToken(self.current_token().unwrap().clone())) }

        let string = StringExpression::new(self.current_token().unwrap().value());

        self.move_pointer();

        Ok(string)
    }

    fn parse_expression(&mut self, precedence: u8) -> anyhow::Result<Box<dyn Expression + Send + Sync>> {
        if self.out_of_tokens() { bail!(RanOutOfTokens) }

        let mut left_expr: anyhow::Result<Box<dyn Expression + Send + Sync>> = match self.current_token().unwrap() {
            Token::Ident(_) => self.parse_identifier().map(|x| Box::new(x) as Box<dyn Expression + Send + Sync>),
            Token::Int(val) => self.parse_integer().map(|x| Box::new(x) as Box<dyn Expression + Send + Sync>),
            Token::String(_) => self.parse_string().map(|x| Box::new(x) as Box<dyn Expression + Send + Sync>),
            Token::True => {
                self.move_pointer();
                Ok(Box::new(Boolean::new(true)))
            }
            Token::False => {
                self.move_pointer();
                Ok(Box::new(Boolean::new(false)))
            }
            Token::Minus |
            Token::Bang => self.parse_prefix_expression(),
            Token::LParent => self.parse_grouped_expression(),
            Token::If => self.parse_if_statement_expression(),
            Token::Function => self.parse_function_expression(),
            _ => bail!(UnexpectedToken(self.current_token().unwrap().clone()))
        };


        while !self.out_of_tokens() && !self.out_of_peek_tokens() && !Token::variant_is_equal(self.peek_token().unwrap(), &Token::Semicolon) && precedence < self.current_precedence()? as u8 {
            let infix = match self.current_token().unwrap() {
                Token::Plus |
                Token::Minus |
                Token::Asterisk |
                Token::Slash |
                Token::Equal |
                Token::NotEqual |
                Token::Lt |
                Token::Gt |
                Token::Lte |
                Token::Gte => self.parse_infix_expression(left_expr?),
                Token::LParent => self.parse_call_expression(left_expr?),
                _ => { return left_expr; }
            };

            left_expr = infix;
        }

        left_expr
    }

    fn parse_call_expression(&mut self, function: Box<dyn Expression + Send + Sync>) -> anyhow::Result<Box<dyn Expression + Send + Sync>> {
        let arguments = self.parse_call_arguments()?;

        Ok(Box::new(CallExpression::new(function, arguments)))
    }

    fn parse_call_arguments(&mut self) -> anyhow::Result<Vec<Box<dyn Expression + Send + Sync>>> {
        self.assert_current_token(&Token::LParent)?;
        self.move_pointer();

        if self.assert_current_token(&Token::RParent).is_ok() {
            self.move_pointer();
            return Ok(vec![]);
        }

        let mut expressions = vec![];

        expressions.push(self.parse_expression(Precedences::Lowest as u8)?);

        while self.assert_current_token(&Token::Comma).is_ok() {
            self.move_pointer();
            if self.assert_current_token(&Token::RParent).is_ok() { break; };
            expressions.push(self.parse_expression(Precedences::Lowest as u8)?);
        }

        self.assert_current_token(&Token::RParent)?;
        self.move_pointer();

        Ok(expressions)
    }

    fn parse_grouped_expression(&mut self) -> anyhow::Result<Box<dyn Expression + Send + Sync>> {
        self.move_pointer();

        let expr = self.parse_expression(Precedences::Lowest as u8);

        if self.out_of_tokens() { bail!(RanOutOfTokens) }
        if self.assert_current_token(&Token::RParent).is_err() { bail!(UnexpectedToken(self.current_token().unwrap().clone())) }
        self.move_pointer();

        return expr;
    }

    fn parse_infix_expression(&mut self, left: Box<dyn Expression + Send + Sync>) -> anyhow::Result<Box<dyn Expression + Send + Sync>> {
        let token = self.current_token().unwrap().clone();
        let precedence = self.current_precedence()? as u8;
        self.move_pointer();
        let right = self.parse_expression(precedence)?;
        Ok(Box::new(InfixExpression::new(left, token, right)))
    }

    fn parse_prefix_expression(&mut self) -> anyhow::Result<Box<dyn Expression + Send + Sync>> {
        let prefix = self.current_token().unwrap().clone();
        self.move_pointer();
        let expression = self.parse_expression(Precedences::Prefix as u8)?;
        Ok(Box::new(PrefixExpression::new(prefix, expression)))
    }

    fn parse_if_statement_expression(&mut self) -> anyhow::Result<Box<dyn Expression + Send + Sync>> {
        self.move_pointer();

        self.assert_current_token(&Token::LParent)?;
        self.move_pointer();

        let condition = self.parse_expression(Precedences::Lowest as u8)?;

        self.assert_current_token(&Token::RParent)?;
        self.move_pointer();

        let consequence = self.parse_block_statement()?;

        if self.assert_current_token(&Token::Else).is_err() {
            return Ok(Box::new(IfExpression::new(condition, consequence, None)));
        }
        self.move_pointer();

        let alternatives = self.parse_block_statement()?;

        Ok(Box::new(IfExpression::new(condition, consequence, Some(alternatives))))
    }

    fn parse_block_statement(&mut self) -> anyhow::Result<BlockStatement> {
        self.assert_current_token(&Token::LBrace)?;
        self.move_pointer();

        let mut consequences: Vec<Box<dyn Statement + Send + Sync>> = vec![];

        while !self.out_of_tokens() && self.assert_current_token(&Token::RBrace).is_err() {
            consequences.push(self.parse_statement()?);
        }

        self.assert_current_token(&Token::RBrace)?;
        self.move_pointer();

        Ok(BlockStatement::new(consequences))
    }

    fn parse_function_expression(&mut self) -> anyhow::Result<Box<dyn Expression + Send + Sync>> {
        self.move_pointer();

        let parameters = self.parse_parameters()?;

        let body = self.parse_block_statement()?;

        Ok(Box::new(FunctionExpression::new(parameters, body, Environment::default())))
    }

    fn parse_parameters(&mut self) -> anyhow::Result<Vec<Identifier>> {
        self.assert_current_token(&Token::LParent)?;
        self.move_pointer();

        if self.assert_current_token(&Token::RParent).is_ok() {
            self.move_pointer();
            return Ok(vec![]);
        }

        let mut identifiers = vec![];

        identifiers.push(self.parse_identifier()?);

        while self.assert_current_token(&Token::Comma).is_ok() {
            self.move_pointer();
            if self.assert_current_token(&Token::RParent).is_ok() { break; };
            identifiers.push(self.parse_identifier()?);
        }

        self.assert_current_token(&Token::RParent)?;
        self.move_pointer();

        Ok(identifiers)
    }

    fn assert_current_token(&mut self, token: &Token) -> anyhow::Result<()> {
        if self.out_of_tokens() { bail!(RanOutOfTokens) }
        if !Token::variant_is_equal(self.current_token().unwrap(), token) { bail!(UnexpectedToken(self.current_token().unwrap().clone())) }
        Ok(())
    }

    fn assert_next_token(&mut self, token: &Token) -> anyhow::Result<()> {
        if self.out_of_peek_tokens() { bail!(RanOutOfTokens) }
        if !Token::variant_is_equal(self.peek_token().unwrap(), token) { bail!(UnexpectedToken(self.current_token().unwrap().clone())) }
        Ok(())
    }

    fn peek_precedence(&self) -> anyhow::Result<Precedences> {
        if self.out_of_peek_tokens() { bail!(RanOutOfTokens) }
        Ok(self.peek_token().unwrap().precedence())
    }

    fn current_precedence(&self) -> anyhow::Result<Precedences> {
        if self.out_of_tokens() { bail!(RanOutOfTokens) }
        Ok(self.current_token().unwrap().precedence())
    }

    /// Returns the char on the current pointer position inside the program
    pub fn current_token(&self) -> Option<&Token> {
        self.tokens.get(self.pointer_position)
    }

    /// Returns the char one advanced from the current pointer position inside the program
    pub fn peek_token(&self) -> Option<&Token> {
        self.tokens.get(self.pointer_position + 1)
    }

    /// Check if we already processed every token
    pub fn out_of_tokens(&self) -> bool {
        self.pointer_position >= self.tokens.len()
    }

    /// Check if we already processed every token
    pub fn out_of_peek_tokens(&self) -> bool {
        self.pointer_position + 1 >= self.tokens.len()
    }

    /// Moves the pointer one index towards the end of the program
    pub fn move_pointer(&mut self) -> Option<()> {
        if self.out_of_tokens() { return None; }
        self.pointer_position += 1;
        Some(())
    }

    /// Moves the pointer one index away the end of the program
    pub fn move_pointer_back(&mut self) -> Option<()> {
        if self.pointer_position == 0 { return None; }
        self.pointer_position -= 1;
        Some(())
    }

    /// Get all tokens
    pub fn tokens(&self) -> &Vec<Token> {
        &self.tokens
    }

    /// Get the current pointer position
    pub fn pointer_position(&self) -> usize {
        self.pointer_position
    }
}