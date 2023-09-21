// turn off dead code warnings
#![allow(dead_code)]
use crate::lexer::{Assoc, Lexer, Token};

#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Boolean(bool),
    Variable(String, Box<Value>),
    Char(char),
    Undefined,
}

#[derive(Debug)]
pub enum AST {
    CharacterLiteral(char),
    StringLiteral(String),
    NumberLiteral(f64),
    OpPls(Box<AST>, Box<AST>),
    VariableDeclaration(String),
    OpEq(Box<AST>, Box<AST>),
    OpEqEq(Box<AST>, Box<AST>),
    VariableAccess(String),
}

impl AST {
    pub fn get_value(&self) -> Box<Value> {
        match self {
            AST::StringLiteral(str) => Box::new(Value::String(str.to_string())),
            AST::NumberLiteral(num) => Box::new(Value::Number(*num)),
            AST::OpPls(_, _) => Box::new(Value::Number(0f64)),
            AST::VariableDeclaration(_) => Box::new(Value::Number(0f64)),
            AST::OpEq(_, _) => Box::new(Value::Number(0f64)),
            AST::CharacterLiteral(c) => Box::new(Value::Char(*c)),
            AST::OpEqEq(_, _) => Box::new(Value::Number(0f64)),
            AST::VariableAccess(_) => Box::new(Value::Number(0f64)),
        }
    }

    pub fn pretty_print(&self) -> String {
        match self {
            AST::StringLiteral(value) => format!("\"{}\"", value),
            AST::NumberLiteral(value) => value.to_string(),
            AST::OpPls(left, right) => {
                format!("{} + {}", left.pretty_print(), right.pretty_print())
            }
            AST::VariableDeclaration(name) => format!("var {}", name),
            AST::OpEq(left, right) => {
                format!("{} = {}", left.pretty_print(), right.pretty_print())
            }
            AST::CharacterLiteral(c) => format!("'{}'", c),
            AST::OpEqEq(left, right) => {
                format!("{} == {}", left.pretty_print(), right.pretty_print())
            }
            AST::VariableAccess(name) => name.to_string(),
        }
    }
}

#[derive(Debug)]
pub struct Parser {
    lexer: Lexer,
}
fn usable_operator(op: &Token, max_op_prec: i32) -> Result<bool, Box<dyn std::error::Error>> {
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
    fn parse_clause(&mut self, max_op_prec: i32) -> Result<Box<AST>, Box<dyn std::error::Error>> {
        let mut ret: Option<Box<AST>>;
        match self.lexer.get_next_token()? {
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
            let rhs = self.parse_clause(op.get_op_prec()?)?;

            ret = Some(match op {
                Token::OpEqEq => Box::new(AST::OpEqEq(ret.unwrap(), rhs)),
                Token::OpEq => Box::new(AST::OpEq(ret.unwrap(), rhs)),
                Token::OpPls => Box::new(AST::OpPls(ret.unwrap(), rhs)),
                _ => {
                    return Err(format!("Operator not implemented in parse clause {:?}", op).into())
                }
            });
        }
        match ret {
            Some(v) => Ok(v),
            None => Err("Exiting parse clause without a parse AST".into()),
        }
    }
    fn parse_variable_declaration(&mut self) -> Result<Box<AST>, Box<dyn std::error::Error>> {
        let token = self.lexer.get_next_token()?;
        let var_decl = match token {
            Token::Ident(name) => Box::new(AST::VariableDeclaration(name)),
            _ => return Err(format!("Expected identifier, got {:?}", token).into()),
        };

        match self.lexer.get_next_token()? {
            Token::OpEq => {
                let expr = self.parse_clause(Token::OpEq.get_op_prec()?)?;
                Ok(Box::new(AST::OpEq(var_decl, expr)))
            }
            t => Err(format!("Expected =, got {:?}", t).into()),
        }
    }
    fn parse_statement(&mut self) -> Result<Box<AST>, Box<dyn std::error::Error>> {
        let token = self.lexer.get_next_token()?;
        Ok(match token {
            Token::Var => self.parse_variable_declaration()?,
            Token::Ident(_) => {
                let ast = self.parse_clause(1000)?;
                self.lexer.get_next_token()?;
                ast
            }
            _ => return Err(format!("Token not yet implemented in parser: {:?}", token).into()),
        })
    }
    pub fn parse(&mut self) -> Result<Vec<Box<AST>>, Box<dyn std::error::Error>> {
        let mut ret: Vec<Box<AST>> = vec![];
        let ast = self.parse_statement()?;
        ret.push(ast);
        Ok(ret)
    }
}
#[cfg(test)]
mod test_parser {}
