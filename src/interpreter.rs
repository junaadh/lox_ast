use std::borrow::Borrow;

use crate::{
    environment::Environment,
    error::LoxError,
    expr::Expr,
    stmt::Stmt,
    token_type::{Object, TokenType},
    traits::{ExprVisitor, StmtVisitor},
};

#[derive(Debug)]
pub struct Interpreter {
    environment: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        Self {
            environment: Environment::initialize(),
        }
    }
}

impl Interpreter {
    fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    // pub fn interpret(&mut self, expr: &Expr) -> Result<(), LoxError> {
    //     let value = self.evaluate(expr);
    //     match value {
    //         Ok(val) => {
    //             println!("{}", val);
    //             Ok(())
    //         }
    //         Err(err) => Err(err),
    //     }
    // }
    pub fn interpret(&mut self, stmts: Vec<Stmt>) -> Result<(), LoxError> {
        for stmt in stmts {
            // println!("{:?}", stmt);
            self.execute(stmt.borrow())?;
            // println!("{:#?}", self.environment);
        }
        Ok(())
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), LoxError> {
        let outer_scope = self.environment.clone();
        self.environment = environment;
        for statement in statements {
            self.execute(statement)?;
        }
        self.environment = outer_scope;
        Ok(())
    }
}

impl ExprVisitor<Result<Object, LoxError>> for Interpreter {
    fn visit_binary(&mut self, expr: &crate::expr::Binary) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        left.literal_eval(&right, expr.token.clone())
    }

    fn visit_grouping(&mut self, expr: &crate::expr::Grouping) -> Result<Object, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_unary(&mut self, expr: &crate::expr::Unary) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;
        match expr.token.token_type {
            TokenType::Minus => Ok(right.negate_wrap()),
            TokenType::Bang => Ok(right.truth_wrap()),
            _ => Err(LoxError::from(
                expr.token.borrow().to_owned(),
                "Unreachable",
            )),
        }
    }

    fn visit_literal(&mut self, expr: &crate::expr::Literal) -> Result<Object, LoxError> {
        Ok(expr.object.clone())
    }

    fn visit_variable(&mut self, expr: &crate::expr::Variable) -> Result<Object, LoxError> {
        let var = self.environment.get(&expr.name)?;
        Ok(var)
    }

    fn visit_assign(&mut self, expr: &crate::expr::Assign) -> Result<Object, LoxError> {
        let val = self.evaluate(&expr.value)?;
        self.environment.assign(&expr.name, val.clone())?;
        Ok(val)
    }

    fn visit_logical(&mut self, expr: &crate::expr::Logical) -> Result<Object, LoxError> {
        let val = self.evaluate(&expr.left)?;

        if expr.operator.token_type == TokenType::Or {
            if val.is_truthy() {
                return Ok(val);
            }
        } else if !val.is_truthy() {
            return Ok(val);
        }

        self.evaluate(&expr.right)
    }
}

impl StmtVisitor<Result<(), LoxError>> for Interpreter {
    fn visit_print(&mut self, stmt: &crate::stmt::Print) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{}", value);
        Ok(())
    }

    fn visit_expression(&mut self, stmt: &crate::stmt::Expression) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_var(&mut self, stmt: &crate::stmt::Var) -> Result<(), LoxError> {
        let val = self.evaluate(&stmt.initializer)?;
        self.environment.define(stmt.token.lexeme.clone(), val);
        // println!("{:#?}", self.environment);
        Ok(())
    }

    fn visit_block(&mut self, stmt: &crate::stmt::Block) -> Result<(), LoxError> {
        let env = Environment::from(self.environment.clone());
        self.execute_block(&stmt.statements, env)?;
        Ok(())
    }

    fn visit_if(&mut self, stmt: &crate::stmt::If) -> Result<(), LoxError> {
        if self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&stmt.then_branch)?;
        } else if stmt.else_branch.is_some() {
            self.execute(&stmt.else_branch.clone().unwrap())?;
        }
        Ok(())
    }

    fn visit_while(&mut self, stmt: &crate::stmt::While) -> Result<(), LoxError> {
        while self.evaluate(&stmt.condition)?.is_truthy() {
            self.execute(&stmt.body)?;
        }
        Ok(())
    }
}
