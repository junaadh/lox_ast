use crate::{
    expr::{Assign, Binary, Grouping, Literal, Logical, Unary, Variable},
    stmt::{Block, Expression, If, Print, Var, While},
    token_type::TokenType,
};

pub trait KeywordIdentidiers {
    fn identify(&self) -> TokenType;
}

impl KeywordIdentidiers for String {
    fn identify(&self) -> TokenType {
        match self.as_str() {
            "and" => TokenType::And,
            "class" => TokenType::Class,
            "else" => TokenType::Else,
            "false" => TokenType::False,
            "for" => TokenType::For,
            "fun" => TokenType::Fun,
            "if" => TokenType::If,
            "nil" => TokenType::Nil,
            "or" => TokenType::Or,
            "print" => TokenType::Print,
            "return" => TokenType::Return,
            "super" => TokenType::Super,
            "this" => TokenType::This,
            "true" => TokenType::True,
            "var" => TokenType::Var,
            "while" => TokenType::While,
            _ => TokenType::Identifiers,
        }
    }
}

pub trait ExprVisitor<R> {
    fn visit_binary(&mut self, expr: &Binary) -> R;
    fn visit_grouping(&mut self, expr: &Grouping) -> R;
    fn visit_unary(&mut self, expr: &Unary) -> R;
    fn visit_literal(&mut self, expr: &Literal) -> R;
    fn visit_variable(&mut self, expr: &Variable) -> R;
    fn visit_assign(&mut self, expr: &Assign) -> R;
    fn visit_logical(&mut self, expr: &Logical) -> R;
}

pub trait StmtVisitor<R> {
    fn visit_print(&mut self, stmt: &Print) -> R;
    fn visit_expression(&mut self, stmt: &Expression) -> R;
    fn visit_var(&mut self, stmt: &Var) -> R;
    fn visit_block(&mut self, stmt: &Block) -> R;
    fn visit_if(&mut self, stmt: &If) -> R;
    fn visit_while(&mut self, stmt: &While) -> R;
}
