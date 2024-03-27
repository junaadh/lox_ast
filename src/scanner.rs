use crate::{
    lox_error_report,
    token::Token,
    token_type::{Object, TokenType},
    traits::KeywordIdentidiers,
    LoxError,
};

#[derive(Debug)]
pub struct Scanner {
    source: String,
    char_vec: Vec<char>,
    start: usize,
    current: usize,
    line: usize,
    pub tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &str) -> Self {
        Self {
            source: source.to_string(),
            char_vec: source.chars().collect(),
            start: 0,
            current: 0,
            line: 1,
            tokens: Vec::new(),
        }
    }

    pub fn scan_tokens(&mut self) {
        loop {
            self.start = self.current;
            let char = self.char_vec.get(self.current).unwrap_or(&' ');
            self.current += 1;
            match char {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '{' => self.add_token(TokenType::LeftBrace),
                '}' => self.add_token(TokenType::RightBrace),
                ',' => self.add_token(TokenType::Comma),
                '.' => self.add_token(TokenType::Dot),
                '-' => self.add_token(TokenType::Minus),
                '+' => self.add_token(TokenType::Plus),
                ';' => self.add_token(TokenType::SemiColon),
                '*' => self.add_token(TokenType::Star),
                '!' => {
                    let tok = if self.match_char('=') {
                        TokenType::BangEqual
                    } else {
                        TokenType::Bang
                    };
                    self.add_token(tok);
                }
                '=' => {
                    let tok = if self.match_char('=') {
                        TokenType::EqualEqual
                    } else {
                        TokenType::Equal
                    };
                    self.add_token(tok);
                }
                '<' => {
                    let tok = if self.match_char('=') {
                        TokenType::LessEqual
                    } else {
                        TokenType::Less
                    };
                    self.add_token(tok);
                }
                '>' => {
                    let tok = if self.match_char('=') {
                        TokenType::GreaterEqual
                    } else {
                        TokenType::Greater
                    };
                    self.add_token(tok);
                }
                '/' => {
                    if self.match_char('/') {
                        while self.peek() != '\n' && !self.at_end() {
                            self.current += 1;
                        }
                    // } else if self.match_char('*') {
                    //     while self.peek() != '*' && self.double_peek() != '/' && !self.at_end() {
                    //         if self.peek() == '\n' {
                    //             self.line += 1;
                    //         }
                    //         self.current += 1;
                    //     }
                    //     self.current += 2;
                    } else {
                        self.add_token(TokenType::Slash);
                    };
                }
                ' ' | '\r' | '\t' => (),
                '\n' => self.line += 1,
                '"' => {
                    let _ = self.handle_string();
                }
                _ => {
                    if char.is_numeric() {
                        let _ = self.handle_digit();
                    } else if char.is_alphabetic() {
                        let _ = self.handle_identifier();
                    } else {
                        lox_error_report!(
                            self.line,
                            format!("{char}{}", self.peek()).as_str(),
                            "Unexpected character"
                        );
                    }
                }
            }
            if self.at_end() {
                break;
            }
        }
        self.append_eof();
    }

    fn append_eof(&mut self) {
        self.add(Token::new(TokenType::Eof, "".to_string(), None, self.line));
    }

    pub fn add(&mut self, tok: Token) {
        self.tokens.push(tok);
    }

    pub fn add_token(&mut self, token_type: TokenType) {
        self.add_tokens(token_type, None);
    }

    pub fn add_tokens(&mut self, token_type: TokenType, literal: Option<Object>) {
        let text = self.source[self.start..self.current].to_string();
        // println!("{text}");
        self.add(Token::new(token_type, text, literal, self.line));
    }

    pub fn match_char(&mut self, expected: char) -> bool {
        if self.at_end() {
            false
        } else if self.char_vec.get(self.current).unwrap_or(&' ') == &expected {
            self.current += 1;
            return true;
        } else {
            return false;
        }
    }

    pub fn at_end(&self) -> bool {
        self.current >= self.char_vec.len()
    }

    pub fn peek(&self) -> char {
        if self.at_end() {
            '\0'
        } else {
            return *self.char_vec.get(self.current).unwrap_or(&' ');
        }
    }

    pub fn double_peek(&self) -> char {
        if self.current + 1 >= self.char_vec.len() {
            '\0'
        } else {
            return *self.char_vec.get(self.current + 1).unwrap_or(&' ');
        }
    }

    pub fn handle_string(&mut self) -> Result<(), LoxError> {
        // consume " opening
        self.current += 1;
        loop {
            if self.peek() != '"' && !self.at_end() {
                // println!("Here");
                if self.peek() == '\n' {
                    self.line += 1;
                }
                self.current += 1;
            } else {
                // println!("Broke. why? {}", self.char_vec.get(self.current).unwrap());
                break;
            }
        }

        if self.at_end() {
            lox_error_report!(self.line, "", "Unterminated String");
        }

        self.current += 1;
        let text = self.source[self.start + 1..self.current - 1].to_string();
        self.add_tokens(TokenType::String, Some(Object::String(text)));
        Ok(())
    }

    pub fn handle_digit(&mut self) -> Result<(), LoxError> {
        loop {
            if self.peek().is_numeric() {
                self.current += 1;
            } else {
                break;
            }
        }

        if self.peek() == '.' && self.double_peek().is_numeric() {
            self.current += 1;
            loop {
                if self.peek().is_numeric() {
                    self.current += 1;
                } else {
                    break;
                }
            }
        }
        let number = self.source[self.start..self.current]
            .to_string()
            .parse::<f64>()
            .unwrap_or_default();
        self.add_tokens(TokenType::Number, Some(Object::Number(number)));
        Ok(())
    }

    pub fn handle_identifier(&mut self) -> Result<(), LoxError> {
        loop {
            if self.peek().is_alphanumeric() {
                self.current += 1;
            } else {
                break;
            }
        }

        let identifier = self.source[self.start..self.current].to_string().identify();
        self.add_token(identifier);
        Ok(())
    }
}
