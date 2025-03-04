use std::collections::HashMap;

use crate::token::token::{Token, TokenType};

pub struct Lexer {
    input: String,
    position: usize,
    read_position: usize,
    ch: u8,
    reserved_word: HashMap<String, TokenType>,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut lexer = Lexer {
            input,
            position: 0,
            read_position: 0,
            ch: 0,
            reserved_word: HashMap::new(),
        };
        lexer
            .reserved_word
            .insert("let".to_string(), TokenType::LET);
        lexer
            .reserved_word
            .insert("fn".to_string(), TokenType::FUNCTION);
        lexer.reserved_word.insert("if".to_string(), TokenType::IF);
        lexer
            .reserved_word
            .insert("true".to_string(), TokenType::TRUE);
        lexer
            .reserved_word
            .insert("false".to_string(), TokenType::FALSE);
        lexer
            .reserved_word
            .insert("else".to_string(), TokenType::ELSE);
        lexer
            .reserved_word
            .insert("return".to_string(), TokenType::RETURN);
        lexer.read_char();
        lexer
    }

    pub fn next_token(&mut self) -> Token {
        let tok: Token;
        self.skip_whitespace();
        match self.ch {
            b'=' => {
                let peek_char = self.peek_char();
                if (peek_char as char) == '=' {
                    tok = Self::new_token(TokenType::EQ, "==".to_string());
                    self.read_char();
                } else {
                    tok = Self::new_token(TokenType::ASSIGN, "=".to_string());
                }
                self.read_char();
                tok
            }
            b'+' => {
                tok = Self::new_token(TokenType::PLUS, "+".to_string());
                self.read_char();
                tok
            }
            b',' => {
                tok = Self::new_token(TokenType::COMMA, ",".to_string());
                self.read_char();
                tok
            }
            b';' => {
                tok = Self::new_token(TokenType::SEMICOLON, ";".to_string());
                self.read_char();
                tok
            }
            b'(' => {
                tok = Self::new_token(TokenType::LPAREN, "(".to_string());
                self.read_char();
                tok
            }
            b')' => {
                tok = Self::new_token(TokenType::RPAREN, ")".to_string());
                self.read_char();
                tok
            }
            b'{' => {
                tok = Self::new_token(TokenType::LBRACE, "{".to_string());
                self.read_char();
                tok
            }
            b'}' => {
                tok = Self::new_token(TokenType::RBRACE, "}".to_string());
                self.read_char();
                tok
            }
            b'!' => {
                let peek_char = self.peek_char();
                if (peek_char as char) == '=' {
                    tok = Self::new_token(TokenType::NotEQ, "!=".to_string());
                    self.read_char();
                } else {
                    tok = Self::new_token(TokenType::BANG, "!".to_string());
                }
                self.read_char();
                tok
            }
            b'-' => {
                tok = Self::new_token(TokenType::MINUS, "-".to_string());
                self.read_char();
                tok
            }
            b'*' => {
                tok = Self::new_token(TokenType::ASTERISK, "*".to_string());
                self.read_char();
                tok
            }
            b'/' => {
                tok = Self::new_token(TokenType::SLASH, "/".to_string());
                self.read_char();
                tok
            }
            b'<' => {
                tok = Self::new_token(TokenType::LT, "<".to_string());
                self.read_char();
                tok
            }
            b'>' => {
                tok = Self::new_token(TokenType::GT, ">".to_string());
                self.read_char();
                tok
            }
            _ => {
                if (self.ch as char).is_ascii_alphabetic() {
                    let identifier = self.read_identifier();
                    let token_type = self.lookup_ident(identifier.clone());
                    return Self::new_token(token_type, identifier);
                } else if (self.ch as char).is_ascii_digit() {
                    let number = self.read_digit();
                    return Self::new_token(
                        TokenType::INT(number.parse::<i64>().expect("fail to parse string")),
                        number,
                    );
                } else {
                    self.read_char();
                    return Self::new_token(TokenType::EOF, "".to_string());
                }
            }
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = *self
                .input
                .as_bytes()
                .get(self.read_position)
                .expect("index out of bound");
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn peek_char(&mut self) -> u8 {
        if self.read_position >= self.input.len() {
            return 0;
        } else {
            return self
                .input
                .as_bytes()
                .get(self.read_position)
                .expect("index out of bound")
                .clone();
        }
    }

    pub fn read_identifier(&mut self) -> String {
        let position = self.read_position - 1;
        while (self.ch as char).is_ascii_alphabetic() {
            self.read_char();
        }
        self.input
            .get(position..self.read_position - 1)
            .expect("index out of bound")
            .to_string()
    }

    pub fn read_digit(&mut self) -> String {
        let position = self.read_position - 1;
        while (self.ch as char).is_ascii_digit() {
            self.read_char();
        }
        self.input
            .get(position..self.read_position - 1)
            .expect("index out of bound")
            .to_string()
    }

    pub fn skip_whitespace(&mut self) {
        while (self.ch as char) == ' '
            || (self.ch as char) == '\t'
            || (self.ch as char) == '\n'
            || (self.ch as char) == '\r'
        {
            self.read_char()
        }
    }

    pub fn lookup_ident(&self, ident: String) -> TokenType {
        if let Some(reserved_word) = self.reserved_word.get(&ident) {
            return reserved_word.clone();
        }
        TokenType::IDENT(ident)
    }

    pub fn new_token(token_type: TokenType, literal: String) -> Token {
        Token {
            token_type,
            literal,
        }
    }
}
