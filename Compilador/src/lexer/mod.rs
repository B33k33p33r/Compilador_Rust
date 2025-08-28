use crate::lexer::token::Token;
use anyhow::Result;

pub struct Lexer {
    input: Vec<char>,
    pos: usize,
    ch: char,
}

impl Lexer {
    pub fn new(input: String) -> Self {
        let chars: Vec<char> = input.chars().collect();
        let mut lexer = Lexer {
            input: chars,
            pos: 0,
            ch: '\0',
        };
        if !lexer.input.is_empty() {
            lexer.ch = lexer.input[0];
        }
        lexer
    }

    fn read_char(&mut self) {
        self.pos += 1;
        if self.pos >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.pos];
        }
    }

    fn skip_whitespace(&mut self) {
        while self.ch.is_whitespace() && self.ch != '\0' {
            self.read_char();
        }
    }

    fn read_identifier(&mut self) -> String {
        let start = self.pos;
        while self.ch.is_alphabetic() || self.ch == '_' {
            self.read_char();
        }
        self.input[start..self.pos].iter().collect()
    }

    fn read_number(&mut self) -> i64 {
        let start = self.pos;
        while self.ch.is_digit(10) {
            self.read_char();
        }
        self.input[start..self.pos].iter().collect::<String>().parse().unwrap()
    }

    fn read_string(&mut self) -> String {
        self.read_char(); // skip opening quote
        let start = self.pos;
        while self.ch != '"' && self.ch != '\0' {
            self.read_char();
        }
        let result: String = self.input[start..self.pos].iter().collect();
        if self.ch == '"' {
            self.read_char(); // skip closing quote
        }
        result
    }

    pub fn next_token(&mut self) -> Result<Token> {
        self.skip_whitespace();

        let tok = match self.ch {
            '=' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    Token::EqEq
                } else {
                    Token::Eq
                }
            }
            '!' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    Token::NotEq
                } else {
                    return Err(anyhow::anyhow!("Carácter no válido después de !"));
                }
            }
            '<' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    Token::LtEq
                } else {
                    Token::Lt
                }
            }
            '>' => {
                self.read_char();
                if self.ch == '=' {
                    self.read_char();
                    Token::GtEq
                } else {
                    Token::Gt
                }
            }
            '+' => {
                self.read_char();
                Token::Plus
            }
            '-' => {
                self.read_char();
                Token::Minus
            }
            '*' => {
                self.read_char();
                Token::Star
            }
            '/' => {
                self.read_char();
                Token::Slash
            }
            ';' => {
                self.read_char();
                Token::Semicolon
            }
            ',' => {
                self.read_char();
                Token::Comma
            }
            ':' => {
                self.read_char();
                Token::Colon
            }
            '(' => {
                self.read_char();
                Token::LParen
            }
            ')' => {
                self.read_char();
                Token::RParen
            }
            '{' => {
                self.read_char();
                Token::LBrace
            }
            '}' => {
                self.read_char();
                Token::RBrace
            }
            '[' => {
                self.read_char();
                Token::LBracket
            }
            ']' => {
                self.read_char();
                Token::RBracket
            }
            '"' => {
                Token::String(self.read_string())
            }
            '\0' => Token::Eof,
            c if c.is_alphabetic() || c == '_' => {
                let ident = self.read_identifier();
                match ident.as_str() {
                    "let" => Token::Let,
                    "fn" => Token::Fn,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "while" => Token::While,
                    "for" => Token::For,
                    "return" => Token::Return,
                    "true" => Token::True,
                    "false" => Token::False,
                    "print" => Token::Print,
                    "int" => Token::Ident("int".to_string()),
                    "bool" => Token::Ident("bool".to_string()),
                    "string" => Token::Ident("string".to_string()),
                    "void" => Token::Ident("void".to_string()),
                    _ => Token::Ident(ident),
                }
            }
            c if c.is_digit(10) => Token::Number(self.read_number()),
            _ => return Err(anyhow::anyhow!("Carácter no válido: {}", c)),
        };

        Ok(tok)
    }
}
