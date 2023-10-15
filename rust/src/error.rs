#![allow(dead_code)]

use crate::ast::AST;

pub trait MapleError: std::fmt::Debug + std::fmt::Display + std::error::Error {
    fn get_line(&self) -> usize;
    fn get_msg(&self) -> String {
        format!("{}", self)
    }
    fn set_msg(&mut self, msg: String);
}

#[derive(Debug, Clone)]
pub struct LexerError {
    msg: String,
    line: usize,
}

impl std::fmt::Display for LexerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Lexer error at line {}: {}", self.line, self.msg)
    }
}

impl MapleError for LexerError {
    fn get_line(&self) -> usize {
        self.line
    }
    fn set_msg(&mut self, msg: String) {
        self.msg = msg;
    }
}

impl std::error::Error for LexerError {}

impl LexerError {
    pub fn new(msg: String, line: usize) -> LexerError {
        LexerError { msg, line }
    }
}

#[derive(Debug, Clone)]
pub struct ParserError {
    msg: String,
    line: usize,
}

impl std::fmt::Display for ParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Parser error at line {}: {}", self.line, self.msg)
    }
}

impl MapleError for ParserError {
    fn get_line(&self) -> usize {
        self.line
    }
    fn set_msg(&mut self, msg: String) {
        self.msg = msg;
    }
}

impl std::error::Error for ParserError {}

impl ParserError {
    pub fn new(msg: String, line: usize) -> ParserError {
        ParserError { msg, line }
    }
}

#[derive(Debug, Clone)]
pub struct RuntimeError {
    msg: String,
    line: usize,
    current_ast: AST,
    base_asts: Vec<AST>,
}

impl std::fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let ast_and_line = self
            .base_asts
            .iter()
            .map(|e| (e.get_line(), e.pretty_print()))
            .map(|(line, ast)| format!("{:?} at line {}", ast, line))
            .collect::<Vec<String>>();
        write!(
            f,
            "\nRuntime error at line {}: {}\n while evaluating: {:?}\n while evaluating: {}",
            self.line,
            self.msg,
            self.current_ast.pretty_print(),
            ast_and_line.join("\n while evaluating: ")
        )
    }
}

impl std::error::Error for RuntimeError {}

impl RuntimeError {
    pub fn new(msg: String, line: usize, current_ast: AST) -> RuntimeError {
        RuntimeError {
            msg,
            line,
            current_ast,
            base_asts: vec![],
        }
    }
    pub fn add_base_ast(&mut self, base_ast: AST) {
        self.base_asts.push(base_ast);
    }
}

impl MapleError for RuntimeError {
    fn get_line(&self) -> usize {
        self.line
    }
    fn set_msg(&mut self, msg: String) {
        self.msg = msg;
    }
}

#[derive(Debug, Clone)]
pub struct ScopeError {
    msg: String,
    line: usize,
}

impl std::fmt::Display for ScopeError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Scope error at line {}: {}", self.line, self.msg)
    }
}

impl std::error::Error for ScopeError {}

impl ScopeError {
    pub fn new(msg: String, line: usize) -> ScopeError {
        ScopeError { msg, line }
    }
    pub fn to_runtime_error(&self, current_ast: AST) -> RuntimeError {
        RuntimeError::new(self.msg.clone(), self.line, current_ast)
    }
}
