use crate::{
    error::LoxError,
    expr::{Assign, Binary, Expr, Grouping, Literal, Logical, Unary, Variable},
    lox_error,
    stmt::{Block, Expression, If, Print, Stmt, Var, While},
    token::Token,
    token_type::{Object, TokenType},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, current: 0 }
    }

    pub fn peek(&self) -> Token {
        self.tokens.get(self.current).cloned().unwrap_or_default()
    }

    pub fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    pub fn previous(&self) -> Token {
        self.tokens
            .get(self.current - 1)
            .cloned()
            .unwrap_or_default()
    }

    pub fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    pub fn check(&self, token_type: &TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            &self.peek().token_type == token_type
        }
    }

    pub fn match_types(&mut self, token_types: &[TokenType]) -> bool {
        for token in token_types.iter() {
            if self.check(token) {
                self.advance();
                return true;
            }
        }
        false
    }

    pub fn expression(&mut self) -> Result<Expr, LoxError> {
        self.assignment()
    }

    fn assignment(&mut self) -> Result<Expr, LoxError> {
        let expr = self.or()?;

        if self.match_types(&[TokenType::Equal]) {
            let equal = self.previous();
            let value = self.assignment()?;

            if let Expr::Variable(var) = expr {
                Ok(Expr::Assign(Assign::new(var.name.clone(), value)))
            } else {
                Err(lox_error!(equal.clone(), "Invalid assignment target."))
            }
        } else {
            Ok(expr)
        }
    }

    fn or(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.and()?;

        while self.match_types(&[TokenType::Or]) {
            let operator = self.previous();
            let right = self.and()?;
            expr = Expr::Logical(Logical::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn and(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.equality()?;

        while self.match_types(&[TokenType::And]) {
            let operator = self.previous();
            let right = self.equality()?;
            expr = Expr::Logical(Logical::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.comparison()?;

        while self.match_types(&[TokenType::BangEqual, TokenType::EqualEqual]) {
            let operator = self.previous();
            let right = self.comparison()?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }

        Ok(expr)
    }

    fn comparison(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.term()?;

        while self.match_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = self.term()?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.factor()?;
        while self.match_types(&[TokenType::Slash, TokenType::Star]) {
            let operator = self.previous();
            let right = self.factor()?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, LoxError> {
        let mut expr = self.unary()?;
        while self.match_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = self.unary()?;
            expr = Expr::Binary(Binary::new(expr, operator, right));
        }
        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, LoxError> {
        if self.match_types(&[TokenType::Minus, TokenType::Bang]) {
            let operator = self.previous();
            let right = self.unary()?;
            Ok(Expr::Unary(Unary::new(operator, right)))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, LoxError> {
        if self.match_types(&[TokenType::False]) {
            return Ok(Expr::Literal(Literal::new(Object::Boolean(false))));
        }
        if self.match_types(&[TokenType::True]) {
            return Ok(Expr::Literal(Literal::new(Object::Boolean(true))));
        }
        if self.match_types(&[TokenType::Nil]) {
            return Ok(Expr::Literal(Literal::new(Object::Nil)));
        }
        if self.match_types(&[TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(Literal::new(
                self.previous().literal.unwrap_or_default(),
            )));
        }
        if self.match_types(&[TokenType::Identifiers]) {
            return Ok(Expr::Variable(Variable::new(self.previous())));
        }
        if self.match_types(&[TokenType::LeftParen]) {
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Expect ')' after expression")?;
            return Ok(Expr::Grouping(Grouping::new(expr)));
        }

        let err = lox_error!(self.peek(), "Expect expression");
        self.synchronize();
        Err(err)
    }

    pub fn consume(&mut self, token_type: TokenType, msg: &str) -> Result<Token, LoxError> {
        if self.check(&token_type) {
            Ok(self.advance())
        } else {
            Err(lox_error!(self.peek(), msg))
        }
    }

    pub fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().token_type == TokenType::SemiColon {
                return;
            }

            match self.peek().token_type {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => return,
                _ => {}
            }

            self.advance();
        }
    }

    pub fn parse(&mut self) -> Result<Vec<Stmt>, LoxError> {
        // self.expression()
        let mut stmts = Vec::<Stmt>::new();
        while !self.is_at_end() {
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }
        Ok(stmts)
    }

    fn declaration(&mut self) -> Result<Stmt, LoxError> {
        if self.match_types(&[TokenType::Var]) {
            match self.var_declaration() {
                Ok(stmt) => Ok(stmt),
                Err(err) => {
                    self.synchronize();
                    Err(err)
                }
            }
        } else {
            match self.statement() {
                Ok(stmt) => Ok(stmt),
                Err(err) => {
                    self.synchronize();
                    Err(err)
                }
            }
        }
    }

    fn statement(&mut self) -> Result<Stmt, LoxError> {
        if self.match_types(&[TokenType::Print]) {
            return self.print_statement();
        }
        if self.match_types(&[TokenType::LeftBrace]) {
            return self.block();
        }
        if self.match_types(&[TokenType::If]) {
            return self.if_statement();
        }
        if self.match_types(&[TokenType::While]) {
            return self.while_statement();
        }

        self.expression_statement()
    }

    fn print_statement(&mut self) -> Result<Stmt, LoxError> {
        let value = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after value.")?;
        Ok(Stmt::Print(Print::new(value)))
    }

    fn expression_statement(&mut self) -> Result<Stmt, LoxError> {
        let expr = self.expression()?;
        self.consume(TokenType::SemiColon, "Expect ';' after expression.")?;
        Ok(Stmt::Expression(Expression::new(expr)))
    }

    fn var_declaration(&mut self) -> Result<Stmt, LoxError> {
        let name = self.consume(TokenType::Identifiers, "Expect variable name.")?;
        let initializer = if self.match_types(&[TokenType::Equal]) {
            self.expression().ok()
        } else {
            None
        };
        // println!("{:?}", self.previous());
        // println!("{:?}", self.peek());
        self.consume(
            TokenType::SemiColon,
            "Expect ';' after variable declaration.",
        )?;
        match initializer {
            Some(expr) => Ok(Stmt::Var(Var::new(name, expr))),
            None => Err(lox_error!(name, "Failed to declare variable.")),
        }
    }

    fn block(&mut self) -> Result<Stmt, LoxError> {
        let mut stmts = Vec::<Stmt>::new();
        while !self.check(&TokenType::RightBrace) && !self.is_at_end() {
            let stmt = self.declaration()?;
            stmts.push(stmt);
        }
        self.consume(TokenType::RightBrace, "Expect '}' after block.")?;

        Ok(Stmt::Block(Block::new(stmts)))
    }

    fn if_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after an 'if'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after an 'if'.")?;

        let then_branch = self.statement()?;
        let else_branch = if self.match_types(&[TokenType::Else]) {
            self.statement().ok()
        } else {
            None
        };

        Ok(Stmt::If(If::new(condition, then_branch, else_branch)))
    }

    fn while_statement(&mut self) -> Result<Stmt, LoxError> {
        self.consume(TokenType::LeftParen, "Expect '(' after 'while'.")?;
        let condition = self.expression()?;
        self.consume(TokenType::RightParen, "Expect ')' after 'while'.")?;

        let body = self.statement()?;
        Ok(Stmt::While(While::new(condition, body)))
    }
}
