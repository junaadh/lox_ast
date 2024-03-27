use crate::{expr::Expr, token::Token, traits::StmtVisitor};

#[derive(Debug, Clone)]
pub enum Stmt {
    Print(Box<Print>),
    Expression(Box<Expression>),
    Var(Box<Var>),
    Block(Box<Block>),
    If(Box<If>),
    While(Box<While>),
}

impl Stmt {
    pub fn accept<R>(&self, visitor: &mut dyn StmtVisitor<R>) -> R {
        match self {
            Self::Print(stmt) => visitor.visit_print(stmt),
            Self::Expression(stmt) => visitor.visit_expression(stmt),
            Self::Var(stmt) => visitor.visit_var(stmt),
            Self::Block(stmt) => visitor.visit_block(stmt),
            Self::If(stmt) => visitor.visit_if(stmt),
            Self::While(stmt) => visitor.visit_while(stmt),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Print {
    pub expression: Expr,
}

impl Print {
    pub fn new(expression: Expr) -> Box<Self> {
        Box::new(Self { expression })
    }
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub expression: Expr,
}

impl Expression {
    pub fn new(expression: Expr) -> Box<Self> {
        Box::new(Self { expression })
    }
}

#[derive(Debug, Clone)]
pub struct Var {
    pub token: Token,
    pub initializer: Expr,
}

impl Var {
    pub fn new(token: Token, initializer: Expr) -> Box<Self> {
        Box::new(Self { token, initializer })
    }
}

#[derive(Debug, Clone)]
pub struct Block {
    pub statements: Vec<Stmt>,
}

impl Block {
    pub fn new(statements: Vec<Stmt>) -> Box<Self> {
        Box::new(Self { statements })
    }
}

#[derive(Debug, Clone)]
pub struct If {
    pub condition: Expr,
    pub then_branch: Stmt,
    pub else_branch: Option<Stmt>,
}

impl If {
    pub fn new(condition: Expr, then_branch: Stmt, else_branch: Option<Stmt>) -> Box<Self> {
        Box::new(Self {
            condition,
            then_branch,
            else_branch,
        })
    }
}

#[derive(Debug, Clone)]
pub struct While {
    pub condition: Expr,
    pub body: Stmt,
}

impl While {
    pub fn new(condition: Expr, body: Stmt) -> Box<Self> {
        Box::new(Self { condition, body })
    }
}

#[derive(Debug, Clone)]
pub struct For {}
