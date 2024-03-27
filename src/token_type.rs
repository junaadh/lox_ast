use std::fmt;

use crate::{error::LoxError, lox_error, token::Token};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenType {
    // Single Character Tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    SemiColon,
    Slash,
    Star,
    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,
    //Literals
    Identifiers,
    String,
    Number,
    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
    Eof,
}

#[derive(Debug, Clone, Default)]
pub enum Object {
    Number(f64),
    String(String),
    #[default]
    Nil,
    Boolean(bool),
    Function(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{n}"),
            Self::String(s) => write!(f, "{s}"),
            Self::Nil => write!(f, "nil"),
            Self::Boolean(b) => write!(f, "{b}"),
            // Self::Function(f) => write!(f, "{f}"),
            _ => unimplemented!(),
        }
    }
}

impl Object {
    pub fn is_truthy(&self) -> bool {
        match self {
            Self::Nil => false,
            Self::Boolean(b) => *b,
            _ => true,
        }
    }

    pub fn truth_wrap(&self) -> Self {
        Object::Boolean(!self.is_truthy())
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Self::Number(_))
    }

    pub fn get_number(&self) -> Option<f64> {
        match self {
            Self::Number(n) => Some(n.to_owned()),
            _ => None,
        }
    }

    pub fn is_str(&self) -> bool {
        matches!(self, Self::String(_))
    }

    pub fn get_str(&self) -> Option<String> {
        match self {
            Self::String(x) => Some(x.to_owned()),
            _ => None,
        }
    }

    pub fn negate_wrap(&self) -> Self {
        match self.get_number() {
            Some(n) => Object::Number(n * -1.),
            None => Object::Nil,
        }
    }

    pub fn number_wrap(value: &f64) -> Self {
        Object::Number(value.to_owned())
    }

    pub fn str_wrap(value: &str) -> Self {
        Object::String(value.to_string())
    }

    pub fn literal_eval(&self, right: &Self, operator: Token) -> Result<Self, LoxError> {
        match (self, right) {
            (Self::String(l), Self::Number(r)) => {
                if operator.token_type == TokenType::Plus {
                    Ok(Self::String(format!("{}{}", l, r)))
                } else {
                    Err(lox_error!(
                        operator,
                        "String and Number can only be concatenated"
                    ))
                }
            }
            (Self::Number(l), Self::String(r)) => {
                if operator.token_type == TokenType::Plus {
                    Ok(Self::String(format!("{}{}", l, r)))
                } else {
                    Err(lox_error!(
                        operator,
                        "Number and String can only be concatenated"
                    ))
                }
            }
            (Self::String(l), Self::String(r)) => match operator.token_type {
                TokenType::Plus => Ok(Self::String(format!("{}{}", l, r))),
                TokenType::EqualEqual => Ok(Self::Boolean(l == r)),
                TokenType::BangEqual => Ok(Self::Boolean(l != r)),
                _ => Err(lox_error!(
                    operator,
                    "String and string doesnt support operation"
                )),
            },
            (Self::Number(l), Self::Number(r)) => match operator.token_type {
                TokenType::Minus => Ok(Object::Number(l - r)),
                TokenType::Plus => Ok(Object::Number(l + r)),
                TokenType::Slash => Ok(Object::Number(l / r)),
                TokenType::Star => Ok(Object::Number(l * r)),
                TokenType::Greater => Ok(Self::Boolean(l > r)),
                TokenType::GreaterEqual => Ok(Self::Boolean(l >= r)),
                TokenType::Less => Ok(Self::Boolean(l < r)),
                TokenType::LessEqual => Ok(Self::Boolean(l <= r)),
                TokenType::BangEqual => Ok(Self::Boolean(l != r)),
                TokenType::EqualEqual => Ok(Self::Boolean(l == r)),
                _ => Err(lox_error!(
                    operator,
                    "Number and number doesnt support operation"
                )),
            },
            _ => Err(lox_error!(operator, "Unexpected combination")),
        }
    }
}
