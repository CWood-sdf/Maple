// turn off dead code warnings
#![allow(dead_code)]
use std::rc::Rc;

use crate::error::{MapleError, ParserError, RuntimeError, ScopeError};

use crate::ast::{FunctionLiteral, IfLiteral, AST};
use crate::lexer::{Assoc, Lexer, Token};
use crate::scopechain::ScopeChain;

pub trait Unpack<T> {
    fn unpack(&self, scope_chain: &ScopeChain, line: usize) -> Result<T, ScopeError>;
    fn unpack_and_transform(
        &self,
        scope_chain: &ScopeChain,
        line: usize,
        ast: &AST,
    ) -> Result<Rc<Value>, Box<RuntimeError>>;
}
impl Unpack<Rc<Value>> for Rc<Value> {
    fn unpack(&self, scope_chain: &ScopeChain, line: usize) -> Result<Rc<Value>, ScopeError> {
        match self.as_ref() {
            Value::Variable(name) => {
                let value = scope_chain.get_variable(&name, line);
                match value {
                    Ok(value) => value.unpack(scope_chain, line),
                    Err(e) => Err(e),
                }
            }
            // Value::Undefined => Err("Unpacking an undefined value".into()),
            _ => Ok(self.clone()),
        }
    }
    fn unpack_and_transform(
        &self,
        scope_chain: &ScopeChain,
        line: usize,
        ast: &AST,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let value = self.unpack(scope_chain, line);
        match value {
            Ok(value) => Ok(value),
            Err(e) => Err(Box::new(e.to_runtime_error(ast.clone()))),
        }
    }
}
#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Variable(String),
    Char(char),
    Function(FunctionLiteral),
    BuiltinFunction(
        fn(Vec<Rc<Value>>, &AST, &ScopeChain, usize) -> Result<Rc<Value>, Box<RuntimeError>>,
        usize,
    ),
    Undefined,
}
impl Value {
    pub fn pretty_type(&self, scope_chain: &ScopeChain, line: usize) -> String {
        match self {
            Value::BuiltinFunction(_, count) => format!("builtin_function(<{}>)", count),
            Value::Function(_) => "function".to_string(),
            Value::String(_) => "string".to_string(),
            Value::Number(_) => "number".to_string(),
            Value::Boolean(_) => "boolean".to_string(),
            Value::Variable(name) => scope_chain
                .get_variable(&name, line)
                .unwrap()
                .pretty_type(scope_chain, line),
            Value::Char(_) => "char".to_string(),
            Value::Undefined => "undefined".to_string(),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
    pub name: String,
    pub value: Rc<Value>,
    pub is_const: bool,
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}
fn usable_operator(
    op: &Token,
    max_op_prec: i32,
    lexer: &Lexer,
) -> Result<bool, Box<dyn MapleError>> {
    if op.is_op() {
        if op.get_op_assoc(lexer)? == Assoc::Left {
            Ok(op.get_op_prec(lexer)? < max_op_prec)
        } else {
            Ok(op.get_op_prec(lexer)? <= max_op_prec)
        }
    } else {
        Err(Box::new(ParserError::new(
            "Did not pass in an operator to usable_operator".into(),
            lexer.get_line(),
        )))
    }
}
impl Parser {
    pub fn new(contents: String) -> Parser {
        let lexer = Lexer::new(contents);
        Parser { lexer }
    }
    fn parse_function(&mut self, anon: bool) -> Result<Box<AST>, Box<dyn MapleError>> {
        let mut params: Vec<String> = vec![];
        let body: Vec<Box<AST>>;
        match self.lexer.get_current_token() {
            Token::Fn => (),
            _ => {
                return Err(Box::new(ParserError::new(
                    format!("Expected fn, got {:?}", self.lexer.get_current_token()).into(),
                    self.lexer.get_line(),
                )))
            }
        };
        let name: String;
        if !anon {
            match self.lexer.get_next_token()? {
                Token::Ident(n) => {
                    name = n;
                }
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!(
                            "Expected left paren, got {:?}",
                            self.lexer.get_current_token()
                        ),
                        self.lexer.get_line(),
                    )))
                }
            };
        } else {
            name = "".to_string();
        }
        match self.lexer.get_next_token()? {
            Token::LeftParen => (),
            _ => {
                return Err(Box::new(ParserError::new(
                    format!(
                        "Expected left paren, got {:?}",
                        self.lexer.get_current_token()
                    ),
                    self.lexer.get_line(),
                )))
            }
        };
        loop {
            match self.lexer.get_next_token()? {
                Token::Ident(name) => params.push(name),
                Token::RightParen => break,
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!(
                            "Expected identifier or right paren, got {:?}",
                            self.lexer.get_current_token()
                        ),
                        self.lexer.get_line(),
                    )))
                }
            }
            match self.lexer.get_next_token()? {
                Token::RightParen => break,
                Token::Comma => (),
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!(
                            "Expected comma or right paren, got {:?}",
                            self.lexer.get_current_token()
                        ),
                        self.lexer.get_line(),
                    )))
                }
            }
        }
        self.lexer.get_next_token()?;
        body = self.parse_block()?;
        if !anon {
            Ok(Box::new(AST::OpEq(
                Box::new(AST::VariableDeclaration(
                    name.clone(),
                    true,
                    self.lexer.get_line(),
                )),
                Box::new(AST::FunctionLiteral(
                    FunctionLiteral::basic(params, body),
                    self.lexer.get_line(),
                )),
                self.lexer.get_line(),
            )))
        } else {
            Ok(Box::new(AST::FunctionLiteral(
                FunctionLiteral::basic(params, body),
                self.lexer.get_line(),
            )))
        }
    }
    fn parse_clause(&mut self, max_op_prec: i32) -> Result<Box<AST>, Box<dyn MapleError>> {
        let mut ret: Option<Box<AST>>;
        match self.lexer.get_current_token() {
            Token::Fn => ret = Some(self.parse_function(true)?),
            Token::Number(num) => {
                ret = Some(Box::new(AST::NumberLiteral(num, self.lexer.get_line())))
            }
            Token::String(str) => {
                ret = Some(Box::new(AST::StringLiteral(str, self.lexer.get_line())))
            }
            Token::Char(c) => ret = Some(Box::new(AST::CharacterLiteral(c, self.lexer.get_line()))),
            Token::True => ret = Some(Box::new(AST::BooleanLiteral(true, self.lexer.get_line()))),
            Token::False => ret = Some(Box::new(AST::BooleanLiteral(false, self.lexer.get_line()))),
            Token::Ident(name) => {
                ret = Some(Box::new(AST::VariableAccess(name, self.lexer.get_line())))
            }
            Token::LeftParen => {
                _ = self.lexer.get_next_token()?;
                ret = Some(Box::new(AST::Paren(
                    self.parse_clause(1000)?,
                    self.lexer.get_line(),
                )));
                match self.lexer.get_next_token()? {
                    Token::RightParen => (),
                    _ => {
                        return Err(Box::new(ParserError::new(
                            format!(
                                "Expected right paren, got {:?} while evaluating a parenthese expression",
                                self.lexer.get_current_token()
                            ),
                            self.lexer.get_line(),
                        )))
                    }
                }
            }
            _ => {
                return Err(Box::new(ParserError::new(
                    format!(
                        "Expected an expression, instead got {:?}",
                        self.lexer.get_current_token()
                    ),
                    self.lexer.get_line(),
                )))
            }
        }
        if self.lexer.peek_next_token()? == Token::LeftParen {
            _ = self.lexer.get_next_token()?;
            let mut args: Vec<Box<AST>> = vec![];
            loop {
                if args.len() == 0 && self.lexer.get_next_token()? == Token::RightParen {
                    break;
                }
                args.push(self.parse_clause(1000)?);
                match self.lexer.get_next_token()? {
                    Token::RightParen => break,
                    Token::Comma => _ = self.lexer.get_next_token()?,
                    _ => {
                        return Err(Box::new(ParserError::new(
                            format!(
                                "Expected comma or right paren, got {:?} in function call",
                                self.lexer.get_current_token()
                            ),
                            self.lexer.get_line(),
                        )))
                    }
                }
            }
            ret = Some(Box::new(AST::FunctionCall(
                ret.unwrap(),
                args,
                self.lexer.get_line(),
            )));
        }
        while self.lexer.peek_next_token()?.is_op()
            && usable_operator(&self.lexer.peek_next_token()?, max_op_prec, &self.lexer)?
        {
            let op = self.lexer.get_next_token()?;
            _ = self.lexer.get_next_token()?;
            let rhs = self.parse_clause(op.get_op_prec(&self.lexer)?)?;

            ret = Some(match op {
                Token::OpEqEq => Box::new(AST::OpEqEq(ret.unwrap(), rhs, self.lexer.get_line())),
                Token::OpEq => Box::new(AST::OpEq(ret.unwrap(), rhs, self.lexer.get_line())),
                Token::OpPls => Box::new(AST::OpPls(ret.unwrap(), rhs, self.lexer.get_line())),
                Token::OpPlsEq => Box::new(AST::OpPlsEq(ret.unwrap(), rhs, self.lexer.get_line())),
                Token::OpNotEq => Box::new(AST::OpNotEq(ret.unwrap(), rhs, self.lexer.get_line())),
                Token::OpGt => Box::new(AST::OpGt(ret.unwrap(), rhs, self.lexer.get_line())),
                Token::OpLt => Box::new(AST::OpLt(ret.unwrap(), rhs, self.lexer.get_line())),
                Token::OpGtEq => Box::new(AST::OpGtEq(ret.unwrap(), rhs, self.lexer.get_line())),
                Token::OpLtEq => Box::new(AST::OpLtEq(ret.unwrap(), rhs, self.lexer.get_line())),
                Token::OpAndAnd => {
                    Box::new(AST::OpAndAnd(ret.unwrap(), rhs, self.lexer.get_line()))
                }
                Token::OpOrOr => Box::new(AST::OpOrOr(ret.unwrap(), rhs, self.lexer.get_line())),
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!("Operator not implemented in parse clause {:?}, note: this is an internal error, nothing necessarily wrong with ur code", op),
                        self.lexer.get_line(),
                    )))
                }
            });
        }
        match ret {
            Some(v) => Ok(v),
            None => Err(Box::new(ParserError::new(
                "Exiting parse clause without an AST".into(),
                self.lexer.get_line(),
            ))),
        }
    }
    fn parse_variable_declaration(
        &mut self,
        is_const: bool,
    ) -> Result<Box<AST>, Box<dyn MapleError>> {
        let token = self.lexer.get_next_token()?;
        let var_decl = match token {
            Token::Ident(name) => Box::new(AST::VariableDeclaration(
                name,
                is_const,
                self.lexer.get_line(),
            )),
            _ => {
                return Err(Box::new(ParserError::new(
                    format!("Expected identifier, got {:?}", token),
                    self.lexer.get_line(),
                )))
            }
        };

        match self.lexer.get_next_token()? {
            Token::OpEq => {
                self.lexer.get_next_token()?;
                let expr = self.parse_clause(Token::OpEq.get_op_prec(&self.lexer)?)?;
                Ok(Box::new(AST::OpEq(var_decl, expr, self.lexer.get_line())))
            }
            Token::EndOfStatement | Token::EOF => {
                self.lexer.feed_token(Token::EndOfStatement);
                Ok(var_decl)
            }
            t => Err(Box::new(ParserError::new(
                format!("Expected =, got {:?}", t),
                self.lexer.get_line(),
            ))),
        }
    }
    // fn parse_statement(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
    //     let token = self.lexer.get_next_token()?;
    //     Ok(match token {
    //         Token::Var => self.parse_variable_declaration()?,
    //         Token::Ident(_) => {
    //             let ast = self.parse_clause(1000)?;
    //             self.lexer.get_next_token()?;
    //             ast
    //         }
    //         _ => return Err(format!("Token not yet implemented in parser: {:?}", token).into()),
    //     })
    // }

    fn parse_block(&mut self) -> Result<Vec<Box<AST>>, Box<dyn MapleError>> {
        while self.lexer.get_current_token() == Token::EndOfStatement {
            self.lexer.get_next_token()?;
        }
        match self.lexer.get_current_token() {
            Token::LeftBrace => (),
            _ => {
                return Err(Box::new(ParserError::new(
                    format!(
                        "Expected left brace to start block, got {:?}",
                        self.lexer.get_current_token()
                    ),
                    self.lexer.get_line(),
                )))
            }
        };
        match self.lexer.get_next_token()? {
            Token::EndOfStatement => (),
            _ => {
                return Err(Box::new(ParserError::new(
                    format!(
                        "Expected newline after left brace, got {:?}",
                        self.lexer.get_current_token()
                    ),
                    self.lexer.get_line(),
                )))
            }
        };
        let body = self.parse(false)?;
        Ok(body)
    }

    fn parse_condition_and_block(
        &mut self,
    ) -> Result<(Box<AST>, Vec<Box<AST>>), Box<dyn MapleError>> {
        let cond = self.parse_clause(1000)?;
        self.lexer.get_next_token()?;
        let body = self.parse_block()?;
        Ok((cond, body))
    }
    fn parse_while(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        self.lexer.get_next_token()?;
        let (cond, body) = self.parse_condition_and_block()?;
        self.lexer.get_next_token()?;
        self.lexer.feed_token(Token::EndOfStatement);
        Ok(Box::new(AST::While(cond, body, self.lexer.get_line())))
    }
    fn parse_if(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        self.lexer.get_next_token()?;
        let (cond, body) = self.parse_condition_and_block()?;
        let mut elseifs: Vec<(Box<AST>, Vec<Box<AST>>)> = vec![];
        self.lexer.get_next_token()?;
        let mut feed_token = Token::EndOfStatement;
        loop {
            while self.lexer.get_current_token() == Token::EndOfStatement {
                self.lexer.get_next_token()?;
            }
            if self.lexer.get_current_token() != Token::Elseif {
                break;
            }
            self.lexer.get_next_token()?;
            let block = self.parse_condition_and_block()?;
            elseifs.push(block);
            self.lexer.get_next_token()?;
        }
        while self.lexer.get_current_token() == Token::EndOfStatement {
            self.lexer.get_next_token()?;
        }
        let else_body = if self.lexer.get_current_token() == Token::Else {
            self.lexer.get_next_token()?;
            let body = self.parse_block()?;
            Some(body)
        } else {
            feed_token = self.lexer.get_current_token();
            None
        };
        self.lexer.feed_token(Token::EndOfStatement);
        if feed_token != Token::EndOfStatement {
            self.lexer.feed_token(feed_token);
        }

        // parse takes care of }, so we don't need to check for it here
        Ok(Box::new(AST::If(
            IfLiteral {
                cond,
                body,
                elseifs,
                else_body,
            },
            self.lexer.get_line(),
        )))
    }

    fn parse_break(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        // self.lexer.get_next_token()?;
        Ok(Box::new(AST::Break(self.lexer.get_line())))
    }
    fn parse_continue(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        // self.lexer.get_next_token()?;
        Ok(Box::new(AST::Continue(self.lexer.get_line())))
    }
    fn parse_return(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        self.lexer.get_next_token()?;
        let expr = self.parse_clause(1000)?;
        Ok(Box::new(AST::Return(expr, self.lexer.get_line())))
    }
    pub fn parse(&mut self, top_level: bool) -> Result<Vec<Box<AST>>, Box<dyn MapleError>> {
        let mut ret: Vec<Box<AST>> = vec![];
        if top_level {
            self.lexer.get_next_token()?;
        }
        loop {
            let ast = match self.lexer.get_current_token() {
                Token::Fn => Some(self.parse_function(false)?),
                Token::Break => Some(self.parse_break()?),
                Token::Continue => Some(self.parse_continue()?),
                Token::Return => Some(self.parse_return()?),
                Token::Const => Some(self.parse_variable_declaration(true)?),
                Token::Var => Some(self.parse_variable_declaration(false)?),
                Token::Ident(_) => {
                    let ast = self.parse_clause(1000)?;
                    // self.lexer.get_next_token()?;
                    Some(ast)
                }
                Token::While => Some(self.parse_while()?),
                Token::If => Some(self.parse_if()?),
                Token::EOF if top_level => break,
                Token::EOF if !top_level => {
                    return Err(Box::new(ParserError::new(
                        "Unexpected EOF (aka unclosed brace)".into(),
                        self.lexer.get_line(),
                    )));
                }
                Token::RightBrace if !top_level => {
                    break;
                }
                Token::RightBrace if top_level => {
                    return Err(Box::new(ParserError::new(
                        "Unexpected right brace in top level".into(),
                        self.lexer.get_line(),
                    )));
                }
                Token::EndOfStatement => None,
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!("Unexpected token {:?}", self.lexer.get_current_token()),
                        self.lexer.get_line(),
                    )))
                }
            };
            match self.lexer.get_next_token()? {
                Token::RightBrace if !top_level => {
                    break;
                }
                Token::RightBrace if top_level => {
                    return Err(Box::new(ParserError::new(
                        "Unexpected right brace in top level".into(),
                        self.lexer.get_line(),
                    )));
                }
                Token::EndOfStatement => (),
                Token::EOF if !top_level => {
                    return Err(Box::new(ParserError::new(
                        "Unexpected EOF (aka unclosed brace)".into(),
                        self.lexer.get_line(),
                    )));
                }
                Token::EOF => break,
                _ if ast.is_none() => (),
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!(
                            "Expected newline after statement \"{}\", instead got {:?}",
                            match ast {
                                Some(ast) => ast.pretty_print(),
                                None => "".to_string(),
                            },
                            self.lexer.get_current_token(),
                        ),
                        self.lexer.get_line(),
                    )));
                }
            };
            // _ = self.lexer.get_next_token()?;
            match ast {
                Some(ast) => {
                    // println!("{}", ast.pretty_print());
                    ret.push(ast)
                }
                None => {}
            }
        }
        Ok(ret)
    }
}
#[cfg(test)]
mod test_parser {
    #[test]
    fn test_newline_placement() {
        let code = r#"
var x = 0
x = 1
const c = 0"#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        println!("{}", code.to_string());
        assert!(ast.is_ok());
        let code_copy = code.to_string();
        // replace all newlines in code with 2 newlines
        let code_copy = code_copy.replace("\n", "\n\n");
        let mut parser = super::Parser::new(code_copy);
        let ast = parser.parse(true);
        assert!(ast.is_ok());
    }
    #[test]
    fn test_newline_in_if() {
        let code = r#"
if true {
    var x = 0
}"#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_ok());
        let code_copy = code.to_string();
        // replace all newlines in code with 2 newlines
        let code_copy = code_copy.replace("\n", "\n\n");
        let mut parser = super::Parser::new(code_copy);
        let ast = parser.parse(true);
        assert!(ast.is_ok());
    }
    #[test]
    fn test_newline_in_if2() {
        let mut code = r#"
if true {
    var x = 0
}
"#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        if !ast.is_ok() {
            println!("{}", ast.err().unwrap());
            assert!(false);
        }
        let code_copy = code.to_string();
        // replace all newlines in code with 2 newlines
        let code_copy = code_copy.replace("\n", "\n\n");
        let mut parser = super::Parser::new(code_copy);
        let ast = parser.parse(true);
        if !ast.is_ok() {
            println!("{}", ast.err().unwrap());
            assert!(false);
        }
        code = r#"
if true {
    var x = 0
} elseif true {
    var y = 0
} "#;
        parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        if !ast.is_ok() {
            println!("{}", ast.err().unwrap());
            assert!(false);
        }
        let code_copy = code.to_string();
        // replace all newlines in code with 2 newlines
        let code_copy = code_copy.replace("\n", "\n\n");
        let mut parser = super::Parser::new(code_copy);
        let ast = parser.parse(true);
        if !ast.is_ok() {
            println!("{}", ast.err().unwrap());
            assert!(false);
        }
        code = r#"
if true {
    var x = 0
} 

elseif true {
    var y = 0
} elseif true {
    var z = 0
} else {
    var a = 0
}"#;
        parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        if !ast.is_ok() {
            println!("{}", ast.err().unwrap());
            assert!(false);
        }
        let code_copy = code.to_string();
        // replace all newlines in code with 2 newlines
        let code_copy = code_copy.replace("\n", "\n\n");
        let mut parser = super::Parser::new(code_copy);
        let ast = parser.parse(true);
        if !ast.is_ok() {
            println!("{}", ast.err().unwrap());
            assert!(false);
        }
    }
    #[test]
    fn fails_on_unclosed_brace() {
        let code = r#"
if true {
    var x = 0
"#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_err());
    }
    #[test]
    fn fails_on_no_brace() {
        let code = r#"
if true
    var x = 0
"#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_err());
    }
    #[test]
    fn fails_on_no_newline() {
        let code = r#"
var x = 0 var y = 0"#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_err());
    }
    #[test]
    fn fails_on_extra_else() {
        let code = r#"
if true {
    var x = 0
} else {
    var y = 0
} else {
    var z = 0
}"#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_err());
    }

    #[test]
    fn interpret_if() {
        let code = r#"
var a
if true {
    a = 1
} else {
    a = 2
}"#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_ok());
        let mut scope_chain: super::ScopeChain = super::ScopeChain::new();
        let ast = ast.unwrap();
        for (_, stmt) in ast.iter().enumerate() {
            stmt.get_value(&mut scope_chain).unwrap();
        }
        let a_name = "a".to_string();
        let a = scope_chain.get_variable(&a_name, 0).unwrap();
        assert_eq!(a, super::Rc::new(super::Value::Number(1.0)));
    }
    #[test]
    fn interpret_if_else() {
        let code = r#"
var a
if false {
    a = 1
} else {
    a = 2
}"#;

        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_ok());
        let mut scope_chain: super::ScopeChain = super::ScopeChain::new();
        let ast = ast.unwrap();
        for (_, stmt) in ast.iter().enumerate() {
            stmt.get_value(&mut scope_chain).unwrap();
        }
        let a_name = "a".to_string();
        let a = scope_chain.get_variable(&a_name, 0).unwrap();
        assert_eq!(a, super::Rc::new(super::Value::Number(2.0)));
    }
    #[test]
    fn interpret_if_else2() {
        let code = r#"
var a
var b = true
if b {
    a = 1
} else {
    a = 2
}"#;

        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_ok());
        let mut scope_chain: super::ScopeChain = super::ScopeChain::new();
        let ast = ast.unwrap();
        for (_, stmt) in ast.iter().enumerate() {
            stmt.get_value(&mut scope_chain).unwrap();
        }
        let a_name = "a".to_string();
        let a = scope_chain.get_variable(&a_name, 0).unwrap();
        assert_eq!(a, super::Rc::new(super::Value::Number(1.0)));
    }
    #[test]
    fn function_pass_by_reference() {
        let code = r#"
var a = 0
fn add_one(b) {
    a += 1
}
add_one(a)
        "#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_ok());

        let mut scope_chain: super::ScopeChain = super::ScopeChain::new();
        let ast = ast.unwrap();
        for (_, stmt) in ast.iter().enumerate() {
            stmt.get_value(&mut scope_chain).unwrap();
        }
        let a_name = "a".to_string();
        let a = scope_chain.get_variable(&a_name, 0).unwrap();
        assert_eq!(a, super::Rc::new(super::Value::Number(1.0)));
    }
    #[test]
    fn first_eq_by_reference() {
        let code = r#"
var a = 0
var b = a
b = 10
        "#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_ok());

        let mut scope_chain: super::ScopeChain = super::ScopeChain::new();
        let ast = ast.unwrap();
        for (_, stmt) in ast.iter().enumerate() {
            stmt.get_value(&mut scope_chain).unwrap();
        }
        let a_name = "a".to_string();
        let a = scope_chain.get_variable(&a_name, 0).unwrap();
        assert_eq!(a, super::Rc::new(super::Value::Number(10.0)));
    }
    #[test]
    fn second_eq_by_copy() {
        let code = r#"
var a = 0
var b = 1
b = a
b = 10
    "#;
        let mut parser = super::Parser::new(code.to_string());
        let ast = parser.parse(true);
        assert!(ast.is_ok());

        let mut scope_chain: super::ScopeChain = super::ScopeChain::new();
        let ast = ast.unwrap();
        for (_, stmt) in ast.iter().enumerate() {
            stmt.get_value(&mut scope_chain).unwrap();
        }
        let a_name = "a".to_string();
        let a = scope_chain.get_variable(&a_name, 0).unwrap();
        assert_eq!(a, super::Rc::new(super::Value::Number(0.0)));
    }
}
