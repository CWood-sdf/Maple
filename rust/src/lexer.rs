use crate::error::{LexerError, MapleError, ParserError};

trait GetFromIndex<Ret> {
    fn at(&self, i: usize, line: usize) -> Result<Ret, Box<dyn MapleError>>;
}
impl GetFromIndex<char> for String {
    fn at(&self, i: usize, line: usize) -> Result<char, Box<dyn MapleError>> {
        if i >= self.len() {
            Err(Box::new(LexerError::new(
                "Index out of bounds".to_string() + &i.to_string(),
                line,
            )))
        } else {
            Ok(self.as_bytes()[i] as char)
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Number(f64),
    Char(char),
    String(String),
    Ident(String),
    Dot,
    True,
    False,
    Var,
    Const,
    Fn,
    If,
    Else,
    While,
    Elseif,
    Return,
    Break,
    Continue,
    EndOfStatement,
    OpEq,
    OpNot,
    OpNotEq,
    OpEqEq,
    OpPls,
    OpPlsEq,
    OpLt,
    OpGt,
    OpLtEq,
    OpGtEq,
    OpAndAnd,
    OpOrOr,
    OpMns,
    EOF,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    Comma,
}
impl Token {
    pub fn get_op_prec(&self, lexer: &Lexer) -> Result<i32, Box<dyn MapleError>> {
        match self {
            Token::OpLt => Ok(9),
            Token::OpGt => Ok(9),
            Token::OpLtEq => Ok(9),
            Token::OpGtEq => Ok(9),
            Token::OpAndAnd => Ok(14),
            Token::OpOrOr => Ok(15),
            Token::OpEq => Ok(16),
            Token::OpPlsEq => Ok(16),
            Token::OpEqEq => Ok(10),
            Token::OpNotEq => Ok(10),
            Token::OpPls => Ok(6),
            Token::OpMns => Ok(6),
            _ => Err(Box::new(ParserError::new(
                format!("Unknown operator: {:?}", self).into(),
                lexer.line,
            ))),
        }
    }
    pub fn get_unary_op_prec(&self, lexer: &Lexer) -> Result<i32, Box<dyn MapleError>> {
        match self {
            Token::OpMns => Ok(3),
            Token::OpNot => Ok(3),
            _ => Err(Box::new(ParserError::new(
                format!("Unknown unary operator: {:?}", self).into(),
                lexer.line,
            ))),
        }
    }
    pub fn get_op_assoc(&self, lexer: &Lexer) -> Result<Assoc, Box<dyn MapleError>> {
        match self {
            Token::OpLt => Ok(Assoc::Left),
            Token::OpGt => Ok(Assoc::Left),
            Token::OpLtEq => Ok(Assoc::Left),
            Token::OpGtEq => Ok(Assoc::Left),
            Token::OpAndAnd => Ok(Assoc::Left),
            Token::OpOrOr => Ok(Assoc::Left),
            Token::OpEq => Ok(Assoc::Right),
            Token::OpPlsEq => Ok(Assoc::Right),
            Token::OpEqEq => Ok(Assoc::Left),
            Token::OpNotEq => Ok(Assoc::Left),
            Token::OpPls => Ok(Assoc::Left),
            Token::OpMns => Ok(Assoc::Left),
            _ => Err(Box::new(ParserError::new(
                format!("Unknown operator: {:?}", self).into(),
                lexer.line,
            ))),
        }
    }
    pub fn is_op(&self) -> bool {
        match self {
            Token::OpLt
            | Token::OpGt
            | Token::OpLtEq
            | Token::OpGtEq
            | Token::OpPlsEq
            | Token::OpAndAnd
            | Token::OpOrOr
            | Token::OpEq
            | Token::OpNotEq
            | Token::OpEqEq
            | Token::OpMns
            | Token::OpPls => true,
            _ => false,
        }
    }
    pub fn is_unary_prefix_op(&self) -> bool {
        match self {
            Token::OpMns => true,
            Token::OpNot => true,
            _ => false,
        }
    }
    pub fn is_unary_postfix_op(&self) -> bool {
        match self {
            _ => false,
        }
    }
}
#[derive(PartialEq)]
pub enum Assoc {
    Left,
    Right,
}
#[derive(Debug)]
pub struct Lexer {
    i: usize,
    line: usize,
    input: String,
    current_token: Token,
    feed_tokens: Vec<Token>,
}
impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            i: 0,
            line: 1,
            input,
            current_token: Token::EOF,
            feed_tokens: Vec::new(),
        }
    }
    pub fn get_current_token(&self) -> Token {
        // println!("Current token: {:?}", self.current_token);
        self.current_token.clone()
    }
    fn get_number(&mut self) -> Result<Token, Box<dyn MapleError>> {
        let number;
        let start_index = self.i;
        let starts_with_zero = self.input.at(self.i, self.line)? == '0'
            && self.input.at(self.i + 1, self.line)? != '.';
        let mut char_number = 0;
        let mut integer_base = 10;
        let mut err = Option::None;
        let mut decimal_count = 0;
        while self.i < self.input.len() {
            match self.input.at(self.i, self.line)? {
                'b' if starts_with_zero && char_number == 1 => {
                    integer_base = 2;
                }
                'x' if starts_with_zero && char_number == 1 => {
                    integer_base = 16;
                }
                'o' if starts_with_zero && char_number == 1 => {
                    integer_base = 8;
                }
                '.' if !starts_with_zero && decimal_count > 0 => {
                    err = Some(Box::new(LexerError::new(
                        "Invalid number \"\", note: number already has decimal point".to_string(),
                        self.line,
                    )));
                }
                '.' if !starts_with_zero => {
                    decimal_count += 1;
                }
                '.' if starts_with_zero => {
                    err = Some(Box::new(LexerError::new(
                        "Integer \"\" starting with 0 cannot have decimal point".to_string(),
                        self.line,
                    )));
                }
                '0'..='1' if integer_base == 2 => {}
                '0'..='7' if integer_base == 8 => {}
                '0'..='9' if integer_base == 10 => {}
                '0'..='9' | 'a'..='f' | 'A'..='F' if integer_base == 16 => {}
                '0'..='9' | 'a'..='z' | 'A'..='Z' | '_' => {
                    err = Some(Box::new(LexerError::new(
                        format!(
                            "Invalid number \"\", note: number is in base {}",
                            integer_base
                        ),
                        self.line,
                    )));
                }
                _ => break,
            };
            self.i += 1;
            char_number += 1;
        }
        if err.is_some() {
            let mut err = err.unwrap();
            err.set_msg(
                err.get_msg()
                    .replace("\"\"", &self.input[start_index..self.i]),
            );
            return Err(err);
        }
        if integer_base != 10 {
            let start_index = start_index + 2;
            if self.i - start_index == 0 {
                return Err(Box::new(LexerError::new(
                    format!("Integer number has no digits"),
                    self.line,
                )));
            }
            number = match i32::from_str_radix(&self.input[start_index..self.i], integer_base) {
                Ok(n) => n as f64,
                Err(_) => {
                    return Err(Box::new(LexerError::new(
                        format!(
                            "Invalid integer \"{}\" with base {}",
                            &self.input[start_index..self.i],
                            integer_base
                        ),
                        self.line,
                    )))
                }
            };
            self.i += 1;
        } else {
            number = match self.input[start_index..self.i].parse::<f64>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(Box::new(LexerError::new(
                        "Invalid number".to_string() + &self.input[start_index..self.i],
                        self.line,
                    )))
                }
            };
        }
        Ok(Token::Number(number))
    }
    fn get_special_char(c: char, lexer: &Lexer) -> Result<char, Box<dyn MapleError>> {
        match c {
            'n' => Ok('\n'),
            't' => Ok('\t'),
            'r' => Ok('\r'),
            '0' => Ok('\0'),
            '\\' => Ok('\\'),
            _ => Err(Box::new(LexerError::new(
                format!("Invalid special character: '\\{}'", &c.to_string()),
                lexer.line,
            ))),
        }
    }
    fn get_char(&mut self) -> Result<Token, Box<dyn MapleError>> {
        // eat the first '
        self.i += 1;

        let c = match self.input.at(self.i, self.line)? {
            '\\' => {
                // eat
                self.i += 1;
                Self::get_special_char(self.input.at(self.i, self.line)?, self)?
            }
            '\n' => {
                // eat
                self.line += 1;
                self.i += 1;
                return Err(Box::new(LexerError::new(
                    "Newline in character literal".to_string(),
                    self.line,
                )));
            }
            c => c,
        };

        if self.input.at(self.i + 1, self.line)? != '\'' {
            return Err(Box::new(LexerError::new(
                "Unterminated or overlong character literal: ".to_string() + &c.to_string(),
                self.line,
            )));
        }
        self.i += 2;
        Ok(Token::Char(c))
    }
    pub fn feed_token(&mut self, token: Token) {
        self.feed_tokens.push(token);
    }
    fn get_string(&mut self) -> Result<Token, Box<dyn MapleError>> {
        self.i += 1;
        let mut string = String::new();
        while self.i < self.input.len() {
            match self.input.at(self.i, self.line)? {
                '\\' => {
                    self.i += 2;
                    string.push(Self::get_special_char(
                        self.input.at(self.i - 1, self.line)?,
                        self,
                    )?);
                }
                '"' => {
                    self.i += 1;
                    return Ok(Token::String(string));
                }
                '\n' => {
                    self.line += 1;
                    self.i += 1;
                    return Err(Box::new(LexerError::new(
                        "Newline in string literal".to_string(),
                        self.line,
                    )));
                }
                c => {
                    string.push(c);
                    self.i += 1;
                }
            }
        }
        Err(Box::new(LexerError::new(
            "Unterminated string literal".to_string(),
            self.line,
        )))
    }
    fn read_ident(&mut self) -> Result<Token, Box<dyn MapleError>> {
        let ident;
        let start_index = self.i;
        while self.i < self.input.len() {
            match self.input.at(self.i, self.line)? {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    self.i += 1;
                }
                _ => break,
            }
        }
        ident = self.input[start_index..self.i].to_string();
        Ok(match ident.as_str() {
            "while" => Token::While,
            "const" => Token::Const,
            "if" => Token::If,
            "else" => Token::Else,
            "elseif" => Token::Elseif,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "var" => Token::Var,
            "fn" => Token::Fn,
            "true" => Token::True,
            "false" => Token::False,
            _ => Token::Ident(ident),
        })
    }
    fn peek_next_char(&self) -> char {
        if self.i + 1 >= self.input.len() {
            '\0'
        } else {
            self.input.as_bytes()[self.i + 1] as char
        }
    }
    pub fn peek_next_token(&mut self) -> Result<Token, Box<dyn MapleError>> {
        let i = self.i;
        let line = self.line;
        let current_token = self.current_token.clone();
        let token = self.get_next_token();
        self.i = i;
        self.line = line;
        self.current_token = current_token;
        token
    }
    pub fn get_next_token(&mut self) -> Result<Token, Box<dyn MapleError>> {
        if self.feed_tokens.len() > 0 {
            self.current_token = self.feed_tokens.remove(0);
            return Ok(self.current_token.clone());
        }
        if self.i >= self.input.len() {
            self.current_token = Token::EOF;
            Ok(Token::EOF)
        } else {
            let current_token = match self.input.at(self.i, self.line)? {
                'a'..='z' | 'A'..='Z' | '_' => self.read_ident()?,
                '0'..='9' => self.get_number()?,
                '\'' => self.get_char()?,
                '"' => self.get_string()?,
                '\n' => {
                    self.line += 1;
                    self.i += 1;
                    Token::EndOfStatement
                }
                ' ' | '\t' | '\r' => {
                    self.i += 1;
                    self.get_next_token()?
                }
                '.' => {
                    self.i += 1;
                    Token::Dot
                }
                ',' => {
                    self.i += 1;
                    Token::Comma
                }
                '!' if self.peek_next_char() == '=' => {
                    self.i += 2;
                    Token::OpNotEq
                }
                '!' => {
                    self.i += 1;
                    Token::OpNot
                }
                '=' if self.peek_next_char() == '=' => {
                    self.i += 2;
                    Token::OpEqEq
                }
                '=' => {
                    self.i += 1;
                    Token::OpEq
                }
                '+' if self.peek_next_char() == '=' => {
                    self.i += 2;
                    Token::OpPlsEq
                }
                '+' => {
                    self.i += 1;
                    Token::OpPls
                }
                '&' if self.peek_next_char() == '&' => {
                    self.i += 2;
                    Token::OpAndAnd
                }
                '|' if self.peek_next_char() == '|' => {
                    self.i += 2;
                    Token::OpOrOr
                }
                '<' if self.peek_next_char() == '=' => {
                    self.i += 2;
                    Token::OpLtEq
                }
                '<' => {
                    self.i += 1;
                    Token::OpLt
                }
                '>' if self.peek_next_char() == '=' => {
                    self.i += 2;
                    Token::OpGtEq
                }
                '>' => {
                    self.i += 1;
                    Token::OpGt
                }
                '-' => {
                    self.i += 1;
                    Token::OpMns
                }

                '{' => {
                    self.i += 1;
                    Token::LeftBrace
                }
                '}' => {
                    self.i += 1;
                    Token::RightBrace
                }
                '(' => {
                    self.i += 1;
                    Token::LeftParen
                }
                ')' => {
                    self.i += 1;
                    Token::RightParen
                }
                '[' => {
                    self.i += 1;
                    Token::LeftSquare
                }
                ']' => {
                    self.i += 1;
                    Token::RightSquare
                }
                '/' if self.peek_next_char() == '/' => {
                    while self.i < self.input.len() && self.peek_next_char() != '\n' {
                        self.i += 1;
                    }
                    self.i += 1;
                    Token::EndOfStatement
                }
                _ => {
                    return Err(Box::new(LexerError::new(
                        "Invalid character: ".to_string() + &self.input[self.i..self.i + 1],
                        self.line,
                    )))
                }
            };
            // println!("Current token: {:?}", current_token);
            self.current_token = current_token.clone();
            Ok(current_token)
        }
    }
    pub fn get_line(&self) -> usize {
        self.line
    }
}
#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, Token};

    fn expect_tokens(contents: String, tokens: Vec<Token>) {
        let mut lexer = Lexer::new(contents);
        for token in tokens {
            let next_token = lexer.get_next_token().unwrap();
            // println!("{:?} {:?}", token, next_token);
            assert_eq!(next_token, token);
        }
    }
    #[test]
    fn test_lexer() {
        let contents: String = "var a_ = '\\\\''\\n'\nfn".to_string();

        let tokens: Vec<Token> = vec![
            Token::Var,
            Token::Ident("a_".to_string()),
            Token::OpEq,
            Token::Char('\\'),
            Token::Char('\n'),
            Token::EndOfStatement,
            Token::Fn,
            Token::EOF,
        ];
        expect_tokens(contents, tokens);
    }

    #[test]
    fn test_variables() {
        let mut contents = "a_";
        let mut tokens: Vec<Token> = vec![Token::Ident("a_".to_string()), Token::EOF];
        expect_tokens(contents.to_string(), tokens);
        contents = "a_1";
        tokens = vec![Token::Ident("a_1".to_string()), Token::EOF];
        expect_tokens(contents.to_string(), tokens);
        contents = "a1";
        tokens = vec![Token::Ident("a1".to_string()), Token::EOF];
        expect_tokens(contents.to_string(), tokens);
    }

    #[test]
    fn test_numbers() {
        let mut contents = "1";
        let mut tokens: Vec<Token> = vec![Token::Number(1.0), Token::EOF];
        expect_tokens(contents.to_string(), tokens);
        contents = "1.0";
        tokens = vec![Token::Number(1.0), Token::EOF];
        expect_tokens(contents.to_string(), tokens);

        contents = "1.01";
        tokens = vec![Token::Number(1.01), Token::EOF];
        expect_tokens(contents.to_string(), tokens);
    }

    #[test]
    fn test_ops() {
        let mut contents = "==";
        let mut tokens: Vec<Token> = vec![Token::OpEqEq, Token::EOF];
        expect_tokens(contents.to_string(), tokens);
        contents = "=";
        tokens = vec![Token::OpEq, Token::EOF];
        expect_tokens(contents.to_string(), tokens);
    }

    #[test]
    fn test_chars() {
        let mut contents = "'a";
        let mut lexer = Lexer::new(contents.to_string());
        assert!(lexer.get_next_token().is_err());

        contents = "'a ";
        lexer = Lexer::new(contents.to_string());
        assert!(lexer.get_next_token().is_err());

        contents = "'a\n";
        lexer = Lexer::new(contents.to_string());
        assert!(lexer.get_next_token().is_err());

        contents = "const c = '0\n";
        lexer = Lexer::new(contents.to_string());
        let tokens: Vec<Token> = vec![Token::Const, Token::Ident("c".to_string()), Token::OpEq];
        for token in tokens {
            let next_token = lexer.get_next_token().unwrap();
            // println!("{:?} {:?}", token, next_token);
            assert_eq!(next_token, token);
        }
        assert!(lexer.get_next_token().is_err());
    }
}
