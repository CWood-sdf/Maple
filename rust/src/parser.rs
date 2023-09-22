// turn off dead code warnings
#![allow(dead_code)]
use std::{error::Error, rc::Rc};

use crate::ast::AST;
use crate::lexer::{Assoc, Lexer, Token};
use crate::scopechain::ScopeChain;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Variable(String),
    Char(char),
    Undefined,
}
impl Value {
    pub fn pretty_type(&self, scope_chain: &ScopeChain) -> String {
        match self {
            Value::String(_) => "string".to_string(),
            Value::Number(_) => "number".to_string(),
            Value::Boolean(_) => "boolean".to_string(),
            Value::Variable(name) => scope_chain
                .get_variable(&name)
                .unwrap()
                .pretty_type(scope_chain),
            Value::Char(_) => "char".to_string(),
            Value::Undefined => "undefined".to_string(),
        }
    }
}
#[derive(Debug)]
pub struct Variable {
    pub name: String,
    pub value: Rc<Value>,
    pub is_const: bool,
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}
fn usable_operator(op: &Token, max_op_prec: i32) -> Result<bool, Box<dyn Error>> {
    if op.is_op() {
        if op.get_op_assoc()? == Assoc::Left {
            Ok(op.get_op_prec()? < max_op_prec)
        } else {
            Ok(op.get_op_prec()? <= max_op_prec)
        }
    } else {
        Err("Did not pass in an operator to usable_operator".into())
    }
}
impl Parser {
    pub fn new(contents: String) -> Parser {
        let lexer = Lexer::new(contents);
        Parser { lexer }
    }
    fn parse_clause(&mut self, max_op_prec: i32) -> Result<Box<AST>, Box<dyn Error>> {
        let mut ret: Option<Box<AST>>;
        match self.lexer.get_current_token() {
            Token::Number(num) => ret = Some(Box::new(AST::NumberLiteral(num))),
            Token::String(str) => ret = Some(Box::new(AST::StringLiteral(str))),
            Token::Char(c) => ret = Some(Box::new(AST::CharacterLiteral(c))),
            Token::Ident(name) if self.lexer.peek_next_token()? == Token::LeftParen => {
                return Err(format!(
                    "Function calls not yet implemented in parse_clause for function call {:?}",
                    name
                )
                .into())
            }
            Token::Ident(name) => ret = Some(Box::new(AST::VariableAccess(name))),
            Token::LeftParen => {
                _ = self.lexer.get_next_token()?;
                ret = Some(Box::new(AST::Paren(self.parse_clause(1000)?)));
                match self.lexer.get_next_token()? {
                    Token::RightParen => (),
                    _ => {
                        return Err(format!(
                            "Expected right paren, got {:?}",
                            self.lexer.get_current_token()
                        )
                        .into())
                    }
                }
            }
            _ => {
                return Err(format!(
                    "Unexpected token passed into parse_clause {:?}",
                    self.lexer.get_current_token()
                )
                .into())
            }
        }
        while self.lexer.peek_next_token()?.is_op()
            && usable_operator(&self.lexer.peek_next_token()?, max_op_prec)?
        {
            let op = self.lexer.get_next_token()?;
            _ = self.lexer.get_next_token()?;
            let rhs = self.parse_clause(op.get_op_prec()?)?;

            ret = Some(match op {
                Token::OpEqEq => Box::new(AST::OpEqEq(ret.unwrap(), rhs)),
                Token::OpEq => Box::new(AST::OpEq(ret.unwrap(), rhs)),
                Token::OpPls => Box::new(AST::OpPls(ret.unwrap(), rhs)),
                Token::OpPlsEq => Box::new(AST::OpPlsEq(ret.unwrap(), rhs)),
                _ => {
                    return Err(format!("Operator not implemented in parse clause {:?}", op).into())
                }
            });
        }
        match ret {
            Some(v) => Ok(v),
            None => Err("Exiting parse clause without an AST".into()),
        }
    }
    fn parse_variable_declaration(&mut self, is_const: bool) -> Result<Box<AST>, Box<dyn Error>> {
        let token = self.lexer.get_next_token()?;
        let var_decl = match token {
            Token::Ident(name) => Box::new(AST::VariableDeclaration(name, is_const)),
            _ => return Err(format!("Expected identifier, got {:?}", token).into()),
        };

        match self.lexer.get_next_token()? {
            Token::OpEq => {
                self.lexer.get_next_token()?;
                let expr = self.parse_clause(Token::OpEq.get_op_prec()?)?;
                Ok(Box::new(AST::OpEq(var_decl, expr)))
            }
            Token::EndOfStatement | Token::EOF => Ok(var_decl),
            t => Err(format!("Expected =, got {:?}", t).into()),
        }
    }
    // fn parse_statement(&mut self) -> Result<Box<AST>, Box<dyn Error>> {
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
    pub fn parse(&mut self, top_level: bool) -> Result<Vec<Box<AST>>, Box<dyn Error>> {
        let mut ret: Vec<Box<AST>> = vec![];
        if top_level {
            self.lexer.get_next_token()?;
        }
        loop {
            let ast = match self.lexer.get_current_token() {
                Token::Const => Some(self.parse_variable_declaration(true)?),
                Token::Var => Some(self.parse_variable_declaration(false)?),
                Token::Ident(_) => {
                    let ast = self.parse_clause(1000)?;
                    Some(ast)
                }
                Token::EOF if top_level => break,
                Token::EOF if !top_level => {
                    return Err("Unexpected EOF (aka unclosed brace)".into());
                }
                Token::EndOfStatement => None,
                _ => {
                    return Err(
                        format!("Unexpected token {:?}", self.lexer.get_current_token()).into(),
                    )
                }
            };
            match self.lexer.get_current_token() {
                Token::RightBrace if !top_level => {
                    break;
                }
                Token::RightBrace if top_level => {
                    return Err("Unexpected right brace in top level".into());
                }
                Token::EndOfStatement => (),
                Token::EOF => break,
                _ => {
                    return Err(format!(
                        "Expected newline after statement \"{}\", instead got {:?}",
                        match ast {
                            Some(ast) => ast.pretty_print(),
                            None => "".to_string(),
                        },
                        self.lexer.get_current_token()
                    )
                    .into())
                }
            };
            _ = self.lexer.get_next_token()?;
            match ast {
                Some(ast) => {
                    // println!("{}", ast.pretty_print());
                    ret.push(ast)
                }
                None => println!(""),
            }
        }
        Ok(ret)
    }
}
#[cfg(test)]
mod test_parser {}
