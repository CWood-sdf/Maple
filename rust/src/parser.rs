// turn off dead code warnings
#![allow(dead_code)]
use core::fmt;
use std::rc::Rc;

use crate::error::{MapleError, ParserError, RuntimeError, ScopeError};

use crate::ast::{ASTType, FunctionLiteral, IfLiteral, AST};
use crate::lexer::{Assoc, Lexer, Token, TokenType};
use crate::scopechain::ScopeChain;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ObjectKey {
    String(String),
    Number(f64),
}
impl fmt::Display for ObjectKey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ObjectKey::String(s) => write!(f, "{}", s),
            ObjectKey::Number(n) => write!(f, "{}", n),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Object {
    pub fields: Vec<(ObjectKey, Rc<Value>)>,
}

impl Object {
    pub fn get(&self, key: ObjectKey, line: usize) -> Result<Rc<Value>, Box<RuntimeError>> {
        for (k, v) in self.fields.iter() {
            if k == &key {
                return Ok(v.clone());
            }
        }
        Err(Box::new(RuntimeError::new(
            format!("Object does not have key {:?}", key),
            line,
        )))
    }
    pub fn set(&mut self, key: ObjectKey, value: Rc<Value>) {
        for (k, v) in self.fields.iter_mut() {
            if k == &key {
                let val_ptr = Rc::<Value>::as_ptr(&v) as *mut Value;
                unsafe {
                    *val_ptr = value.as_ref().clone();
                }
                return;
            }
        }
        self.fields.push((key, value));
    }
    pub fn new() -> Object {
        Object { fields: vec![] }
    }
}
pub trait Unpack<T> {
    fn unpack(&self, scope_chain: &ScopeChain, line: usize) -> Result<T, ScopeError>;
    fn unpack_and_transform(
        &self,
        scope_chain: &ScopeChain,
        line: usize,
        ast: &AST,
    ) -> Result<Rc<Value>, Box<RuntimeError>>;
}
impl Unpack<Rc<Value>> for Result<Rc<Value>, Box<RuntimeError>> {
    fn unpack(&self, _scope_chain: &ScopeChain, _line: usize) -> Result<Rc<Value>, ScopeError> {
        panic!("Yo this is never gonna be implemented")
    }
    fn unpack_and_transform(
        &self,
        scope_chain: &ScopeChain,
        line: usize,
        ast: &AST,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        match self {
            Ok(value) => Ok(value.unpack_and_transform(scope_chain, line, ast)?),
            Err(e) => Err(e.clone()),
        }
    }
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
            Value::ObjectAccess(v, key) => match v.as_ref() {
                Value::Object(l) => match l.get(key.clone(), line) {
                    Ok(value) => value.unpack(scope_chain, line),
                    Err(_) => Err(ScopeError::new(
                        format!("Object does not have key {}", key),
                        line,
                    )),
                },

                _ => Err(ScopeError::new(
                    "Cannot get object access of non-object".into(),
                    line,
                )),
            },
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
            Err(e) => Err(Box::new(e.to_runtime_error().add_base_ast(ast.clone()))),
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
    Object(Object),
    ObjectAccess(Rc<Value>, ObjectKey),
    BuiltinFunction(
        fn(Vec<Rc<Value>>, &AST, &ScopeChain, usize) -> Result<Rc<Value>, Box<RuntimeError>>,
        usize,
    ),
    Undefined,
}
impl Value {
    pub fn pretty_type(&self, scope_chain: &ScopeChain, line: usize) -> String {
        match self {
            Value::ObjectAccess(obj, key) => match obj.as_ref() {
                Value::Object(l) => l
                    .get(key.clone(), line)
                    .unwrap()
                    .pretty_type(scope_chain, line),
                _ => format!("unknown type"),
            },
            Value::Object(_) => "object".to_string(),
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
    op: &TokenType,
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
        match self.lexer.get_current_token().t {
            TokenType::Fn => (),
            _ => {
                return Err(Box::new(ParserError::new(
                    format!("Expected fn, got {:?}", self.lexer.get_current_token()).into(),
                    self.lexer.get_line(),
                )))
            }
        };
        let fn_token = self.lexer.get_current_token();
        let name: String;
        if !anon {
            match self.lexer.get_next_token()?.t {
                TokenType::Ident(n) => {
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
        match self.lexer.get_next_token()?.t {
            TokenType::LeftParen => (),
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
            match self.lexer.get_next_token()?.t {
                TokenType::Ident(name) => params.push(name),
                TokenType::RightParen => break,
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
            match self.lexer.get_next_token()?.t {
                TokenType::RightParen => break,
                TokenType::Comma => (),
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
            Ok(Box::new(AST {
                t: ASTType::OpEq(
                    Box::new(AST {
                        t: ASTType::VariableDeclaration(name.clone(), true),
                        token: fn_token.clone(),
                    }),
                    Box::new(AST {
                        t: ASTType::FunctionLiteral(FunctionLiteral::basic(params, body)),
                        token: fn_token.clone(),
                    }),
                ),
                token: fn_token.clone(),
            }))
        } else {
            Ok(Box::new(AST {
                t: ASTType::FunctionLiteral(FunctionLiteral::basic(params, body)),
                token: fn_token,
            }))
        }
    }
    fn parse_object_literal(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        let token = self.lexer.get_next_token()?;
        let mut fields: Vec<(ObjectKey, Box<AST>)> = vec![];
        loop {
            while self.lexer.get_current_token().t == TokenType::EndOfStatement {
                self.lexer.get_next_token()?;
            }
            if self.lexer.get_current_token().t == TokenType::RightBrace {
                break;
            }
            let name = match self.lexer.get_current_token().t {
                TokenType::Ident(name) => ObjectKey::String(name),
                TokenType::Number(num) => ObjectKey::Number(num),
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!(
                            "Expected identifier, got {:?} in object literal",
                            self.lexer.get_current_token()
                        ),
                        self.lexer.get_line(),
                    )))
                }
            };
            match self.lexer.get_next_token()?.t {
                TokenType::OpEq => (),
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!(
                            "Expected =, got {:?} in object literal",
                            self.lexer.get_current_token()
                        ),
                        self.lexer.get_line(),
                    )))
                }
            }
            self.lexer.get_next_token()?;
            let expr = self.parse_clause(1000)?;
            fields.push((name, expr));
            match self.lexer.get_next_token()?.t {
                TokenType::RightBrace => break,
                TokenType::Comma | TokenType::EndOfStatement => (),
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!(
                            "Expected comma or right brace, got {:?} in object literal",
                            self.lexer.get_current_token()
                        ),
                        self.lexer.get_line(),
                    )))
                }
            }
            self.lexer.get_next_token()?;
        }
        Ok(Box::new(AST {
            t: ASTType::ObjectLiteral(fields),
            token,
        }))
    }
    fn parse_array_literal(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        let token = self.lexer.get_current_token();
        self.lexer.get_next_token()?;
        let mut fields: Vec<(ObjectKey, Box<AST>)> = vec![];
        let mut index = 0f64;
        loop {
            while self.lexer.get_current_token().t == TokenType::EndOfStatement {
                self.lexer.get_next_token()?;
            }
            if self.lexer.get_current_token().t == TokenType::RightSquare {
                break;
            }
            let expr = self.parse_clause(1000)?;
            fields.push((ObjectKey::Number(index), expr));
            match self.lexer.get_next_token()?.t {
                TokenType::RightSquare => break,
                TokenType::Comma | TokenType::EndOfStatement => (),
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!(
                            "Expected comma or right brace, got {:?} in object literal",
                            self.lexer.get_current_token()
                        ),
                        self.lexer.get_line(),
                    )))
                }
            }
            self.lexer.get_next_token()?;
            index += 1.0;
        }
        Ok(Box::new(AST {
            t: ASTType::ObjectLiteral(fields),
            token,
        }))
    }
    fn parse_clause(&mut self, max_op_prec: i32) -> Result<Box<AST>, Box<dyn MapleError>> {
        let mut ret: Option<Box<AST>>;

        match self.lexer.get_current_token().t {
            TokenType::Import(import_str) => {
                ret = Some(Box::new(AST {
                    t: ASTType::Import(import_str),
                    token: self.lexer.get_current_token(),
                }));
            }
            TokenType::LeftSquare => {
                ret = Some(self.parse_array_literal()?);
            }
            TokenType::Fn => ret = Some(self.parse_function(true)?),
            TokenType::Number(num) => {
                ret = Some(Box::new(AST {
                    t: ASTType::NumberLiteral(num),
                    token: self.lexer.get_current_token(),
                }))
            }
            TokenType::String(str) => {
                ret = Some(Box::new(AST {
                    t: ASTType::StringLiteral(str),
                    token: self.lexer.get_current_token(),
                }))
            }
            TokenType::Char(c) => {
                ret = Some(Box::new(AST {
                    t: ASTType::CharacterLiteral(c),
                    token: self.lexer.get_current_token(),
                }))
            }
            TokenType::True => {
                ret = Some(Box::new(AST {
                    t: ASTType::BooleanLiteral(true),
                    token: self.lexer.get_current_token(),
                }))
            }
            TokenType::False => {
                ret = Some(Box::new(AST {
                    t: ASTType::BooleanLiteral(false),
                    token: self.lexer.get_current_token(),
                }))
            }
            TokenType::Ident(name) => {
                ret = Some(Box::new(AST {
                    t: ASTType::VariableAccess(name),
                    token: self.lexer.get_current_token(),
                }))
            }
            TokenType::LeftBrace => {
                ret = Some(self.parse_object_literal()?);
            }
            TokenType::LeftParen => {
                let token = self.lexer.get_current_token();
                _ = self.lexer.get_next_token()?;
                ret = Some(Box::new(AST {
                    t: ASTType::Paren(self.parse_clause(1000)?),
                    token,
                }));
                match self.lexer.get_next_token()?.t {
                    TokenType::RightParen => (),
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
            op if op.is_unary_prefix_op() && op.get_unary_op_prec(&self.lexer)? <= max_op_prec => {
                let token = self.lexer.get_current_token();
                self.lexer.get_next_token()?;
                let innards = self.parse_clause(op.get_unary_op_prec(&self.lexer)?)?;
                match op {
                    TokenType::OpMns => {
                        ret = Some(Box::new(AST {
                            t: ASTType::OpMnsPrefix(innards),
                            token,
                        }));
                    }
                    TokenType::OpNot => {
                        ret = Some(Box::new(AST {
                            t: ASTType::OpNot(innards),
                            token,
                        }));
                    }
                    _ => {
                        return Err(Box::new(ParserError::new(
                            format!("Unusable unary prefix operator {:?}", op),
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
        loop {
            match self.lexer.peek_next_token()?.t {
                TokenType::LeftParen => {
                    self.lexer.get_next_token()?;
                    let token = self.lexer.get_current_token();
                    let mut args: Vec<Box<AST>> = vec![];
                    loop {
                        if args.len() == 0
                            && self.lexer.get_next_token()?.t == TokenType::RightParen
                        {
                            break;
                        }
                        args.push(self.parse_clause(1000)?);
                        match self.lexer.get_next_token()?.t {
                            TokenType::RightParen => break,
                            TokenType::Comma => _ = self.lexer.get_next_token()?,
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
                    ret = Some(Box::new(AST {
                        token,
                        t: ASTType::FunctionCall(ret.unwrap(), args),
                    }));
                }
                TokenType::LeftSquare => {
                    let token = self.lexer.get_next_token()?;
                    _ = self.lexer.get_next_token()?;
                    let index = self.parse_clause(1000)?;
                    match self.lexer.get_next_token()?.t {
                        TokenType::RightSquare => (),
                        _ => {
                            return Err(Box::new(ParserError::new(
                                format!(
                                    "Expected right square bracket, got {:?} on right side of '['",
                                    self.lexer.get_current_token()
                                ),
                                self.lexer.get_line(),
                            )))
                        }
                    }
                    ret = Some(Box::new(AST {
                        t: ASTType::BracketAccess(ret.unwrap(), index),
                        token,
                    }));
                }
                TokenType::Dot => {
                    let token = self.lexer.get_next_token()?;
                    let name = self.lexer.get_next_token()?;
                    match name.t {
                        TokenType::Ident(name) => {
                            ret = Some(Box::new(AST {
                                t: ASTType::DotAccess(ret.unwrap(), name),
                                token,
                            }));
                        }
                        _ => {
                            return Err(Box::new(ParserError::new(
                                format!(
                                    "Expected identifier, got {:?} on right side of '.'",
                                    self.lexer.get_current_token()
                                ),
                                self.lexer.get_line(),
                            )))
                        }
                    }
                }
                _ => break,
            }
        }
        while self.lexer.peek_next_token()?.t.is_op()
            && usable_operator(&self.lexer.peek_next_token()?.t, max_op_prec, &self.lexer)?
        {
            let op = self.lexer.get_next_token()?;
            _ = self.lexer.get_next_token()?;
            let rhs = self.parse_clause(op.t.get_op_prec(&self.lexer)?)?;

            ret = Some(match op.t {
                TokenType::OpEqEq => Box::new(AST{t:ASTType::OpEqEq(ret.unwrap(), rhs), token: op}),
                TokenType::OpEq => Box::new(AST{t:ASTType::OpEq(ret.unwrap(), rhs), token: op}),
                TokenType::OpMns => Box::new(AST{t:ASTType::OpMns(ret.unwrap(), rhs), token: op}),
                TokenType::OpPls => Box::new(AST{t:ASTType::OpPls(ret.unwrap(), rhs), token: op}),
                TokenType::OpTimes => Box::new(AST{t:ASTType::OpTimes(ret.unwrap(), rhs), token: op}),
                TokenType::OpDiv => Box::new(AST{t:ASTType::OpDiv(ret.unwrap(), rhs), token: op}),
                TokenType::OpPlsEq => Box::new(AST{t:ASTType::OpPlsEq(ret.unwrap(), rhs), token: op}),
                TokenType::OpNotEq => Box::new(AST{t:ASTType::OpNotEq(ret.unwrap(), rhs), token: op}),
                TokenType::OpGt => Box::new(AST{t:ASTType::OpGt(ret.unwrap(), rhs), token: op}),
                TokenType::OpLt => Box::new(AST{t:ASTType::OpLt(ret.unwrap(), rhs), token: op}),
                TokenType::OpGtEq => Box::new(AST{t:ASTType::OpGtEq(ret.unwrap(), rhs), token: op}),
                TokenType::OpLtEq => Box::new(AST{t:ASTType::OpLtEq(ret.unwrap(), rhs), token: op}),
                TokenType::OpAndAnd => {
                    Box::new(AST{t:ASTType::OpAndAnd(ret.unwrap(), rhs), token: op})
                }
                TokenType::OpOrOr => Box::new(AST{t:ASTType::OpOrOr(ret.unwrap(), rhs), token: op}),
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
        let var_decl = match token.t {
            TokenType::Ident(ref name) => Box::new(AST {
                t: ASTType::VariableDeclaration(name.to_string(), is_const),
                token,
            }),
            _ => {
                return Err(Box::new(ParserError::new(
                    format!("Expected identifier, got {:?}", token),
                    self.lexer.get_line(),
                )))
            }
        };

        match self.lexer.get_next_token()?.t {
            TokenType::OpEq => {
                let token = self.lexer.get_current_token();
                self.lexer.get_next_token()?;
                let expr = self.parse_clause(TokenType::OpEq.get_op_prec(&self.lexer)?)?;
                Ok(Box::new(AST {
                    t: ASTType::OpEq(var_decl, expr),
                    token,
                }))
            }
            TokenType::EndOfStatement | TokenType::EOF => {
                self.lexer.feed_token(Token {
                    t: TokenType::EndOfStatement,
                    line: self.lexer.get_line(),
                    char_start: 0,
                    char_end: 0,
                });
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
    //         TokenType::Var => self.parse_variable_declaration()?,
    //         TokenType::Ident(_) => {
    //             let ast = self.parse_clause(1000)?;
    //             self.lexer.get_next_token()?;
    //             ast
    //         }
    //         _ => return Err(format!("Token not yet implemented in parser: {:?}", token).into()),
    //     })
    // }

    fn parse_block(&mut self) -> Result<Vec<Box<AST>>, Box<dyn MapleError>> {
        while self.lexer.get_current_token().t == TokenType::EndOfStatement {
            self.lexer.get_next_token()?;
        }
        match self.lexer.get_current_token().t {
            TokenType::LeftBrace => (),
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
        match self.lexer.get_next_token()?.t {
            TokenType::EndOfStatement => (),
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
        let token = self.lexer.get_current_token();
        self.lexer.get_next_token()?;
        let (cond, body) = self.parse_condition_and_block()?;
        self.lexer.get_next_token()?;
        self.lexer.feed_token(Token {
            t: TokenType::EndOfStatement,
            line: self.lexer.get_line(),
            char_start: 0,
            char_end: 0,
        });
        Ok(Box::new(AST {
            t: ASTType::While(cond, body),
            token,
        }))
    }
    fn parse_if(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        let token = self.lexer.get_current_token();
        self.lexer.get_next_token()?;
        let (cond, body) = self.parse_condition_and_block()?;
        let mut elseifs: Vec<(Box<AST>, Vec<Box<AST>>)> = vec![];
        self.lexer.get_next_token()?;
        let mut feed_token = Token {
            t: TokenType::EndOfStatement,
            line: self.lexer.get_line(),
            char_start: 0,
            char_end: 0,
        };
        loop {
            while self.lexer.get_current_token().t == TokenType::EndOfStatement {
                self.lexer.get_next_token()?;
            }
            if self.lexer.get_current_token().t != TokenType::Elseif {
                break;
            }
            self.lexer.get_next_token()?;
            let block = self.parse_condition_and_block()?;
            elseifs.push(block);
            self.lexer.get_next_token()?;
        }
        while self.lexer.get_current_token().t == TokenType::EndOfStatement {
            self.lexer.get_next_token()?;
        }
        let else_body = if self.lexer.get_current_token().t == TokenType::Else {
            self.lexer.get_next_token()?;
            let body = self.parse_block()?;
            Some(body)
        } else {
            feed_token = self.lexer.get_current_token();
            None
        };
        self.lexer.feed_token(Token {
            t: TokenType::EndOfStatement,
            line: self.lexer.get_line(),
            char_start: 0,
            char_end: 0,
        });
        if feed_token.t != TokenType::EndOfStatement {
            self.lexer.feed_token(feed_token);
        }

        // parse takes care of }, so we don't need to check for it here
        Ok(Box::new(AST {
            t: ASTType::If(IfLiteral {
                cond,
                body,
                elseifs,
                else_body,
            }),
            token,
        }))
    }

    fn parse_break(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        let token = self.lexer.get_current_token();
        // self.lexer.get_next_token()?;
        Ok(Box::new(AST {
            t: ASTType::Break,
            token,
        }))
    }
    fn parse_continue(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        let token = self.lexer.get_current_token();
        // self.lexer.get_next_token()?;
        Ok(Box::new(AST {
            t: ASTType::Continue,
            token,
        }))
    }
    fn parse_return(&mut self) -> Result<Box<AST>, Box<dyn MapleError>> {
        let token = self.lexer.get_current_token();
        self.lexer.get_next_token()?;
        let expr = self.parse_clause(1000)?;
        // self.lexer.get_next_token()?;
        Ok(Box::new(AST {
            t: ASTType::Return(expr),
            token,
        }))
    }
    pub fn parse(&mut self, top_level: bool) -> Result<Vec<Box<AST>>, Box<dyn MapleError>> {
        let mut ret: Vec<Box<AST>> = vec![];
        if top_level {
            self.lexer.get_next_token()?;
        }
        loop {
            let ast = match self.lexer.get_current_token().t {
                TokenType::Fn => Some(self.parse_function(false)?),
                TokenType::Break => Some(self.parse_break()?),
                TokenType::Continue => Some(self.parse_continue()?),
                TokenType::Return => Some(self.parse_return()?),
                TokenType::Const => Some(self.parse_variable_declaration(true)?),
                TokenType::Var => Some(self.parse_variable_declaration(false)?),
                TokenType::Ident(_) => {
                    let ast = self.parse_clause(1000)?;
                    // self.lexer.get_next_token()?;
                    Some(ast)
                }
                TokenType::While => Some(self.parse_while()?),
                TokenType::If => Some(self.parse_if()?),
                TokenType::EOF if top_level => break,
                TokenType::EOF if !top_level => {
                    return Err(Box::new(ParserError::new(
                        "Unexpected EOF (aka unclosed brace)".into(),
                        self.lexer.get_line(),
                    )));
                }
                TokenType::RightBrace if !top_level => {
                    break;
                }
                TokenType::RightBrace if top_level => {
                    return Err(Box::new(ParserError::new(
                        "Unexpected right brace in top level".into(),
                        self.lexer.get_line(),
                    )));
                }
                TokenType::EndOfStatement => None,
                _ => {
                    return Err(Box::new(ParserError::new(
                        format!("Unexpected token {:?}", self.lexer.get_current_token()),
                        self.lexer.get_line(),
                    )))
                }
            };
            match self.lexer.get_next_token()?.t {
                TokenType::RightBrace if !top_level => {
                    break;
                }
                TokenType::RightBrace if top_level => {
                    return Err(Box::new(ParserError::new(
                        "Unexpected right brace in top level".into(),
                        self.lexer.get_line(),
                    )));
                }
                TokenType::EndOfStatement => (),
                TokenType::EOF if !top_level => {
                    return Err(Box::new(ParserError::new(
                        "Unexpected EOF (aka unclosed brace)".into(),
                        self.lexer.get_line(),
                    )));
                }
                TokenType::EOF => break,
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
