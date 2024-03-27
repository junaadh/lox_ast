use crate::token_type::{Object, TokenType};

#[derive(Debug, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Object>,
    pub line: usize,
}

impl Default for Token {
    fn default() -> Self {
        Self {
            token_type: TokenType::Nil,
            lexeme: String::new(),
            literal: None,
            line: 0,
        }
    }
}

impl Token {
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Object>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        format!(
            "{:?} {} {}",
            self.token_type,
            self.lexeme,
            self.literal.clone().unwrap_or_default()
        )
    }
}
