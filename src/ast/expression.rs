#![allow(unused)]

use std::fmt::{Debug, Display, format, Formatter};
use anyhow::bail;
use crate::evaluate::object::{Evaluate, Object};
use crate::ast::statement::{BlockStatement, Statement};
use crate::evaluate::error::EvalError::{IllegalOperation, MixedTypeOperation};
use crate::lexer::token::Token;


pub trait Expression: Display + Debug + Evaluate {}

#[derive(Debug)]
pub struct Identifier {
    value: String,
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Evaluate for Identifier {
    fn eval(&self) -> anyhow::Result<Object> {
        todo!()
    }
}

impl Expression for Identifier {}

impl Display for Identifier {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.value))
    }
}

#[derive(Debug)]
pub struct Integer {
    val: String,
}

impl Integer {
    pub fn new(val: String) -> Self {
        Self { val }
    }
}

impl Evaluate for Integer {
    fn eval(&self) -> anyhow::Result<Object> {
        Ok(Object::Int(self.val.parse()?))
    }
}

impl Expression for Integer {}

impl Display for Integer {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.val)
    }
}

#[derive(Debug)]
pub struct Boolean {
    val: bool,
}

impl Boolean {
    pub fn new(val: bool) -> Self {
        Self { val }
    }
}

impl Evaluate for Boolean {
    fn eval(&self) -> anyhow::Result<Object> {
        Ok(Object::Bool(self.val))
    }
}

impl Expression for Boolean {}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.val))
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    prefix: Token,
    right: Box<dyn Expression>,
}

impl PrefixExpression {
    pub fn new(prefix: Token, right: Box<dyn Expression>) -> Self {
        Self { prefix, right }
    }
}

impl Evaluate for PrefixExpression {
    fn eval(&self) -> anyhow::Result<Object> {
        let val = self.right.eval()?;
        Ok(match self.prefix {
            Token::Minus => {
                match val {
                    Object::Int(val) => Object::Int(-val),
                    _ => bail!(IllegalOperation(Token::Minus, val)),
                }
            }
            Token::Bang => {
                match val {
                    Object::Bool(val) => Object::Bool(!val),
                    _ => bail!(IllegalOperation(Token::Bang, val)),
                }
            }
            _ => unreachable!(),
        })
    }
}

impl Expression for PrefixExpression {}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}{})", self.prefix, self.right))
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    left: Box<dyn Expression>,
    operator: Token,
    right: Box<dyn Expression>,
}

impl InfixExpression {
    pub fn new(left: Box<dyn Expression>, operator: Token, right: Box<dyn Expression>) -> Self {
        Self { left, operator, right }
    }
}

impl Evaluate for InfixExpression {
    fn eval(&self) -> anyhow::Result<Object> {
        let left = self.left.eval()?;
        let right = self.right.eval()?;

        if !Object::variant_is_equal(&left, &right) { bail!(MixedTypeOperation(self.operator.clone(), left, right)) }

        match self.operator {
            Token::Plus |
            Token::Minus |
            Token::Asterisk |
            Token::Slash |
            Token::Lt |
            Token::Gt |
            Token::Lte |
            Token::Gte => {
                match left {
                    Object::Int(_) => {}
                    _ => bail!(IllegalOperation(self.operator.clone(), left))
                }
            },
            Token::Equal |
            Token::NotEqual => match left {
                Object::Int(_) => {}
                Object::Bool(_) => {}
                _ => bail!(IllegalOperation(self.operator.clone(), left))
            },
            _ => unreachable!()
        }
        let left_int = match left {
            Object::Int(val) => Some(val),
            _ => None,
        };
        let right_int = match right {
            Object::Int(val) => Some(val),
            _ => None,
        };

        let left_bool = match left {
            Object::Bool(val) => Some(val),
            _ => None,
        };
        let right_bool = match right {
            Object::Bool(val) => Some(val),
            _ => None,
        };

        Ok(match self.operator {
            Token::Plus => Object::Int(left_int.unwrap() + right_int.unwrap()),
            Token::Minus => Object::Int(left_int.unwrap() - right_int.unwrap()),
            Token::Asterisk => Object::Int(left_int.unwrap() * right_int.unwrap()),
            Token::Slash => Object::Int(left_int.unwrap() / right_int.unwrap()),
            Token::Equal => Object::Bool({
                if left_bool.is_some() { left_bool.unwrap() == right_bool.unwrap()  }
                else { left_int.unwrap() == right_int.unwrap() }
            }),
            Token::NotEqual => Object::Bool({
                if left_bool.is_some() { left_bool.unwrap() != right_bool.unwrap()  }
                else { left_int.unwrap() != right_int.unwrap() }
            }),
            Token::Lt => Object::Bool(left_int.unwrap() < right_int.unwrap()),
            Token::Gt => Object::Bool(left_int.unwrap() > right_int.unwrap()),
            Token::Lte => Object::Bool(left_int.unwrap() <= right_int.unwrap()),
            Token::Gte => Object::Bool(left_int.unwrap() >= right_int.unwrap()),
            _ => unreachable!()
        })
    }
}

impl Expression for InfixExpression {}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({} {} {})", self.left, self.operator, self.right))
    }
}

#[derive(Debug)]
pub struct IfExpression {
    condition: Box<dyn Expression>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(condition: Box<dyn Expression>, consequence: BlockStatement, alternative: Option<BlockStatement>) -> Self {
        Self { condition, consequence, alternative }
    }
}

impl Evaluate for IfExpression {
    fn eval(&self) -> anyhow::Result<Object> {
        todo!()
    }
}

impl Expression for IfExpression {}

impl Display for IfExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if let Some(alternative) = &self.alternative {
            f.write_str(&format!("if ({}) {{ {} }} else {{ {} }}", self.condition, self.consequence, alternative))
        } else {
            f.write_str(&format!("if ({}) {{ {} }}", self.condition, self.consequence))
        }
    }
}

#[derive(Debug)]
pub struct FunctionExpression {
    parameters: Vec<Identifier>,
    body: BlockStatement,
}

impl FunctionExpression {
    pub fn new(parameters: Vec<Identifier>, body: BlockStatement) -> Self {
        Self { parameters, body }
    }
}

impl Evaluate for FunctionExpression {
    fn eval(&self) -> anyhow::Result<Object> {
        todo!()
    }
}

impl Expression for FunctionExpression {}

impl Display for FunctionExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut parameters = String::new();
        for param in &self.parameters {
            parameters.push_str(&param.to_string());
            parameters.push(',');
        }
        parameters.pop();
        f.write_str(&format!("fn({}){{ {} }}", parameters, self.body))
    }
}

#[derive(Debug)]
pub struct CallExpression {
    function: Box<dyn Expression>,
    arguments: Vec<Box<dyn Expression>>,
}

impl CallExpression {
    pub fn new(function: Box<dyn Expression>, arguments: Vec<Box<dyn Expression>>) -> Self {
        Self { function, arguments }
    }
}

impl Evaluate for CallExpression {
    fn eval(&self) -> anyhow::Result<Object> {
        todo!()
    }
}

impl Expression for CallExpression {}

impl Display for CallExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut args = String::new();
        for arg in &self.arguments {
            args.push_str(&arg.to_string());
            args.push(',');
        }
        args.pop();
        f.write_str(&format!("{}({})", self.function, args))
    }
}