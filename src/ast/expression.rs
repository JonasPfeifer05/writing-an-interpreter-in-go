use std::any::{Any, TypeId};
use std::fmt::{Debug, Display, format, Formatter};
use anyhow::bail;
use crate::ast::precedences::Precedences::{Call, Prefix};
use crate::evaluate::object::{Evaluate, Object};
use crate::ast::statement::{BlockStatement, CloneAsStatement, Statement};
use crate::evaluate::environment::Environment;
use crate::evaluate::error::EvalError::{CannotCallNoneFunctinal, DifferentAmountOfArguments, IllegalOperation, MixedTypeOperation, UnexpectedObject, UnknownIdentifier};
use crate::evaluate::evaluate::eval_all;
use crate::lexer::token::Token;

pub trait CloneAsExpression: Send + Sync {
    fn clone_as_expression(&self) -> Box<dyn Expression + Send + Sync>;
}

pub trait Expression: Display + Debug + Evaluate + CloneAsExpression + Send + Sync {
    fn expression_id(&self) -> TypeId;
    fn as_any(&mut self) -> &mut dyn Any;
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
pub struct Identifier {
    value: String,
}

impl Identifier {
    pub fn new(value: String) -> Self {
        Self { value }
    }
}

impl Evaluate for Identifier {
    fn eval(&mut self, environment: &mut Environment) -> anyhow::Result<Object> {
        let result = environment.get(&self.value).ok_or(UnknownIdentifier(self.value.clone()))?;
        Ok(result.clone())
    }
}

impl CloneAsExpression for Identifier {
    fn clone_as_expression(&self) -> Box<dyn Expression + Send + Sync> {
        Box::new(Identifier::new(self.value.clone()))
    }
}

impl Expression for Identifier {
    fn expression_id(&self) -> TypeId {
        TypeId::of::<Identifier>()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

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
    fn eval(&mut self, environment: &mut Environment) -> anyhow::Result<Object> {
        Ok(Object::Int(self.val.parse()?))
    }
}

impl CloneAsExpression for Integer {
    fn clone_as_expression(&self) -> Box<dyn Expression + Send + Sync> {
        Box::new(Integer::new(self.val.clone()))
    }
}

impl Expression for Integer {
    fn expression_id(&self) -> TypeId {
        TypeId::of::<Integer>()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

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
    fn eval(&mut self, environment: &mut Environment) -> anyhow::Result<Object> {
        Ok(Object::Bool(self.val))
    }
}

impl CloneAsExpression for Boolean {
    fn clone_as_expression(&self) -> Box<dyn Expression + Send + Sync> {
        Box::new(Boolean::new(self.val))
    }
}

impl Expression for Boolean {
    fn expression_id(&self) -> TypeId {
        TypeId::of::<Boolean>()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for Boolean {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("{}", self.val))
    }
}

#[derive(Debug)]
pub struct PrefixExpression {
    prefix: Token,
    right: Box<dyn Expression + Send + Sync>,
}

impl PrefixExpression {
    pub fn new(prefix: Token, right: Box<dyn Expression + Send + Sync>) -> Self {
        Self { prefix, right }
    }
}

impl Evaluate for PrefixExpression {
    fn eval(&mut self, environment: &mut Environment) -> anyhow::Result<Object> {
        let val = self.right.eval(environment)?;
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

impl CloneAsExpression for PrefixExpression {
    fn clone_as_expression(&self) -> Box<dyn Expression + Send + Sync> {
        Box::new(PrefixExpression::new(self.prefix.clone(), self.right.clone_as_expression()))
    }
}

impl Expression for PrefixExpression {
    fn expression_id(&self) -> TypeId {
        TypeId::of::<PrefixExpression>()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for PrefixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({}{})", self.prefix, self.right))
    }
}

#[derive(Debug)]
pub struct InfixExpression {
    left: Box<dyn Expression + Send + Sync>,
    operator: Token,
    right: Box<dyn Expression + Send + Sync>,
}

impl InfixExpression {
    pub fn new(left: Box<dyn Expression + Send + Sync>, operator: Token, right: Box<dyn Expression + Send + Sync>) -> Self {
        Self { left, operator, right }
    }
}

impl Evaluate for InfixExpression {
    fn eval(&mut self, environment: &mut Environment) -> anyhow::Result<Object> {
        let left = self.left.eval(environment)?;
        let right = self.right.eval(environment)?;

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

impl CloneAsExpression for InfixExpression {
    fn clone_as_expression(&self) -> Box<dyn Expression + Send + Sync> {
        Box::new(InfixExpression::new(self.left.clone_as_expression(), self.operator.clone(), self.right.clone_as_expression()))
    }
}

impl Expression for InfixExpression {
    fn expression_id(&self) -> TypeId {
        TypeId::of::<InfixExpression>()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

impl Display for InfixExpression {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&format!("({} {} {})", self.left, self.operator, self.right))
    }
}

#[derive(Debug)]
pub struct IfExpression {
    condition: Box<dyn Expression + Send + Sync>,
    consequence: BlockStatement,
    alternative: Option<BlockStatement>,
}

impl IfExpression {
    pub fn new(condition: Box<dyn Expression + Send + Sync>, consequence: BlockStatement, alternative: Option<BlockStatement>) -> Self {
        Self { condition, consequence, alternative }
    }
}

impl Evaluate for IfExpression {
    fn eval(&mut self, environment: &mut Environment) -> anyhow::Result<Object> {
        let condition = match self.condition.eval(environment)? {
            Object::Bool(val) => val,
            _ => bail!(UnexpectedObject("Boolean".to_string()))
        };

        if condition {
            eval_all(self.consequence.statements(), environment, false)
        }
        else if let Some(alternative) = &mut self.alternative {
            eval_all(alternative.statements(),environment, false)
        } else {
            Ok(Object::Null)
        }
    }
}

impl CloneAsExpression for IfExpression {
    fn clone_as_expression(&self) -> Box<dyn Expression + Send + Sync> {
        let alt = if let Some(alt) = &self.alternative {
            Some(alt.clone_as_block_statement())
        } else { None };
        Box::new(IfExpression::new(self.condition.clone_as_expression(), self.consequence.clone_as_block_statement(), alt))
    }
}

impl Expression for IfExpression {
    fn expression_id(&self) -> TypeId {
        TypeId::of::<IfExpression>()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

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
    env: Environment,
}

impl FunctionExpression {
    pub fn new(parameters: Vec<Identifier>, body: BlockStatement, env: Environment) -> Self {
        Self { parameters, body, env }
    }
}

impl Evaluate for FunctionExpression {
    fn eval(&mut self, environment: &mut Environment) -> anyhow::Result<Object> {
        Ok(Object::Function {
            parameters: self.parameters.clone(),
            body: self.body.clone_as_block_statement(),
            env: environment.clone(),
        })
    }
}

impl CloneAsExpression for FunctionExpression {
    fn clone_as_expression(&self) -> Box<dyn Expression + Send + Sync> {
        Box::new(FunctionExpression::new(self.parameters.clone(), self.body.clone_as_block_statement(), Environment::default()))
    }
}

impl Expression for FunctionExpression {
    fn expression_id(&self) -> TypeId {
        TypeId::of::<FunctionExpression>()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

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
    function: Box<dyn Expression + Send + Sync>,
    arguments: Vec<Box<dyn Expression + Send + Sync>>,
}

impl CallExpression {
    pub fn new(function: Box<dyn Expression + Send + Sync>, arguments: Vec<Box<dyn Expression + Send + Sync>>) -> Self {
        Self { function, arguments }
    }
}

impl Evaluate for CallExpression {
    fn eval(&mut self, environment: &mut Environment) -> anyhow::Result<Object> {
        if self.function.expression_id() != TypeId::of::<FunctionExpression>()
        && self.function.expression_id() != TypeId::of::<Identifier>() { bail!(CannotCallNoneFunctinal) }

        let mut function = if self.function.expression_id() == TypeId::of::<Identifier>() {
            let function_object = environment.get(&self.function.to_string()).unwrap();
            Box::new(match function_object {
                Object::Function { body, parameters, env } => FunctionExpression::new(parameters.clone(), body.clone_as_block_statement(), env.clone()),
                _ => bail!(CannotCallNoneFunctinal)
            })
        } else {
            self.function.clone_as_expression()
        };

        let function = function.as_any().downcast_mut::<FunctionExpression>().unwrap();

        if self.arguments.len() != function.parameters.len() { bail!(DifferentAmountOfArguments) }

        let mut env = environment.clone();

        for env_var in function.env.store().iter() {
            env.set(env_var.0.clone(), env_var.1.clone());
        }

        function.env = env;

        for i in 0..self.arguments.len() {
            function.env.set(function.parameters[i].value.clone(), self.arguments[i].eval(environment)?)
        }

        eval_all(&mut function.body.statements(), &mut function.env, true)
    }
}

impl CloneAsExpression for CallExpression {
    fn clone_as_expression(&self) -> Box<dyn Expression + Send + Sync> {
        let mut args = vec![];
        for arg in &self.arguments {
            args.push(arg.clone_as_expression())
        }
        Box::new(CallExpression::new(self.function.clone_as_expression(), args))
    }
}

impl Expression for CallExpression {
    fn expression_id(&self) -> TypeId {
        TypeId::of::<CallExpression>()
    }

    fn as_any(&mut self) -> &mut dyn Any {
        self
    }
}

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