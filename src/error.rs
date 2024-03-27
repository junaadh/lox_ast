use core::fmt;

use crate::{token::Token, token_type::TokenType, ERROR};

#[derive(Debug, Clone, Default)]
pub struct LoxError {
    pub line: usize,
    pub where_error: String,
    pub msg: String,
}

impl fmt::Display for LoxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}] Error {}: {}",
            self.line, self.where_error, self.msg
        )
    }
}

impl LoxError {
    pub fn new(line: usize, where_err: &str, msg: &str) -> Self {
        Self {
            line,
            where_error: where_err.to_string(),
            msg: msg.to_string(),
        }
    }

    pub fn from(token: Token, msg: &str) -> Self {
        if token.token_type == TokenType::Eof {
            Self::new(token.line, " at end ", msg)
        } else {
            Self::new(token.line, format!(" at '{}' ", token.lexeme).as_str(), msg)
        }
    }

    pub fn set_err_global(&self, bool: bool) {
        let mut error = ERROR.lock().unwrap();
        *error = bool;
    }

    pub fn report(&self) -> Self {
        self.set_err_global(true);
        self.clone()
    }

    pub fn clear() {
        let mut error = ERROR.lock().unwrap();
        *error = false;
    }
}

#[macro_export]
macro_rules! lox_error {
    () => {
        LoxError::default()
    };
    ($line: expr, $where_err: expr, $msg: expr) => {
        LoxError::new($line, $where_err, $msg)
    };
    ($token: expr, $msg: expr) => {
        LoxError::from($token, $msg)
    };
}

#[macro_export]
macro_rules! lox_error_report {
    ($line: expr, $where_err: expr, $msg: expr) => {
        LoxError::new($line, $where_err, $msg).report()
    };
    ($token: expr, $msg: expr) => {
        LoxError::from($token, $msg).report()
    };
}
