use std::rc::Rc;

use crate::{token::Token, token_type::Object, traits::ExprVisitor};

#[derive(Debug, Clone)]
pub enum Expr {
    Binary(Box<Binary>),
    Grouping(Box<Grouping>),
    Unary(Box<Unary>),
    Literal(Box<Literal>),
    Variable(Rc<Variable>),
    Assign(Box<Assign>),
    Logical(Box<Logical>),
}

impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        match self {
            Self::Binary(expr) => visitor.visit_binary(expr),
            Self::Grouping(expr) => visitor.visit_grouping(expr),
            Self::Unary(expr) => visitor.visit_unary(expr),
            Self::Literal(expr) => visitor.visit_literal(expr),
            Self::Variable(expr) => visitor.visit_variable(expr),
            Self::Assign(expr) => visitor.visit_assign(expr),
            Self::Logical(expr) => visitor.visit_logical(expr),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Binary {
    pub left: Expr,
    pub token: Token,
    pub right: Expr,
}

impl Binary {
    pub fn new(left: Expr, token: Token, right: Expr) -> Box<Self> {
        Box::new(Self { left, token, right })
    }
}

#[derive(Debug, Clone)]
pub struct Grouping {
    pub expression: Expr,
}

impl Grouping {
    pub fn new(expression: Expr) -> Box<Self> {
        Box::new(Self { expression })
    }
}

#[derive(Debug, Clone)]
pub struct Unary {
    pub token: Token,
    pub right: Expr,
}

impl Unary {
    pub fn new(token: Token, right: Expr) -> Box<Self> {
        Box::new(Self { token, right })
    }
}

#[derive(Debug, Clone)]
pub struct Literal {
    pub object: Object,
}

impl Literal {
    pub fn new(object: Object) -> Box<Self> {
        Box::new(Self { object })
    }
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: Token,
}

impl Variable {
    pub fn new(name: Token) -> Rc<Self> {
        Rc::new(Self { name })
    }
}

#[derive(Debug, Clone)]
pub struct Assign {
    pub name: Token,
    pub value: Expr,
}

impl Assign {
    pub fn new(name: Token, value: Expr) -> Box<Self> {
        Box::new(Self { name, value })
    }
}

#[derive(Debug, Clone)]
pub struct Logical {
    pub left: Expr,
    pub operator: Token,
    pub right: Expr,
}

impl Logical {
    pub fn new(left: Expr, operator: Token, right: Expr) -> Box<Self> {
        Box::new(Self {
            left,
            operator,
            right,
        })
    }
}
