// turn off dead code warnings
#![allow(dead_code)]
use std::{error::Error, rc::Rc};

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
pub enum AST {
    CharacterLiteral(char),
    StringLiteral(String),
    NumberLiteral(f64),
    // really only here for pretty printing
    Paren(Box<AST>),
    VariableDeclaration(String, bool),
    OpPls(Box<AST>, Box<AST>),
    OpEq(Box<AST>, Box<AST>),
    OpEqEq(Box<AST>, Box<AST>),
    OpPlsEq(Box<AST>, Box<AST>),
    VariableAccess(String),
}

impl AST {
    fn unpack_value(
        value: Rc<Value>,
        scope_chain: &ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn std::error::Error>> {
        match value.as_ref() {
            Value::Variable(name) => {
                let value = scope_chain.get_variable(&name)?;
                AST::unpack_value(value, scope_chain)
            }
            // Value::Undefined => Err("Unpacking an undefined value".into()),
            _ => Ok(value),
        }
    }
    fn eval_op_pls(
        right: Rc<Value>,
        left: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn std::error::Error>> {
        let left = AST::unpack_value(left, scope_chain)?;
        let right = AST::unpack_value(right, scope_chain)?;
        match (left.as_ref(), right.as_ref()) {
            (Value::Number(left), Value::Number(right)) => Ok(Rc::new(Value::Number(left + right))),
            (Value::Char(left), Value::Char(right)) => Ok(Rc::new(Value::Number(
                *left as i128 as f64 + *right as i128 as f64,
            ))),
            (Value::Number(left), Value::Char(right)) => {
                Ok(Rc::new(Value::Number(*left + *right as i128 as f64)))
            }
            (Value::Char(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Number(*left as i128 as f64 + *right)))
            }
            (Value::String(left), Value::String(right)) => {
                Ok(Rc::new(Value::String(format!("{}{}", left, right))))
            }
            (Value::String(left), Value::Char(right)) => {
                Ok(Rc::new(Value::String(format!("{}{}", left, right))))
            }
            (Value::Char(left), Value::String(right)) => {
                Ok(Rc::new(Value::String(format!("{}{}", left, right))))
            }
            _ => Err(format!(
                "Cannot add types {} and {}",
                left.pretty_type(scope_chain),
                right.pretty_type(scope_chain)
            )
            .into()),
        }
    }
    fn eval_op_eqeq(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        let left = AST::unpack_value(left, scope_chain)?;
        let right = AST::unpack_value(right, scope_chain)?;
        Ok(Rc::new(Value::Boolean(left == right)))
    }
    fn eval_op_eq(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        match left.as_ref() {
            Value::Variable(name) => {
                scope_chain.set_variable(&name, AST::unpack_value(right.clone(), scope_chain)?)?;
                Ok(left)
            }
            _ => Err("Cannot assign to a non-variable".into()),
        }
    }
    fn eval_plseq(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        match left.as_ref() {
            Value::Variable(name) => {
                let a = scope_chain.get_variable(&name)?;
                let a_ptr = Rc::<Value>::as_ptr(&a) as *mut Value;
                let b = AST::unpack_value(right, scope_chain)?;
                match (a.as_ref(), b.as_ref()) {
                    (Value::Number(a), Value::Number(b)) => unsafe {
                        *a_ptr = Value::Number(*a + *b);
                    },
                    (Value::Char(a), Value::Char(b)) => unsafe {
                        *a_ptr = Value::Number(*a as i128 as f64 + *b as i128 as f64);
                    },
                    (Value::Number(a), Value::Char(b)) => unsafe {
                        *a_ptr = Value::Number(*a + *b as i128 as f64);
                    },
                    (Value::Char(a), Value::Number(b)) => unsafe {
                        *a_ptr = Value::Number(*a as i128 as f64 + *b);
                    },
                    (Value::String(a), Value::String(b)) => unsafe {
                        *a_ptr = Value::String(format!("{}{}", a, b));
                    },
                    (Value::String(a), Value::Char(b)) => unsafe {
                        *a_ptr = Value::String(format!("{}{}", a, b));
                    },
                    (Value::Char(a), Value::String(b)) => unsafe {
                        *a_ptr = Value::String(format!("{}{}", a, b));
                    },
                    _ => {
                        return Err(format!(
                            "Cannot add types {} and {}",
                            a.pretty_type(scope_chain),
                            b.pretty_type(scope_chain)
                        )
                        .into())
                    }
                };
                Ok(Rc::new(Value::Variable(name.clone())))
            }
            _ => Err("Cannot assign to a non-variable in +=".into()),
        }
    }
    pub fn get_value(&self, scope_chain: &mut ScopeChain) -> Result<Rc<Value>, Box<dyn Error>> {
        Ok(match self {
            AST::StringLiteral(str) => Rc::new(Value::String(str.to_string())),
            AST::NumberLiteral(num) => Rc::new(Value::Number(*num)),
            AST::VariableDeclaration(name, is_const) => {
                scope_chain.add_variable(name, *is_const)?;
                Rc::new(Value::Variable(name.clone()))
            }
            AST::OpPls(left, right) => AST::eval_op_pls(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpPlsEq(left, right) => AST::eval_plseq(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpEq(left, right) => AST::eval_op_eq(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::CharacterLiteral(c) => Rc::new(Value::Char(*c)),
            AST::OpEqEq(left, right) => AST::eval_op_eqeq(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::VariableAccess(name) => Value::Variable(name.clone()).into(),
            AST::Paren(ast) => ast.get_value(scope_chain)?,
        })
    }

    pub fn debug_pretty_print(&self) -> String {
        match self {
            AST::StringLiteral(value) => format!("\"{}\"", value),
            AST::NumberLiteral(value) => value.to_string(),
            AST::OpPls(left, right) => {
                format!(
                    "({} + {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpPlsEq(left, right) => {
                format!(
                    "({} += {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::VariableDeclaration(name, is_const) if !*is_const => format!("var {}", name),
            AST::VariableDeclaration(name, _) => format!("const {}", name),
            AST::OpEq(left, right) => {
                format!(
                    "({} = {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::CharacterLiteral(c) => format!("'{}'", c),
            AST::OpEqEq(left, right) => {
                format!(
                    "({} == {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::VariableAccess(name) => name.to_string(),
            AST::Paren(ast) => format!("({})", ast.debug_pretty_print()),
        }
    }

    pub fn pretty_print(&self) -> String {
        match self {
            AST::StringLiteral(value) => format!("\"{}\"", value),
            AST::NumberLiteral(value) => value.to_string(),
            AST::OpPls(left, right) => {
                format!("{} + {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpPlsEq(left, right) => {
                format!("{} += {}", left.pretty_print(), right.pretty_print())
            }
            AST::VariableDeclaration(name, is_const) if !*is_const => format!("var {}", name),
            AST::VariableDeclaration(name, _) => format!("const {}", name),
            AST::OpEq(left, right) => {
                format!("{} = {}", left.pretty_print(), right.pretty_print())
            }
            AST::CharacterLiteral(c) => format!("'{}'", c),
            AST::OpEqEq(left, right) => {
                format!("{} == {}", left.pretty_print(), right.pretty_print())
            }
            AST::VariableAccess(name) => name.to_string(),
            AST::Paren(ast) => format!("({})", ast.pretty_print()),
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
    fn parse_variable_declaration(
        &mut self,
        is_const: bool,
    ) -> Result<Box<AST>, Box<dyn std::error::Error>> {
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
    // fn parse_statement(&mut self) -> Result<Box<AST>, Box<dyn std::error::Error>> {
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
    pub fn parse(&mut self, top_level: bool) -> Result<Vec<Box<AST>>, Box<dyn std::error::Error>> {
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
