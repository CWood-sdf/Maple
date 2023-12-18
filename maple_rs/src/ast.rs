#![allow(dead_code)]
use crate::error::{RuntimeError, ScopeError};
use crate::lexer::Token;
use crate::parser::{Object, ObjectKey, Unpack, Value};
use crate::runtime::*;
use crate::scopechain::ReturnType;
use crate::scopechain::ScopeChain;

use std::rc::Rc;

pub trait ConvertScopeErrorResult<T> {
    fn to_runtime_error(&self) -> Result<&T, Box<RuntimeError>>;
}

impl<T> ConvertScopeErrorResult<T> for Result<T, ScopeError> {
    fn to_runtime_error(&self) -> Result<&T, Box<RuntimeError>> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e.to_runtime_error())),
        }
    }
}

pub type Block = Vec<Box<AST>>;

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinFunction {
    pub param_count: usize,
    pub func: fn(Vec<Rc<Value>>) -> Result<Rc<Value>, Box<RuntimeError>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FnParam {
    pub name: String,
    pub char_start: usize,
    pub char_end: usize,
    pub line: usize,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionLiteral {
    pub params: Vec<FnParam>,
    pub body: Block,
    pub closure: ScopeChain,
}
impl FunctionLiteral {
    pub fn new(params: Vec<FnParam>, body: Block, scope_chain: &ScopeChain) -> FunctionLiteral {
        FunctionLiteral {
            params,
            body,
            closure: scope_chain.get_closure(),
        }
    }
    pub fn basic(params: Vec<FnParam>, body: Block) -> FunctionLiteral {
        FunctionLiteral {
            params,
            body,
            closure: ScopeChain::new(),
        }
    }
    pub fn make_real(&self, scope_chain: &ScopeChain) -> FunctionLiteral {
        FunctionLiteral {
            params: self.params.clone(),
            body: self.body.clone(),
            closure: scope_chain.get_closure(),
        }
    }

    pub fn pretty_print(&self) -> String {
        format!(
            "fn ({}) {{\n{}\n}}",
            self.params
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<String>>()
                .join(", "),
            self.body
                .iter()
                .map(|ast| format!("    {}\n", ast.pretty_print()))
                .collect::<Vec<String>>()
                .join("")
        )
    }
    pub fn call(
        &self,
        scope_chain: &mut ScopeChain,
        params: &Block,
        line: usize,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let params_value = params
            .iter()
            .map(|ast| {
                ast.get_value(scope_chain)
                    .unpack_and_transform(scope_chain, line, ast)
            })
            .collect::<Vec<Result<Rc<Value>, Box<RuntimeError>>>>();
        scope_chain.add_fn_scope(&self.closure);
        if params.len() != self.params.len() {
            return Err(Box::new(RuntimeError::new(
                format!(
                    "Expected {} parameters, got {}",
                    self.params.len(),
                    params.len()
                ),
                line,
            )));
        }
        for (i, param) in self.params.iter().enumerate() {
            scope_chain
                .add_variable(&param.name, false, line)
                .to_runtime_error()?;
            let param_value = params_value[i].clone()?;
            scope_chain
                .set_variable(&param.name, param_value, line)
                .to_runtime_error()?;
        }
        for ast in self.body.iter() {
            ast.get_value(scope_chain)?;
            match scope_chain.get_return_register() {
                ReturnType::None => (),
                _ => break,
            }
        }
        match scope_chain.pop_fn_scope(line) {
            Ok(_) => (),
            Err(e) => return Err(Box::new(e.to_runtime_error())),
        };
        match scope_chain.get_return_register() {
            ReturnType::None => Ok(Rc::new(Value::Undefined)),
            ReturnType::Return(value) => {
                scope_chain.eat_return_register();
                Ok(value)
            }
            ReturnType::Break => Err(Box::new(RuntimeError::new(
                "Cannot call 'break' inside a function".into(),
                line,
            ))),
            ReturnType::Continue => Err(Box::new(RuntimeError::new(
                "Cannot call 'continue' inside a function".into(),
                line,
            ))),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct IfLiteral {
    pub cond: Box<AST>,
    pub body: Block,
    pub elseifs: Vec<(Box<AST>, Block)>,
    pub else_body: Option<Block>,
}
impl IfLiteral {
    pub fn new(
        cond: Box<AST>,
        body: Block,
        elseifs: Vec<(Box<AST>, Block)>,
        else_body: Option<Block>,
    ) -> Result<IfLiteral, Box<RuntimeError>> {
        Ok(IfLiteral {
            cond,
            body,
            elseifs,
            else_body,
        })
    }
    pub fn pretty_print(&self) -> String {
        let mut ret = format!(
            "if {} {{\n{}\n}}",
            self.cond.pretty_print(),
            self.body
                .iter()
                .map(|ast| format!("    {}\n", ast.pretty_print()))
                .collect::<Vec<String>>()
                .join("")
        );
        for (elseif_cond, elseif_body) in self.elseifs.iter() {
            ret += &format!(
                "elseif {} {{\n{}\n}}",
                elseif_cond.pretty_print(),
                elseif_body
                    .iter()
                    .map(|ast| format!("    {}\n", ast.pretty_print()))
                    .collect::<Vec<String>>()
                    .join("")
            );
        }
        if let Some(else_body) = &self.else_body {
            ret += &format!(
                "else {{\n{}\n}}",
                else_body
                    .iter()
                    .map(|ast| format!("    {}\n", ast.pretty_print()))
                    .collect::<Vec<String>>()
                    .join("")
            );
        }
        ret
    }
}
// paren is only for pretty printing
#[derive(Debug, Clone, PartialEq)]
pub enum ASTType {
    Import(String),
    DotAccess(Box<AST>, String),
    BracketAccess(Box<AST>, Box<AST>),
    ObjectLiteral(Vec<(ObjectKey, Box<AST>)>),
    ArrayLiteral(Vec<Box<AST>>),
    CharacterLiteral(char),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Paren(Box<AST>),
    VariableDeclaration(String, bool),
    FunctionLiteral(FunctionLiteral),
    FunctionCall(Box<AST>, Block),
    If(IfLiteral),
    While(Box<AST>, Block),
    OpPls(Box<AST>, Box<AST>),    // +
    OpMns(Box<AST>, Box<AST>),    // -
    OpTimes(Box<AST>, Box<AST>),  // *
    OpDiv(Box<AST>, Box<AST>),    // /
    OpMnsPrefix(Box<AST>),        // -
    OpEq(Box<AST>, Box<AST>),     // =
    OpEqEq(Box<AST>, Box<AST>),   // ==
    OpPlsEq(Box<AST>, Box<AST>),  // +=
    OpNotEq(Box<AST>, Box<AST>),  // !=
    OpNot(Box<AST>),              // !
    OpAndAnd(Box<AST>, Box<AST>), // &&
    OpOrOr(Box<AST>, Box<AST>),   // ||
    OpGt(Box<AST>, Box<AST>),     // >
    OpLt(Box<AST>, Box<AST>),     // <
    OpGtEq(Box<AST>, Box<AST>),   // >=
    OpLtEq(Box<AST>, Box<AST>),   // <=
    VariableAccess(String),
    Return(Box<AST>),
    Break,
    Continue,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AST {
    pub t: ASTType,
    pub token: Token,
}

impl AST {
    pub fn get_line(&self) -> usize {
        self.token.line + 1
    }

    pub fn get_value(&self, scope_chain: &mut ScopeChain) -> Result<Rc<Value>, Box<RuntimeError>> {
        let line = self.get_line();
        let ret = match &self.t {
            ASTType::Import(s) => eval_import(s.clone()),
            ASTType::ArrayLiteral(arr) => {
                let mut obj = Object::new();
                let mut i = 0;
                for value in arr.iter() {
                    let value = value.get_value(scope_chain)?.unpack_and_transform(
                        scope_chain,
                        value.get_line(),
                        value,
                    )?;
                    obj.set(ObjectKey::Number(i as f64), value);
                    i += 1;
                }
                Ok(Rc::new(Value::Object(obj)))
            }
            ASTType::ObjectLiteral(arr) => {
                let mut obj = Object::new();
                for (key, value) in arr.iter() {
                    let value = value.get_value(scope_chain)?.unpack_and_transform(
                        scope_chain,
                        value.get_line(),
                        value,
                    )?;
                    obj.set(key.clone(), value);
                }
                Ok(Rc::new(Value::Object(obj)))
            }
            ASTType::DotAccess(left, name) => Ok(Rc::new(Value::ObjectAccess(
                left.get_value(scope_chain)?
                    .unpack_and_transform(scope_chain, line, self)?,
                ObjectKey::String(name.to_string()),
            ))),
            ASTType::BracketAccess(left, value) => {
                let val = value.get_value(scope_chain)?.unpack_and_transform(
                    scope_chain,
                    value.get_line(),
                    value,
                )?;
                let key = match val.as_ref() {
                    Value::Number(n) => ObjectKey::Number(*n),
                    Value::String(s) => ObjectKey::String(s.clone()),
                    Value::Char(c) => ObjectKey::String(c.to_string()),
                    v => return Err(Box::new(RuntimeError::new(format!("Cannot use type {} as an object key, can only use char, string, or number types", v.pretty_type(scope_chain, line)), line))),
                };
                Ok(Rc::new(Value::ObjectAccess(
                    left.get_value(scope_chain)?
                        .unpack_and_transform(scope_chain, line, self)?,
                    key,
                )))
            }
            ASTType::Return(v) => eval_return(v, scope_chain),
            ASTType::Break => match scope_chain
                .set_return_register(ReturnType::Break)
                .to_runtime_error()
            {
                Ok(_) => Ok(Rc::new(Value::Undefined)),
                Err(e) => return Err(e),
            },
            ASTType::Continue => {
                scope_chain
                    .set_return_register(ReturnType::Continue)
                    .to_runtime_error()?;
                Ok(Rc::new(Value::Undefined))
            }
            ASTType::FunctionCall(func, params) => {
                let func =
                    func.get_value(scope_chain)
                        .unpack_and_transform(scope_chain, line, self)?;
                match func.as_ref() {
                    Value::Function(func) => func.call(scope_chain, params, line),
                    Value::BuiltinFunction(f, arg_len) => {
                        if arg_len != &params.len() {
                            return Err(Box::new(RuntimeError::new(
                                format!("Expected {} arguments, got {}", arg_len, params.len()),
                                line,
                            )));
                        }
                        // convert params to values
                        let mut params = params
                            .iter()
                            .map(|ast| {
                                ast.get_value(scope_chain).unpack_and_transform(
                                    scope_chain,
                                    ast.get_line(),
                                    ast,
                                )
                            })
                            .collect::<Vec<Result<Rc<Value>, Box<RuntimeError>>>>();

                        let mut actual_params = vec![];
                        for param in params.iter_mut() {
                            match param {
                                Ok(v) => actual_params.push(v.clone()),
                                Err(e) => return Err(e.clone()),
                            }
                        }
                        Ok(f(actual_params, self, scope_chain, self.get_line())?)
                    }

                    _ => {
                        return Err(Box::new(RuntimeError::new(
                            "Cannot call a non-function".into(),
                            line,
                        )))
                    }
                }
            }
            ASTType::BooleanLiteral(b) => Ok(Rc::new(Value::Boolean(*b))),
            ASTType::If(if_lit) => eval_if(if_lit, &self, scope_chain),
            ASTType::While(cond, block) => eval_while(cond, block, scope_chain),
            ASTType::FunctionLiteral(f) => Ok(Rc::new(Value::Function(f.make_real(scope_chain)))),
            ASTType::StringLiteral(str) => Ok(Rc::new(Value::String(str.to_string()))),
            ASTType::NumberLiteral(num) => Ok(Rc::new(Value::Number(*num))),
            ASTType::VariableDeclaration(name, is_const) => {
                match scope_chain
                    .add_variable(name, *is_const, self.get_line())
                    .to_runtime_error()
                {
                    Ok(_) => Ok(Rc::new(Value::Variable(name.clone()))),
                    Err(e) => return Err(e),
                }
            }
            ASTType::OpAndAnd(left, right) => eval_op_andand(left, right, scope_chain),
            ASTType::OpOrOr(left, right) => eval_op_oror(left, right, scope_chain),
            ASTType::OpNotEq(left, right) => eval_op_noteq(left, right, scope_chain),
            ASTType::OpNot(right) => eval_op_not(right, scope_chain),
            ASTType::OpPls(left, right) => eval_op_pls(left, right, scope_chain),
            ASTType::OpMns(left, right) => eval_op_mns(left, right, scope_chain),
            ASTType::OpTimes(left, right) => eval_op_times(left, right, scope_chain),
            ASTType::OpDiv(left, right) => eval_op_div(left, right, scope_chain),
            ASTType::OpMnsPrefix(left) => eval_op_mns_prefix(left, scope_chain),
            ASTType::OpPlsEq(left, right) => eval_op_plseq(left, right, scope_chain),
            ASTType::OpEq(left, right) => eval_op_eq(left, right, scope_chain),
            ASTType::OpGt(left, right) => eval_op_gt(left, right, scope_chain),
            ASTType::OpLt(left, right) => eval_op_lt(left, right, scope_chain),
            ASTType::OpGtEq(left, right) => eval_op_gteq(left, right, scope_chain),
            ASTType::OpLtEq(left, right) => eval_op_lteq(left, right, scope_chain),
            ASTType::CharacterLiteral(c) => Ok(Rc::new(Value::Char(*c))),
            ASTType::OpEqEq(left, right) => eval_op_eqeq(left, right, scope_chain),
            ASTType::VariableAccess(name) => Ok(Value::Variable(name.clone()).into()),
            ASTType::Paren(ast) => ast.get_value(scope_chain),
        };

        match ret {
            Ok(v) => Ok(v),
            Err(mut e) => {
                e.add_base_ast(self.clone());
                Err(e)
            }
        }
    }

    pub fn interpret(&self, scope_chain: &mut ScopeChain) -> Result<ReturnType, Box<RuntimeError>> {
        self.get_value(scope_chain)?;
        match scope_chain.get_return_register() {
            // ReturnType::Return => Err(Box::new(
            //     RuntimeError::new("Return statement at top level".into(), self.get_line())
            //         .add_base_ast(self.clone()),
            // )),
            ReturnType::Continue => Err(Box::new(
                RuntimeError::new("Continue statement at top level".into(), self.get_line())
                    .add_base_ast(self.clone()),
            )),
            ReturnType::Break => Err(Box::new(
                RuntimeError::new("Break statement at top level".into(), self.get_line())
                    .add_base_ast(self.clone()),
            )),
            ReturnType::Return(v) => Ok(ReturnType::Return(v)),
            ReturnType::None => Ok(ReturnType::None),
        }
    }
    pub fn debug_pretty_print(&self) -> String {
        match &self.t {
            ASTType::ArrayLiteral(arr) => {
                format!(
                    "[{}]",
                    arr.iter()
                        .map(|ast| ast.debug_pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ASTType::Import(s) => format!("(import {})", s),
            ASTType::ObjectLiteral(_) => todo!(),
            ASTType::BracketAccess(l, v) => {
                format!("{}[{}]", l.debug_pretty_print(), v.debug_pretty_print())
            }
            ASTType::DotAccess(l, v) => format!("{}.{}", l.debug_pretty_print(), v),
            ASTType::Return(v) => format!("return {}", v.debug_pretty_print()),
            ASTType::Break => "break".to_string(),
            ASTType::Continue => "continue".to_string(),
            ASTType::FunctionCall(func, params) => {
                format!(
                    "({}({}))",
                    func.debug_pretty_print(),
                    params
                        .iter()
                        .map(|ast| ast.debug_pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ASTType::BooleanLiteral(b) => b.to_string(),
            ASTType::While(c, block) => {
                format!(
                    "while {} {{\n{}\n}}",
                    c.debug_pretty_print(),
                    block
                        .iter()
                        .map(|ast| format!("    {}\n", ast.debug_pretty_print()))
                        .collect::<Vec<String>>()
                        .join("")
                )
            }
            ASTType::If(if_lit) => if_lit.pretty_print(),
            ASTType::FunctionLiteral(func) => func.pretty_print(),
            ASTType::StringLiteral(value) => format!("\"{}\"", value),
            ASTType::NumberLiteral(value) => value.to_string(),
            ASTType::OpAndAnd(left, right) => {
                format!(
                    "({} && {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::OpOrOr(left, right) => {
                format!(
                    "({} || {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::OpNot(left) => {
                format!("!({})", left.debug_pretty_print(),)
            }
            ASTType::OpMnsPrefix(left) => {
                format!("-({})", left.debug_pretty_print(),)
            }
            ASTType::OpMns(left, right) => {
                format!(
                    "({} - {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::OpPls(left, right) => {
                format!(
                    "({} + {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::OpTimes(left, right) => {
                format!("({} * {})", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpDiv(left, right) => {
                format!("({} / {})", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpPlsEq(left, right) => {
                format!(
                    "({} += {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::OpGt(left, right) => {
                format!(
                    "({} > {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::OpLt(left, right) => {
                format!(
                    "({} < {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::OpGtEq(left, right) => {
                format!(
                    "({} >= {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::OpLtEq(left, right) => {
                format!(
                    "({} <= {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::VariableDeclaration(name, is_const) if !*is_const => format!("var {}", name),
            ASTType::VariableDeclaration(name, _) => format!("const {}", name),
            ASTType::OpEq(left, right) => {
                format!(
                    "({} = {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::CharacterLiteral(c) => format!("'{}'", c),
            ASTType::OpEqEq(left, right) => {
                format!(
                    "({} == {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::OpNotEq(left, right) => {
                format!(
                    "({} != {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            ASTType::VariableAccess(name) => name.to_string(),
            ASTType::Paren(ast) => format!("({})", ast.debug_pretty_print()),
        }
    }

    pub fn pretty_print(&self) -> String {
        match &self.t {
            ASTType::ArrayLiteral(arr) => {
                format!(
                    "[{}]",
                    arr.iter()
                        .map(|ast| ast.pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ASTType::Import(s) => format!("import {}", s),
            ASTType::ObjectLiteral(s) => {
                let mut ret = "{\n".to_string();
                for (key, value) in s {
                    ret += &format!(
                        "  {} = {},\n",
                        match key {
                            ObjectKey::String(s) => format!("{}", s),
                            ObjectKey::Number(n) => n.to_string(),
                        },
                        value.pretty_print()
                    );
                }

                ret += "}";
                ret
            }
            ASTType::BracketAccess(l, v) => {
                format!("{}[{}]", l.pretty_print(), v.pretty_print())
            }
            ASTType::DotAccess(l, v) => format!("{}.{}", l.pretty_print(), v),
            ASTType::Return(v) => format!("return {}", v.pretty_print()),
            ASTType::Break => "break".to_string(),
            ASTType::Continue => "continue".to_string(),
            ASTType::BooleanLiteral(b) => b.to_string(),
            ASTType::FunctionCall(func, params) => {
                format!(
                    "{}({})",
                    func.pretty_print(),
                    params
                        .iter()
                        .map(|ast| ast.pretty_print())
                        .collect::<Vec<String>>()
                        .join(", ")
                )
            }
            ASTType::While(c, block) => {
                format!(
                    "while {} {{\n{}\n}}",
                    c.pretty_print(),
                    block
                        .iter()
                        .map(|ast| format!("    {}\n", ast.pretty_print()))
                        .collect::<Vec<String>>()
                        .join("")
                )
            }
            ASTType::If(if_lit) => if_lit.pretty_print(),
            ASTType::FunctionLiteral(func) => func.pretty_print(),
            ASTType::StringLiteral(value) => format!("\"{}\"", value),
            ASTType::NumberLiteral(value) => value.to_string(),
            ASTType::OpAndAnd(left, right) => {
                format!("{} && {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpOrOr(left, right) => {
                format!("{} || {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpNot(left) => {
                format!("!{}", left.pretty_print())
            }
            ASTType::OpMnsPrefix(left) => {
                format!("-{}", left.pretty_print())
            }
            ASTType::OpMns(left, right) => {
                format!("{} - {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpPls(left, right) => {
                format!("{} + {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpTimes(left, right) => {
                format!("{} * {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpDiv(left, right) => {
                format!("{} / {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpPlsEq(left, right) => {
                format!("{} += {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpGt(left, right) => {
                format!("{} > {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpLt(left, right) => {
                format!("{} < {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpGtEq(left, right) => {
                format!("{} >= {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpLtEq(left, right) => {
                format!("{} <= {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::VariableDeclaration(name, is_const) if !is_const => format!("var {}", name),
            ASTType::VariableDeclaration(name, _) => format!("const {}", name),
            ASTType::OpEq(left, right) => {
                format!("{} = {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::CharacterLiteral(c) => format!("'{}'", c),
            ASTType::OpEqEq(left, right) => {
                format!("{} == {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::OpNotEq(left, right) => {
                format!("{} != {}", left.pretty_print(), right.pretty_print())
            }
            ASTType::VariableAccess(name) => name.to_string(),
            ASTType::Paren(ast) => format!("({})", ast.pretty_print()),
        }
    }
}
