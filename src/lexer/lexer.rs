use core::num;
use std::usize;

use crate::lexer::token::{Position, Token, TokenType};

pub struct Lexer<'a> {
    input: &'a str,
    position: Position,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Lexer {
        Lexer {
            input,
            position: Position::new(1, 0, 0),
        }
    }

    pub fn consume_char(&mut self) {
        self.position.column += 1;
        self.position.char += 1;
    }

    pub fn get_nth_char(&self, position: usize) -> Option<char> {
        return self.input.chars().nth(position);
    }

    pub fn get_current_char(&self) -> Option<char> {
        return self.input.chars().nth(self.position.char);
    }

    pub fn get_peek_char(&self) -> Option<char> {
        return self.input.chars().nth(self.position.char + 1);
    }

    pub fn lex_single_char<'b>(&mut self, kind: TokenType<'b>) -> Option<Token<'b>> {
        let position = self.position;
        self.consume_char();
        return Some(Token { position, kind });
    }

    pub fn lex_double_char<'b>(&mut self, kind: TokenType<'b>) -> Option<Token<'b>> {
        let position = self.position;
        self.consume_char();
        self.consume_char();
        Some(Token { kind, position })
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Token<'a>> {
        let current_char = self.get_current_char();
        if current_char == None {
            return None;
        }

        let peek_char = self.get_peek_char();

        match current_char.unwrap() {
            '(' => self.lex_single_char(TokenType::LParen),
            ')' => self.lex_single_char(TokenType::RParen),
            '[' => self.lex_single_char(TokenType::LBracket),
            ']' => self.lex_single_char(TokenType::RBracket),
            '{' => self.lex_single_char(TokenType::LBrace),
            '}' => self.lex_single_char(TokenType::RBrace),
            '!' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::BangEqual),
                None | Some(' ') | Some('\t') | Some('\r') => self.lex_single_char(TokenType::Bang),
                _ => panic!("Undefined token."),
            },
            '=' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::DoubleEqual),
                None | Some(' ') | Some('\t') | Some('\r') => {
                    self.lex_single_char(TokenType::Equal)
                }
                _ => panic!("Undefined token."),
            },

            '=' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::DoubleEqual),
                None | Some(' ') | Some('\t') | Some('\r') => {
                    self.lex_single_char(TokenType::Equal)
                }
                _ => panic!("Undefined token."),
            },

            '|' => match peek_char {
                Some('|') => self.lex_double_char(TokenType::Or),
                None | Some(' ') | Some('\t') | Some('\r') => self.lex_single_char(TokenType::Bar),
                _ => panic!("Undefined token."),
            },

            '&' => match peek_char {
                Some('&') => self.lex_double_char(TokenType::And),
                None | Some(' ') | Some('\t') | Some('\r') => {
                    self.lex_single_char(TokenType::Ampersand)
                }
                _ => panic!("Undefined token."),
            },
            '+' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::PlusEqual),
                None | Some(' ') | Some('\t') | Some('\r') => self.lex_single_char(TokenType::Plus),
                _ => panic!("Undefined token."),
            },
            '-' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::MinusEqual),
                None | Some(' ') | Some('\t') | Some('\r') => {
                    self.lex_single_char(TokenType::Minus)
                }
                _ => panic!("Undefined token."),
            },
            '/' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::SlashEqual),
                None | Some(' ') | Some('\t') | Some('\r') => {
                    self.lex_single_char(TokenType::Slash)
                }
                _ => panic!("Undefined token."),
            },

            '*' => match peek_char {
                Some('=') => self.lex_double_char(TokenType::AsteriskEqual),
                None | Some(' ') | Some('\t') | Some('\r') => {
                    self.lex_single_char(TokenType::Asterisk)
                }
                _ => panic!("Undefined token."),
            },
            '0'..='9' => {
                enum NumberTypes {
                    Int,
                    Float,
                }

                let position = self.position;
                let mut num = String::new();
                let mut num_type = NumberTypes::Int;

                while let Some(ch) = self.get_current_char() {
                    match ch {
                        '0'..='9' => {
                            num.push(ch);
                            self.consume_char();
                        }
                        '.' if matches!(self.get_peek_char(), Some('0'..='9')) => {
                            num_type = NumberTypes::Float;
                            num.push(ch);
                            self.consume_char();
                        }
                        _ => break,
                    }
                }

                match num_type {
                    NumberTypes::Int => match num.parse::<usize>() {
                        Ok(n) => Some(Token {
                            position,
                            kind: TokenType::Integer(n),
                        }),
                        Err(e) => panic!("{}", e),
                    },
                    NumberTypes::Float => match num.parse::<f32>() {
                        Ok(n) => Some(Token {
                            position,
                            kind: TokenType::Float(n),
                        }),
                        Err(e) => panic!("{}", e),
                    },
                }
            }
            '"' => {
                self.consume_char();
                let position = self.position;
                let mut end: usize = 0;
                while let Some(ch) = self.get_current_char() {
                    match (ch) {
                        '"' => {
                            self.consume_char();
                            end = self.position.char;
                            break;
                        }
                        '\n' => {
                            self.position.line += 1;
                            self.position.column = 0;
                            self.consume_char();
                        }
                        _ => self.consume_char(),
                    }
                }
                return Some(Token {
                    kind: TokenType::String(&self.input[position.char..end]),
                    position: self.position,
                });
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let position = self.position;
                let end: usize = 0;
                while let Some(ch) = self.get_current_char() {
                    match ch {
                        'A'..='Z' | 'a'..='z' | '_' => {
                            self.consume_char();
                        }
                        _ => break,
                    }
                }

                let slice = &self.input[position.char..end];
                let kind = TokenType::match_keyword(slice);

                Some(Token { kind, position })
            }
            '\n' => {
                self.position.line += 1;
                self.position.column = 0;
                self.consume_char();
                self.next()
            }
            ' ' | '\t' | '\r' => {
                self.consume_char();
                self.next()
            }

            _ => panic!("Undefined token."),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::lexer::lexer::{Lexer, Token};
    #[test]
    fn it_works() {
        let test_str = "1 + 2.3555";
        let l = Lexer::new(test_str);
        let z = l.collect::<Vec<_>>();
        println!("{:#?}", z);
    }
}
