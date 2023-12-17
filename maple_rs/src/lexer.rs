use crate::error::{LexerError, MapleError, ParserError};

trait GetFromIndex<Ret> {
    fn at(&self, i: usize, line: usize) -> Result<Ret, Box<dyn MapleError>>;
}
impl GetFromIndex<char> for Vec<String> {
    fn at(&self, i: usize, line: usize) -> Result<char, Box<dyn MapleError>> {
        if line >= self.len() {
            Err(Box::new(LexerError::new(
                "Index out of bounds".to_string() + &line.to_string(),
                line + 1,
                Token {
                    t: TokenType::EOF,
                    line: line + 1,
                    char_start: 0,
                    char_end: 0,
                },
            )))
        } else if i >= self[line].len() {
            Err(Box::new(LexerError::new(
                "Index out of bounds".to_string() + &i.to_string(),
                line + 1,
                Token {
                    t: TokenType::EOF,
                    line: line + 1,
                    char_start: 0,
                    char_end: 0,
                },
            )))
        } else {
            Ok(self[line].as_bytes()[i] as char)
        }
    }
}
impl GetFromIndex<char> for String {
    fn at(&self, i: usize, line: usize) -> Result<char, Box<dyn MapleError>> {
        if i >= self.len() {
            Err(Box::new(LexerError::new(
                "Index out of bounds".to_string() + &i.to_string(),
                line + 1,
                Token {
                    t: TokenType::EOF,
                    line: line + 1,
                    char_start: 0,
                    char_end: 0,
                },
            )))
        } else {
            Ok(self.as_bytes()[i] as char)
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    Import(String),
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
    OpTimes,
    OpDiv,
    EOF,
    LeftBrace,
    RightBrace,
    LeftParen,
    RightParen,
    LeftSquare,
    RightSquare,
    Comma,
    Comment(String),
}
impl TokenType {}
#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    pub t: TokenType,
    pub line: usize,
    pub char_start: usize,
    pub char_end: usize,
}
impl Token {
    pub fn get_op_prec(&self, lexer: &Lexer) -> Result<i32, Box<dyn MapleError>> {
        match self.t {
            TokenType::OpLt => Ok(9),
            TokenType::OpGt => Ok(9),
            TokenType::OpLtEq => Ok(9),
            TokenType::OpGtEq => Ok(9),
            TokenType::OpAndAnd => Ok(14),
            TokenType::OpOrOr => Ok(15),
            TokenType::OpEq => Ok(16),
            TokenType::OpPlsEq => Ok(16),
            TokenType::OpEqEq => Ok(10),
            TokenType::OpNotEq => Ok(10),
            TokenType::OpPls => Ok(6),
            TokenType::OpMns => Ok(6),
            TokenType::OpTimes => Ok(5),
            TokenType::OpDiv => Ok(5),
            _ => Err(Box::new(ParserError::new(
                format!("Unknown operator: {:?}", self).into(),
                lexer.get_line(),
                self.clone(),
            ))),
        }
    }
    pub fn get_unary_op_prec(&self, lexer: &Lexer) -> Result<i32, Box<dyn MapleError>> {
        match self.t {
            TokenType::OpMns => Ok(3),
            TokenType::OpNot => Ok(3),
            _ => Err(Box::new(ParserError::new(
                format!("Unknown unary operator: {:?}", self).into(),
                lexer.get_line(),
                self.clone(),
            ))),
        }
    }
    pub fn get_op_assoc(&self, lexer: &Lexer) -> Result<Assoc, Box<dyn MapleError>> {
        match self.t {
            TokenType::OpLt => Ok(Assoc::Left),
            TokenType::OpGt => Ok(Assoc::Left),
            TokenType::OpLtEq => Ok(Assoc::Left),
            TokenType::OpGtEq => Ok(Assoc::Left),
            TokenType::OpAndAnd => Ok(Assoc::Left),
            TokenType::OpOrOr => Ok(Assoc::Left),
            TokenType::OpEq => Ok(Assoc::Right),
            TokenType::OpPlsEq => Ok(Assoc::Right),
            TokenType::OpEqEq => Ok(Assoc::Left),
            TokenType::OpNotEq => Ok(Assoc::Left),
            TokenType::OpPls => Ok(Assoc::Left),
            TokenType::OpMns => Ok(Assoc::Left),
            TokenType::OpTimes => Ok(Assoc::Left),
            TokenType::OpDiv => Ok(Assoc::Left),
            _ => Err(Box::new(ParserError::new(
                format!("Unknown operator: {:?}", self).into(),
                lexer.get_line(),
                self.clone(),
            ))),
        }
    }
    pub fn is_op(&self) -> bool {
        match self.t {
            TokenType::OpLt
            | TokenType::OpGt
            | TokenType::OpLtEq
            | TokenType::OpGtEq
            | TokenType::OpPlsEq
            | TokenType::OpAndAnd
            | TokenType::OpOrOr
            | TokenType::OpEq
            | TokenType::OpNotEq
            | TokenType::OpEqEq
            | TokenType::OpMns
            | TokenType::OpTimes
            | TokenType::OpDiv
            | TokenType::OpPls => true,
            _ => false,
        }
    }
    pub fn is_unary_prefix_op(&self) -> bool {
        match self.t {
            TokenType::OpMns => true,
            TokenType::OpNot => true,
            _ => false,
        }
    }
    pub fn is_unary_postfix_op(&self) -> bool {
        match self.t {
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
    input: Vec<String>,
    current_token: Token,
    feed_tokens: Vec<Token>,
    pub comments: Vec<Token>,
}
impl Lexer {
    pub fn new(input: String) -> Lexer {
        let input: Vec<String> = input.split('\n').map(|s| s.to_string() + "\n").collect();
        // let mut new_end = input[input.len() - 1].to_owned();
        // new_end.pop();
        // let i = input.len() - 1;
        // input[i] = new_end;
        // if input[i].len() <= 1 {
        //     input.pop();
        // }
        Lexer {
            i: 0,
            line: 0,
            input,
            current_token: Token {
                t: TokenType::EOF,
                line: 0,
                char_start: 0,
                char_end: 0,
            },
            feed_tokens: Vec::new(),
            comments: Vec::new(),
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
            && self.i + 1 < self.input.len()
            && self.input.at(self.i + 1, self.line)? != '.';
        let mut char_number = 0;
        let mut integer_base = 10;
        let mut err = Option::None;
        let mut decimal_count = 0;
        while self.i < self.input[self.line].len() {
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
                        self.get_line(),
                        Token {
                            t: TokenType::Ident(
                                self.input[self.line][start_index..self.i].to_string(),
                            ),
                            line: self.line,
                            char_start: start_index,
                            char_end: self.i,
                        },
                    )));
                }
                '.' if !starts_with_zero => {
                    decimal_count += 1;
                }
                '.' if starts_with_zero => {
                    err = Some(Box::new(LexerError::new(
                        "Integer \"\" starting with 0 cannot have decimal point".to_string(),
                        self.get_line(),
                        Token {
                            t: TokenType::Ident(
                                self.input[self.line][start_index..self.i].to_string(),
                            ),
                            line: self.line,
                            char_start: start_index,
                            char_end: self.i,
                        },
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
                        self.get_line(),
                        Token {
                            t: TokenType::Ident(
                                self.input[self.line][start_index..self.i].to_string(),
                            ),
                            line: self.line,
                            char_start: start_index,
                            char_end: self.i,
                        },
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
                    .replace("\"\"", &self.input[self.line][start_index..self.i]),
            );
            return Err(err);
        }
        if integer_base != 10 {
            let start_index = start_index + 2;
            if self.i - start_index == 0 {
                return Err(Box::new(LexerError::new(
                    format!("Integer number has no digits"),
                    self.get_line(),
                    Token {
                        t: TokenType::Ident(self.input[self.line][start_index..self.i].to_string()),
                        line: self.line,
                        char_start: start_index,
                        char_end: self.i,
                    },
                )));
            }
            number = match i32::from_str_radix(
                &self.input[self.line][start_index..self.i],
                integer_base,
            ) {
                Ok(n) => n as f64,
                Err(_) => {
                    return Err(Box::new(LexerError::new(
                        format!(
                            "Invalid integer \"{}\" with base {}",
                            &self.input[self.line][start_index..self.i],
                            integer_base
                        ),
                        self.get_line(),
                        Token {
                            t: TokenType::Ident(
                                self.input[self.line][start_index..self.i].to_string(),
                            ),
                            line: self.line,
                            char_start: start_index,
                            char_end: self.i,
                        },
                    )))
                }
            };
            self.i += 1;
        } else {
            number = match self.input[self.line][start_index..self.i].parse::<f64>() {
                Ok(n) => n,
                Err(_) => {
                    return Err(Box::new(LexerError::new(
                        "Invalid number".to_string() + &self.input[self.line][start_index..self.i],
                        self.get_line(),
                        Token {
                            t: TokenType::Ident(
                                self.input[self.line][start_index..self.i].to_string(),
                            ),
                            line: self.line,
                            char_start: start_index,
                            char_end: self.i,
                        },
                    )))
                }
            };
        }
        Ok(Token {
            t: TokenType::Number(number),
            line: self.line,
            char_start: start_index,
            char_end: self.i,
        })
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
                lexer.get_line(),
                Token {
                    t: TokenType::Ident(format!("\\{}", c)),
                    line: lexer.line,
                    char_start: lexer.i,
                    char_end: lexer.i + 1,
                },
            ))),
        }
    }
    fn get_char(&mut self) -> Result<Token, Box<dyn MapleError>> {
        // eat the first '
        self.i += 1;
        let start_index = self.i;

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
                    self.get_line(),
                    Token {
                        t: TokenType::Ident("'".to_string()),
                        line: self.line - 1,
                        char_start: start_index,
                        char_end: self.i - 1,
                    },
                )));
            }
            c => c,
        };

        if self.input.at(self.i + 1, self.line)? != '\'' {
            return Err(Box::new(LexerError::new(
                "Unterminated or overlong character literal: ".to_string() + &c.to_string(),
                self.get_line(),
                Token {
                    t: TokenType::Ident("'".to_string()),
                    line: self.line,
                    char_start: start_index,
                    char_end: self.i,
                },
            )));
        }
        self.i += 2;
        Ok(Token {
            t: TokenType::Char(c),
            line: self.line,
            char_start: start_index,
            char_end: self.i,
        })
    }
    pub fn feed_token(&mut self, token: Token) {
        self.feed_tokens.push(token);
    }
    fn get_string(&mut self) -> Result<Token, Box<dyn MapleError>> {
        let start_index = self.i;
        self.i += 1;
        let mut string = String::new();
        while self.i < self.input[self.line].len() {
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
                    return Ok(Token {
                        t: TokenType::String(string),
                        line: self.line,
                        char_start: start_index,
                        char_end: self.i,
                    });
                }
                '\n' => {
                    self.line += 1;
                    self.i += 1;
                    return Err(Box::new(LexerError::new(
                        "Newline in string literal".to_string(),
                        self.get_line(),
                        Token {
                            t: TokenType::Ident("\"".to_string()),
                            line: self.line - 1,
                            char_start: start_index,
                            char_end: self.i - 1,
                        },
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
            self.get_line(),
            Token {
                t: TokenType::Ident("\"".to_string()),
                line: self.line,
                char_start: start_index,
                char_end: self.i,
            },
        )))
    }
    fn read_ident(&mut self) -> Result<Token, Box<dyn MapleError>> {
        let ident;
        let start_index = self.i;
        while self.i < self.input[self.line].len() {
            match self.input.at(self.i, self.line)? {
                'a'..='z' | 'A'..='Z' | '_' | '0'..='9' => {
                    self.i += 1;
                }
                _ => break,
            }
        }
        ident = self.input[self.line][start_index..self.i].to_string();
        let t = match ident.as_str() {
            "while" => TokenType::While,
            "const" => TokenType::Const,
            "if" => TokenType::If,
            "else" => TokenType::Else,
            "elseif" => TokenType::Elseif,
            "return" => TokenType::Return,
            "break" => TokenType::Break,
            "continue" => TokenType::Continue,
            "var" => TokenType::Var,
            "fn" => TokenType::Fn,
            "true" => TokenType::True,
            "false" => TokenType::False,
            "import" => {
                if self.input.at(self.i, self.line)? == ' ' {
                    self.i += 1;
                    let mut import = String::new();
                    while self.i < self.input[self.line].len() {
                        match self.input.at(self.i, self.line)? {
                            'a'..='z' | 'A'..='Z' | '_' | '0'..='9' | '/' | '.' => {
                                import.push(self.input.at(self.i, self.line)?);
                                self.i += 1;
                            }
                            _ => break,
                        }
                    }
                    if import.len() == 0 {
                        return Err(Box::new(LexerError::new(
                            "Invalid import statement (no path specified)\nnote: allowed characters are alphanumeric, '_', '/', and '.'".to_string(),
                            self.get_line(),
                            Token {
                                t: TokenType::Ident("import".to_string()),
                                line: self.line,
                                char_start: start_index,
                                char_end: self.i,
                            },
                        )));
                    }
                    TokenType::Import(import)
                } else {
                    return Err(Box::new(LexerError::new(
                        "Invalid import statement (a space must be placed after the word import)"
                            .to_string(),
                        self.get_line(),
                        Token {
                            t: TokenType::Ident("import".to_string()),
                            line: self.line,
                            char_start: start_index,
                            char_end: self.i,
                        },
                    )));
                }
            }
            _ => TokenType::Ident(ident),
        };
        Ok(Token {
            t,
            line: self.line,
            char_start: start_index,
            char_end: self.i,
        })
    }
    fn peek_next_char(&self) -> char {
        if self.i < self.input[self.line].len() - 1 {
            self.input.at(self.i + 1, self.line).unwrap()
        } else {
            if self.line < self.input.len() {
                self.input.at(0, self.line + 1).unwrap()
            } else {
                '\0'
            }
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
    fn single_char_token(&mut self, t: TokenType) -> Token {
        self.i += 1;
        Token {
            t,
            line: self.line,
            char_start: self.i - 1,
            char_end: self.i,
        }
    }
    fn char_token(&mut self, t: TokenType, w: usize) -> Token {
        self.i += w;
        Token {
            t,
            line: self.line,
            char_start: self.i - w,
            char_end: self.i,
        }
    }
    pub fn get_next_token(&mut self) -> Result<Token, Box<dyn MapleError>> {
        if self.feed_tokens.len() > 0 {
            self.current_token = self.feed_tokens.remove(0);
            return Ok(self.current_token.clone());
        }
        if self.line >= self.input.len()
            || (self.line == self.input.len() - 1 && self.i >= self.input[self.line].len())
        {
            self.current_token = {
                Token {
                    t: TokenType::EOF,
                    line: self.line,
                    char_start: self.i,
                    char_end: self.i,
                }
            };
            Ok(self.current_token.clone())
        } else {
            let current_token = match self.input.at(self.i, self.line)? {
                'a'..='z' | 'A'..='Z' | '_' => self.read_ident()?,
                '0'..='9' => self.get_number()?,
                '\'' => self.get_char()?,
                '"' => self.get_string()?,
                '\n' => {
                    let token = self.single_char_token(TokenType::EndOfStatement);
                    self.line += 1;
                    self.i = 0;
                    token
                }
                ' ' | '\t' | '\r' => {
                    self.i += 1;
                    self.get_next_token()?
                }
                '.' => self.single_char_token(TokenType::Dot),
                ',' => self.single_char_token(TokenType::Comma),
                '!' if self.peek_next_char() == '=' => self.char_token(TokenType::OpNotEq, 2),
                '!' => self.single_char_token(TokenType::OpNot),
                '*' => self.single_char_token(TokenType::OpTimes),
                '=' if self.peek_next_char() == '=' => self.char_token(TokenType::OpEqEq, 2),
                '=' => self.single_char_token(TokenType::OpEq),
                '+' if self.peek_next_char() == '=' => self.char_token(TokenType::OpPlsEq, 2),
                '+' => self.single_char_token(TokenType::OpPls),
                '&' if self.peek_next_char() == '&' => self.char_token(TokenType::OpAndAnd, 2),
                '|' if self.peek_next_char() == '|' => self.char_token(TokenType::OpOrOr, 2),
                '<' if self.peek_next_char() == '=' => self.char_token(TokenType::OpLtEq, 2),
                '<' => self.single_char_token(TokenType::OpLt),
                '>' if self.peek_next_char() == '=' => self.char_token(TokenType::OpGtEq, 2),
                '>' => self.single_char_token(TokenType::OpGt),
                '-' => self.single_char_token(TokenType::OpMns),

                '{' => self.single_char_token(TokenType::LeftBrace),
                '}' => self.single_char_token(TokenType::RightBrace),
                '(' => self.single_char_token(TokenType::LeftParen),
                ')' => self.single_char_token(TokenType::RightParen),
                '[' => self.single_char_token(TokenType::LeftSquare),
                ']' => self.single_char_token(TokenType::RightSquare),
                '/' if self.peek_next_char() == '/' => {
                    let start_i = self.i;
                    while self.i < self.input[self.line].len() && self.peek_next_char() != '\n' {
                        // comment.push(self.input.at(self.i, self.line)?);
                        self.i += 1;
                    }
                    self.i += 1;
                    let comment = self.input[self.line][start_i..self.i].to_string();
                    self.comments.push(Token {
                        t: TokenType::Comment(comment),
                        line: self.line,
                        char_start: start_i,
                        char_end: self.i,
                    });
                    self.get_next_token()?
                }
                '/' => self.single_char_token(TokenType::OpDiv),
                _ => {
                    return Err(Box::new(LexerError::new(
                        format!("Invalid character: {}", self.input.at(self.i, self.line)?),
                        self.get_line(),
                        Token {
                            t: TokenType::Ident(self.input.at(self.i, self.line)?.to_string()),
                            line: self.line,
                            char_start: self.i,
                            char_end: self.i + 1,
                        },
                    )))
                }
            };
            // println!("Current token: {:?}", current_token);
            self.current_token = current_token.clone();
            Ok(current_token)
        }
    }
    pub fn get_line(&self) -> usize {
        self.line + 1
    }
}
#[cfg(test)]
mod test {
    use crate::lexer::{Lexer, TokenType};

    // use super::Token;

    fn expect_tokens(contents: String, tokens: Vec<TokenType>) {
        let mut lexer = Lexer::new(contents);
        for token in tokens {
            let next_token = lexer.get_next_token().unwrap();
            println!("{:?} {:?}", token, next_token);
            assert_eq!(next_token.t, token);
            if token == TokenType::EOF {
                break;
            }
        }
    }
    #[test]
    fn test_lexer() {
        let contents: String = "var a_ = '\\\\''\\n'\nfn".to_string();

        let tokens: Vec<TokenType> = vec![
            TokenType::Var,
            TokenType::Ident("a_".to_string()),
            TokenType::OpEq,
            TokenType::Char('\\'),
            TokenType::Char('\n'),
            TokenType::EndOfStatement,
            TokenType::Fn,
            TokenType::EndOfStatement,
            TokenType::EOF,
        ];
        expect_tokens(contents, tokens);
    }

    #[test]
    fn test_variables() {
        let mut contents = "a_";
        let mut tokens: Vec<TokenType> = vec![
            TokenType::Ident("a_".to_string()),
            TokenType::EndOfStatement,
            TokenType::EOF,
        ];
        expect_tokens(contents.to_string(), tokens);
        contents = "a_1";
        tokens = vec![
            TokenType::Ident("a_1".to_string()),
            TokenType::EndOfStatement,
            TokenType::EOF,
        ];
        expect_tokens(contents.to_string(), tokens);
        contents = "a1";
        tokens = vec![
            TokenType::Ident("a1".to_string()),
            TokenType::EndOfStatement,
            TokenType::EOF,
        ];
        expect_tokens(contents.to_string(), tokens);
    }

    #[test]
    fn test_numbers() {
        let mut contents = "1";
        let mut tokens: Vec<TokenType> = vec![
            TokenType::Number(1.0),
            TokenType::EndOfStatement,
            TokenType::EOF,
        ];
        expect_tokens(contents.to_string(), tokens);
        contents = "1.0";
        tokens = vec![
            TokenType::Number(1.0),
            TokenType::EndOfStatement,
            TokenType::EOF,
        ];
        expect_tokens(contents.to_string(), tokens);

        contents = "1.01";
        tokens = vec![
            TokenType::Number(1.01),
            TokenType::EndOfStatement,
            TokenType::EOF,
        ];
        expect_tokens(contents.to_string(), tokens);
    }

    #[test]
    fn test_ops() {
        let mut contents = "==";
        let mut tokens: Vec<TokenType> =
            vec![TokenType::OpEqEq, TokenType::EndOfStatement, TokenType::EOF];
        expect_tokens(contents.to_string(), tokens);
        contents = "=";
        tokens = vec![TokenType::OpEq, TokenType::EndOfStatement, TokenType::EOF];
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
        let tokens: Vec<TokenType> = vec![
            TokenType::Const,
            TokenType::Ident("c".to_string()),
            TokenType::OpEq,
        ];
        for token in tokens {
            let next_token = lexer.get_next_token().unwrap();
            // println!("{:?} {:?}", token, next_token);
            assert_eq!(next_token.t, token);
        }
        assert!(lexer.get_next_token().is_err());
    }
}
