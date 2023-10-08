// turn off dead code warnings
#![allow(dead_code)]
use std::{error::Error, rc::Rc};

use crate::ast::{FunctionLiteral, IfLiteral, AST};
use crate::lexer::{Assoc, Lexer, Token};
use crate::scopechain::ScopeChain;

pub trait Unpack<T> {
    fn unpack(&self, scope_chain: &ScopeChain) -> Result<T, Box<dyn Error>>;
}
impl Unpack<Rc<Value>> for Rc<Value> {
    fn unpack(&self, scope_chain: &ScopeChain) -> Result<Rc<Value>, Box<dyn std::error::Error>> {
        match self.as_ref() {
            Value::Variable(name) => {
                let value = scope_chain.get_variable(&name)?;
                value.unpack(scope_chain)
            }
            // Value::Undefined => Err("Unpacking an undefined value".into()),
            _ => Ok(self.clone()),
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
    Undefined,
}
impl Value {
    pub fn pretty_type(&self, scope_chain: &ScopeChain) -> String {
        match self {
            Value::Function(_) => "function".to_string(),
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
    fn parse_function(&mut self, anon: bool) -> Result<Box<AST>, Box<dyn Error>> {
        let mut params: Vec<String> = vec![];
        let body: Vec<Box<AST>>;
        match self.lexer.get_current_token() {
            Token::Fn => (),
            _ => {
                return Err(format!("Expected fn, got {:?}", self.lexer.get_current_token()).into())
            }
        };
        let name: String;
        if !anon {
            match self.lexer.get_next_token()? {
                Token::Ident(n) => {
                    name = n;
                }
                _ => {
                    return Err(format!(
                        "Expected left paren, got {:?}",
                        self.lexer.get_current_token()
                    )
                    .into())
                }
            };
        } else {
            name = "".to_string();
        }
        match self.lexer.get_next_token()? {
            Token::LeftParen => (),
            _ => {
                return Err(format!(
                    "Expected left paren, got {:?}",
                    self.lexer.get_current_token()
                )
                .into())
            }
        };
        loop {
            match self.lexer.get_next_token()? {
                Token::Ident(name) => params.push(name),
                Token::RightParen => break,
                _ => {
                    return Err(format!(
                        "Expected identifier or right paren, got {:?}",
                        self.lexer.get_current_token()
                    )
                    .into())
                }
            }
            match self.lexer.get_next_token()? {
                Token::RightParen => break,
                Token::Comma => (),
                _ => {
                    return Err(format!(
                        "Expected comma or right paren, got {:?}",
                        self.lexer.get_current_token()
                    )
                    .into())
                }
            }
        }
        self.lexer.get_next_token()?;
        body = self.parse_block()?;
        if !anon {
            Ok(Box::new(AST::OpEq(
                Box::new(AST::VariableDeclaration(name.clone(), true)),
                Box::new(AST::FunctionLiteral(FunctionLiteral::basic(params, body))),
            )))
        } else {
            Ok(Box::new(AST::FunctionLiteral(FunctionLiteral::basic(
                params, body,
            ))))
        }
    }
    fn parse_clause(&mut self, max_op_prec: i32) -> Result<Box<AST>, Box<dyn Error>> {
        let mut ret: Option<Box<AST>>;
        match self.lexer.get_current_token() {
            Token::Fn => ret = Some(self.parse_function(true)?),
            Token::Number(num) => ret = Some(Box::new(AST::NumberLiteral(num))),
            Token::String(str) => ret = Some(Box::new(AST::StringLiteral(str))),
            Token::Char(c) => ret = Some(Box::new(AST::CharacterLiteral(c))),
            Token::True => ret = Some(Box::new(AST::BooleanLiteral(true))),
            Token::False => ret = Some(Box::new(AST::BooleanLiteral(false))),
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
                        return Err(format!(
                            "Expected comma or right paren, got {:?}",
                            self.lexer.get_current_token()
                        )
                        .into())
                    }
                }
            }
            ret = Some(Box::new(AST::FunctionCall(ret.unwrap(), args)));
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
                Token::OpNotEq => Box::new(AST::OpNotEq(ret.unwrap(), rhs)),
                Token::OpGt => Box::new(AST::OpGt(ret.unwrap(), rhs)),
                Token::OpLt => Box::new(AST::OpLt(ret.unwrap(), rhs)),
                Token::OpGtEq => Box::new(AST::OpGtEq(ret.unwrap(), rhs)),
                Token::OpLtEq => Box::new(AST::OpLtEq(ret.unwrap(), rhs)),
                Token::OpAndAnd => Box::new(AST::OpAndAnd(ret.unwrap(), rhs)),
                Token::OpOrOr => Box::new(AST::OpOrOr(ret.unwrap(), rhs)),
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
            Token::EndOfStatement | Token::EOF => {
                self.lexer.feed_token(Token::EndOfStatement);
                Ok(var_decl)
            }
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

    fn parse_block(&mut self) -> Result<Vec<Box<AST>>, Box<dyn Error>> {
        while self.lexer.get_current_token() == Token::EndOfStatement {
            self.lexer.get_next_token()?;
        }
        match self.lexer.get_current_token() {
            Token::LeftBrace => (),
            _ => {
                return Err(format!(
                    "Expected left brace to start block, got {:?}",
                    self.lexer.get_current_token()
                )
                .into())
            }
        };
        match self.lexer.get_next_token()? {
            Token::EndOfStatement => (),
            _ => {
                return Err(format!(
                    "Expected newline after left brace, got {:?}",
                    self.lexer.get_current_token()
                )
                .into())
            }
        };
        let body = self.parse(false)?;
        Ok(body)
    }

    fn parse_condition_and_block(&mut self) -> Result<(Box<AST>, Vec<Box<AST>>), Box<dyn Error>> {
        let cond = self.parse_clause(1000)?;
        self.lexer.get_next_token()?;
        let body = self.parse_block()?;
        Ok((cond, body))
    }
    fn parse_while(&mut self) -> Result<Box<AST>, Box<dyn Error>> {
        self.lexer.get_next_token()?;
        let (cond, body) = self.parse_condition_and_block()?;
        self.lexer.get_next_token()?;
        self.lexer.feed_token(Token::EndOfStatement);
        Ok(Box::new(AST::While(cond, body)))
    }
    fn parse_if(&mut self) -> Result<Box<AST>, Box<dyn Error>> {
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
        Ok(Box::new(AST::If(IfLiteral {
            cond,
            body,
            elseifs,
            else_body,
        })))
    }
    pub fn parse(&mut self, top_level: bool) -> Result<Vec<Box<AST>>, Box<dyn Error>> {
        let mut ret: Vec<Box<AST>> = vec![];
        if top_level {
            self.lexer.get_next_token()?;
        }
        loop {
            let ast = match self.lexer.get_current_token() {
                Token::Fn => Some(self.parse_function(false)?),
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
                    return Err("Unexpected EOF (aka unclosed brace)".into());
                }
                Token::RightBrace if !top_level => {
                    break;
                }
                Token::RightBrace if top_level => {
                    return Err("Unexpected right brace in top level".into());
                }
                Token::EndOfStatement => None,
                _ => {
                    return Err(
                        format!("Unexpected token {:?}", self.lexer.get_current_token()).into(),
                    )
                }
            };
            match self.lexer.get_next_token()? {
                Token::RightBrace if !top_level => {
                    break;
                }
                Token::RightBrace if top_level => {
                    return Err("Unexpected right brace in top level".into());
                }
                Token::EndOfStatement => (),
                Token::EOF if !top_level => {
                    return Err("Unexpected EOF (aka unclosed brace)".into());
                }
                Token::EOF => break,
                _ if ast.is_none() => (),
                _ => {
                    return Err(format!(
                        "Expected newline after statement \"{}\", instead got {:?}, at line {}",
                        match ast {
                            Some(ast) => ast.pretty_print(),
                            None => "".to_string(),
                        },
                        self.lexer.get_current_token(),
                        self.lexer.get_line()
                    )
                    .into())
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
}
