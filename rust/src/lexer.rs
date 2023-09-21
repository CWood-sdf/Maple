#[derive(Debug, PartialEq)]
pub struct LexerError {
    line: usize,
    msg: String,
}
impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lexer error at line {}: {}", self.line, self.msg)
    }
}
impl std::error::Error for LexerError {}

impl LexerError {
    pub fn new(line: usize, msg: String) -> LexerError {
        LexerError { line, msg }
    }
}
trait GetFromIndex<Ret, Err = LexerError> {
    fn at(&self, i: usize, line: usize) -> Result<Ret, Err>;
}
impl GetFromIndex<char> for String {
    fn at(&self, i: usize, line: usize) -> Result<char, LexerError> {
        if i >= self.len() {
            Err(LexerError::new(
                line,
                "Index out of bounds".to_string() + &i.to_string(),
            ))
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
    Var,
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
    OpEqEq,
    EOF,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    OpPls,
}
impl Token {
    pub fn get_op_prec(&self) -> Result<i32, Box<dyn std::error::Error>> {
        match self {
            Token::OpEq => Ok(16),
            Token::OpEqEq => Ok(10),
            Token::OpPls => Ok(6),
            _ => Err(format!("Unknown operator: {:?}", self).into()),
        }
    }
    pub fn get_op_assoc(&self) -> Result<Assoc, Box<dyn std::error::Error>> {
        match self {
            Token::OpEq => Ok(Assoc::Right),
            Token::OpEqEq => Ok(Assoc::Left),
            Token::OpPls => Ok(Assoc::Left),
            _ => Err(format!("Unknown operator: {:?}", self).into()),
        }
    }
    pub fn is_op(&self) -> bool {
        match self {
            Token::OpEq | Token::OpEqEq | Token::OpPls => true,
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
}
impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            i: 0,
            line: 1,
            input,
            current_token: Token::EOF,
        }
    }
    pub fn get_current_token(&self) -> Token {
        self.current_token.clone()
    }
    fn get_number(&mut self) -> Result<Token, LexerError> {
        let number;
        let start_index = self.i;
        while self.i < self.input.len() {
            match self.input.at(self.i, self.line)? {
                '0'..='9' | '.' => {
                    self.i += 1;
                }
                _ => break,
            }
        }
        number = match self.input[start_index..self.i].parse::<f64>() {
            Ok(n) => n,
            Err(_) => {
                return Err(LexerError::new(
                    self.line,
                    "Invalid number".to_string() + &self.input[start_index..self.i],
                ))
            }
        };
        Ok(Token::Number(number))
    }
    fn get_special_char(c: char) -> Result<char, LexerError> {
        match c {
            'n' => Ok('\n'),
            't' => Ok('\t'),
            'r' => Ok('\r'),
            '0' => Ok('\0'),
            '\\' => Ok('\\'),
            _ => Err(LexerError::new(
                0,
                "Invalid special character: ".to_string() + &c.to_string(),
            )),
        }
    }
    fn get_char(&mut self) -> Result<Token, LexerError> {
        self.i += 1;

        let c = match self.input.at(self.i, self.line)? {
            '\\' => {
                self.i += 1;
                Self::get_special_char(self.input.at(self.i, self.line)?)?
            }
            '\n' => {
                self.line += 1;
                self.i += 1;
                return Err(LexerError::new(
                    self.line,
                    "Newline in character literal".to_string(),
                ));
            }
            c => c,
        };

        if self.input.at(self.i + 1, self.line)? != '\'' {
            return Err(LexerError::new(
                self.line,
                "Unterminated or overlong character literal: ".to_string() + &c.to_string(),
            ));
        }
        self.i += 2;
        Ok(Token::Char(c))
    }
    fn get_string(&mut self) -> Result<Token, LexerError> {
        self.i += 1;
        let mut string = String::new();
        while self.i < self.input.len() {
            match self.input.at(self.i, self.line)? {
                '\\' => {
                    self.i += 2;
                    string.push(Self::get_special_char(self.input.at(self.i, self.line)?)?);
                }
                '"' => {
                    self.i += 1;
                    return Ok(Token::String(string));
                }
                '\n' => {
                    self.line += 1;
                    self.i += 1;
                    return Err(LexerError::new(
                        self.line,
                        "Newline in string literal".to_string(),
                    ));
                }
                c => {
                    string.push(c);
                    self.i += 1;
                }
            }
        }
        Err(LexerError::new(
            self.line,
            "Unterminated string literal".to_string(),
        ))
    }
    fn read_ident(&mut self) -> Result<Token, LexerError> {
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
        match ident.as_str() {
            "while" => Ok(Token::While),
            "if" => Ok(Token::If),
            "else" => Ok(Token::Else),
            "elseif" => Ok(Token::Elseif),
            "return" => Ok(Token::Return),
            "break" => Ok(Token::Break),
            "continue" => Ok(Token::Continue),
            "var" => Ok(Token::Var),
            "fn" => Ok(Token::Fn),
            _ => Ok(Token::Ident(ident)),
        }
    }
    fn peek_next_char(&self) -> char {
        if self.i + 1 >= self.input.len() {
            '\0'
        } else {
            self.input.as_bytes()[self.i + 1] as char
        }
    }
    pub fn peek_next_token(&mut self) -> Result<Token, LexerError> {
        let i = self.i;
        let line = self.line;
        let token = self.get_next_token();
        self.i = i;
        self.line = line;
        token
    }
    pub fn get_next_token(&mut self) -> Result<Token, LexerError> {
        if self.i >= self.input.len() {
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
                ' ' | '\t' => {
                    self.i += 1;
                    self.get_next_token()?
                }
                '=' if self.peek_next_char() == '=' => {
                    self.i += 2;
                    Token::OpEqEq
                }
                '=' => {
                    self.i += 1;
                    Token::OpEq
                }
                '+' => {
                    self.i += 1;
                    Token::OpPls
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
                _ => {
                    return Err(LexerError::new(
                        self.line,
                        "Invalid character: ".to_string() + &self.input[self.i..self.i + 1],
                    ))
                }
            };
            self.current_token = current_token.clone();
            Ok(current_token)
        }
    }
}
#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, Token};

    fn expect_tokens(contents: String, tokens: Vec<Token>) {
        let mut lexer = Lexer::new(contents);
        for token in tokens {
            let next_token = lexer.get_next_token().unwrap();
            println!("{:?} {:?}", token, next_token);
            assert_eq!(next_token, token);
        }
    }
    #[test]
    fn test_lexer() {
        let contents: String = "var a_ = '\\\\''\n'\nfn".to_string();

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
}
