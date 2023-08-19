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
#[derive(Debug, PartialEq)]
pub enum Token {
    Number(i64),
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
}
#[derive(Debug)]
pub struct Lexer {
    i: usize,
    line: usize,
    input: String,
}
impl Lexer {
    pub fn new(input: String) -> Lexer {
        Lexer {
            i: 0,
            line: 1,
            input,
        }
    }

    fn get_number(&mut self) -> Result<Token, LexerError> {
        let number;
        let start_index = self.i;
        while self.i < self.input.len() {
            match self.input.at(self.i, self.line)? {
                '0'..='9' => {
                    self.i += 1;
                }
                _ => break,
            }
        }
        number = match self.input[start_index..self.i].parse::<i64>() {
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
    fn get_char(&mut self) -> Result<Token, LexerError> {
        Ok(Token::Char('a'))
    }
    fn get_string(&mut self) -> Result<Token, LexerError> {
        Ok(Token::String("".to_string()))
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
    pub fn get_next_token(&mut self) -> Result<Token, LexerError> {
        if self.i >= self.input.len() {
            Ok(Token::EOF)
        } else {
            match self.input.at(self.i, self.line)? {
                'a'..='z' | 'A'..='Z' | '_' => self.read_ident(),
                '0'..='9' => self.get_number(),
                '\'' => self.get_char(),
                '"' => self.get_string(),
                '\n' => {
                    self.line += 1;
                    Ok(Token::EndOfStatement)
                }
                ' ' | '\t' => {
                    self.i += 1;
                    self.get_next_token()
                }
                '=' => match self.peek_next_char() {
                    '=' => {
                        self.i += 2;
                        Ok(Token::OpEqEq)
                    }
                    _ => {
                        self.i += 1;
                        Ok(Token::OpEq)
                    }
                },
                _ => Err(LexerError::new(
                    self.line,
                    "Invalid character: ".to_string() + &self.input[self.i..self.i + 1],
                )),
            }
        }
    }
}
