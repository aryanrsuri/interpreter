#[derive(Debug, PartialEq)]
pub enum Token {
    Illegal,
    Eof,
    Ident(String),
    Int(String),
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Fslash,
    Lt,
    Gt,
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    Function,
    Let,
    True,
    False,
    If,
    Else,
    Return,
    Equal,
    NotEqual,
}

pub struct Lexer {
    pub input: Vec<u8>,
    pub position: usize,
    pub peek: usize,
    pub ch: u8,
}

impl Lexer {
    pub fn new(input: String) -> Lexer {
        let mut l = Lexer {
            input: input.into_bytes(),
            position: 0,
            peek: 0,
            ch: 0,
        };
        l.read_char();
        l
    }

    fn read_char(&mut self) {
        if self.peek >= self.input.len() {
            self.ch = 0
        } else {
            self.ch = self.input[self.peek];
        }
        self.position = self.peek;
        self.peek += 1;
    }

    fn peek_char(&mut self) -> u8 {
        if self.peek >= self.input.len() {
            0
        } else {
            self.input[self.peek]
        }
    }

    fn read_ident(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_alphabetic() || self.ch == b'_' {
            self.read_char()
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn read_int(&mut self) -> String {
        let position = self.position;
        while self.ch.is_ascii_digit() {
            self.read_char()
        }
        String::from_utf8_lossy(&self.input[position..self.position]).to_string()
    }

    fn read_whitespace(&mut self) {
        if self.ch.is_ascii_whitespace() {
            self.read_char()
        }
    }

    pub fn consume(&mut self) -> Option<Token> {
        self.read_whitespace();
        let token = match self.ch {
            b'=' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::Equal
                } else {
                    Token::Assign
                }
            }
            b'+' => Token::Plus,
            b'-' => Token::Minus,
            b'!' => {
                if self.peek_char() == b'=' {
                    self.read_char();
                    Token::NotEqual
                } else {
                    Token::Bang
                }
            }
            b'*' => Token::Asterisk,
            b'/' => Token::Fslash,
            b'<' => Token::Lt,
            b'>' => Token::Gt,
            b';' => Token::Semicolon,
            b'(' => Token::Lparen,
            b')' => Token::Rparen,
            b'{' => Token::Lbrace,
            b'}' => Token::Rbrace,
            b',' => Token::Comma,
            b'a'..=b'z' | b'A'..=b'Z' | b'_' => {
                let identifier = self.read_ident();
                return Some(match identifier.as_str() {
                    "fn" => Token::Function,
                    "true" => Token::True,
                    "false" => Token::False,
                    "let" => Token::Let,
                    "if" => Token::If,
                    "else" => Token::Else,
                    "return" => Token::Return,
                    _ => Token::Ident(identifier),
                });
            }
            b'0'..=b'9' => return Some(Token::Int(self.read_int())),
            0 => Token::Eof,
            _ => Token::Illegal,
        };
        self.read_char();
        Some(token)
    }
}

#[cfg(test)]
mod test {
    use super::{Lexer, Token};

    #[test]
    fn test_next() {
        let input = "=+();";
        let mut lexer = Lexer::new(input.into());
        let tokens = vec![
            Token::Assign,
            Token::Plus,
            Token::Lparen,
            Token::Rparen,
            Token::Semicolon,
        ];

        for t in tokens {
            let next = lexer.consume().unwrap();
            println!("Expected {:?}, got {:?}", t, next);
            assert_eq!(t, next)
        }
    }
}
