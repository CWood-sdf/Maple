#![allow(dead_code)]
use crate::error::{RuntimeError, ScopeError};
use crate::parser::{Unpack, Value};
use crate::scopechain::ReturnType;
use crate::ScopeChain;

use std::rc::Rc;

pub trait ConvertScopeErrorResult<T> {
    fn to_runtime_error(&self, ast: &AST) -> Result<T, Box<RuntimeError>>;
}

impl<T> ConvertScopeErrorResult<T> for Result<T, ScopeError>
where
    T: Copy,
{
    fn to_runtime_error(&self, ast: &AST) -> Result<T, Box<RuntimeError>> {
        match self {
            Ok(v) => Ok(*v),
            Err(e) => Err(Box::new(e.to_runtime_error(ast.clone()))),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinFunction {
    pub param_count: usize,
    pub func: fn(Vec<Rc<Value>>) -> Result<Rc<Value>, Box<RuntimeError>>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct FunctionLiteral {
    pub params: Vec<String>,
    pub body: Vec<Box<AST>>,
    pub closure: ScopeChain,
}
impl FunctionLiteral {
    pub fn new(
        params: Vec<String>,
        body: Vec<Box<AST>>,
        scope_chain: &ScopeChain,
    ) -> FunctionLiteral {
        FunctionLiteral {
            params,
            body,
            closure: scope_chain.get_closure(),
        }
    }
    pub fn basic(params: Vec<String>, body: Vec<Box<AST>>) -> FunctionLiteral {
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
            self.params.join(", "),
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
        params: &Vec<Box<AST>>,
        call_ast: &AST,
        line: usize,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        scope_chain.add_fn_scope(&self.closure);
        if params.len() != self.params.len() {
            return Err(Box::new(RuntimeError::new(
                format!(
                    "Expected {} parameters, got {}",
                    self.params.len(),
                    params.len()
                ),
                line,
                call_ast.clone(),
            )));
        }
        for (i, param) in self.params.iter().enumerate() {
            scope_chain
                .add_variable(param, false, line)
                .to_runtime_error(call_ast)?;
            let param_value = params[i].get_value(scope_chain)?;
            let param_unpacked = param_value.unpack_and_transform(scope_chain, line, &params[i])?;
            scope_chain
                .set_variable(param, param_unpacked, line)
                .to_runtime_error(call_ast)?;
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
            Err(e) => return Err(Box::new(e.to_runtime_error(call_ast.clone()))),
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
                call_ast.clone(),
            ))),
            ReturnType::Continue => Err(Box::new(RuntimeError::new(
                "Cannot call 'continue' inside a function".into(),
                line,
                call_ast.clone(),
            ))),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct IfLiteral {
    pub cond: Box<AST>,
    pub body: Vec<Box<AST>>,
    pub elseifs: Vec<(Box<AST>, Vec<Box<AST>>)>,
    pub else_body: Option<Vec<Box<AST>>>,
}
impl IfLiteral {
    pub fn new(
        cond: Box<AST>,
        body: Vec<Box<AST>>,
        elseifs: Vec<(Box<AST>, Vec<Box<AST>>)>,
        else_body: Option<Vec<Box<AST>>>,
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
pub enum AST {
    CharacterLiteral(char, usize),
    StringLiteral(String, usize),
    NumberLiteral(f64, usize),
    BooleanLiteral(bool, usize),
    Paren(Box<AST>, usize),
    VariableDeclaration(String, bool, usize),
    FunctionLiteral(FunctionLiteral, usize),
    FunctionCall(Box<AST>, Vec<Box<AST>>, usize),
    If(IfLiteral, usize),
    While(Box<AST>, Vec<Box<AST>>, usize),
    OpPls(Box<AST>, Box<AST>, usize),
    OpEq(Box<AST>, Box<AST>, usize),
    OpEqEq(Box<AST>, Box<AST>, usize),
    OpPlsEq(Box<AST>, Box<AST>, usize),
    OpNotEq(Box<AST>, Box<AST>, usize),
    OpAndAnd(Box<AST>, Box<AST>, usize),
    OpOrOr(Box<AST>, Box<AST>, usize),
    OpGt(Box<AST>, Box<AST>, usize),
    OpLt(Box<AST>, Box<AST>, usize),
    OpGtEq(Box<AST>, Box<AST>, usize),
    OpLtEq(Box<AST>, Box<AST>, usize),
    VariableAccess(String, usize),
    Return(Box<AST>, usize),
    Break(usize),
    Continue(usize),
}

impl AST {
    pub fn get_line(&self) -> usize {
        match *self {
            AST::CharacterLiteral(_, line) => line,
            AST::StringLiteral(_, line) => line,
            AST::NumberLiteral(_, line) => line,
            AST::BooleanLiteral(_, line) => line,
            AST::Paren(_, line) => line,
            AST::VariableDeclaration(_, _, line) => line,
            AST::FunctionLiteral(_, line) => line,
            AST::FunctionCall(_, _, line) => line,
            AST::If(_, line) => line,
            AST::While(_, _, line) => line,
            AST::OpPls(_, _, line) => line,
            AST::OpEq(_, _, line) => line,
            AST::OpEqEq(_, _, line) => line,
            AST::OpPlsEq(_, _, line) => line,
            AST::OpNotEq(_, _, line) => line,
            AST::OpAndAnd(_, _, line) => line,
            AST::OpOrOr(_, _, line) => line,
            AST::OpGt(_, _, line) => line,
            AST::OpLt(_, _, line) => line,
            AST::OpGtEq(_, _, line) => line,
            AST::OpLtEq(_, _, line) => line,
            AST::VariableAccess(_, line) => line,
            AST::Return(_, line) => line,
            AST::Break(line) => line,
            AST::Continue(line) => line,
        }
    }
    fn eval_op_andand(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;

        match left_val.as_ref() {
            Value::Boolean(false) => return Ok(Rc::new(Value::Boolean(false))),
            Value::Boolean(true) => {}
            _ => {
                return Err(Box::new(RuntimeError::new(
                    format!(
                        "Cannot apply operator && to left side type of {}",
                        left_val.pretty_type(scope_chain, left.get_line())
                    ),
                    left.get_line(),
                    left.as_ref().clone(),
                )))
            }
        };
        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        match right_val.as_ref() {
            Value::Boolean(false) => Ok(Rc::new(Value::Boolean(false))),
            Value::Boolean(true) => Ok(Rc::new(Value::Boolean(true))),
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot && types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                right.get_line(),
                right.as_ref().clone(),
            ))),
        }
    }
    fn eval_op_oror(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;

        match left_val.as_ref() {
            Value::Boolean(true) => return Ok(Rc::new(Value::Boolean(true))),
            Value::Boolean(false) => {}
            _ => {
                return Err(Box::new(RuntimeError::new(
                    format!(
                        "Cannot apply operator || to left side type of {}",
                        left_val.pretty_type(scope_chain, left.get_line())
                    ),
                    left.get_line(),
                    left.as_ref().clone(),
                )))
            }
        };
        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        match right_val.as_ref() {
            Value::Boolean(false) => Ok(Rc::new(Value::Boolean(false))),
            Value::Boolean(true) => Ok(Rc::new(Value::Boolean(true))),
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot || types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                right.get_line(),
                right.as_ref().clone(),
            ))),
        }
    }
    fn eval_op_pls(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;
        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        match (left_val.as_ref(), right_val.as_ref()) {
            (Value::Number(left), Value::Number(right)) => Ok(Rc::new(Value::Number(left + right))),
            (Value::Char(left), Value::Char(right)) => Ok(Rc::new(Value::Number(
                *left as i32 as f64 + *right as i32 as f64,
            ))),
            (Value::Number(left), Value::Char(right)) => {
                Ok(Rc::new(Value::Number(*left + *right as i32 as f64)))
            }
            (Value::Char(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Number(*left as i32 as f64 + *right)))
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
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot add types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                left.get_line(),
                left.as_ref().clone(),
            ))),
        }
    }
    fn eval_op_noteq(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;
        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        Ok(Rc::new(Value::Boolean(left_val != right_val)))
    }
    fn eval_op_eqeq(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;
        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        Ok(Rc::new(Value::Boolean(left_val == right_val)))
    }
    fn eval_op_eq(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?;

        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        match left_val.as_ref() {
            Value::Variable(name) => {
                match scope_chain.set_variable(&name, right_val, left.get_line()) {
                    Ok(_) => (),
                    Err(e) => return Err(Box::new(e.to_runtime_error(left.as_ref().clone()))),
                };
                Ok(left_val)
            }
            _ => Err(Box::new(RuntimeError::new(
                "Cannot assign to a non-variable".into(),
                left.get_line(),
                left.as_ref().clone(),
            ))),
        }
    }
    fn eval_op_gteq(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;
        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        match (left_val.as_ref(), right_val.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Boolean(left >= right)))
            }
            (Value::Char(left), Value::Char(right)) => Ok(Rc::new(Value::Boolean(
                *left as i32 as f64 >= *right as i32 as f64,
            ))),
            (Value::Number(left), Value::Char(right)) => {
                Ok(Rc::new(Value::Boolean(*left >= *right as i32 as f64)))
            }
            (Value::Char(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Boolean(*left as i32 as f64 >= *right)))
            }
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot compare types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                left.get_line(),
                left.as_ref().clone(),
            ))),
        }
    }
    fn eval_op_lteq(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;
        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        match (left_val.as_ref(), right_val.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Boolean(left <= right)))
            }
            (Value::Char(left), Value::Char(right)) => Ok(Rc::new(Value::Boolean(*left <= *right))),
            (Value::Number(left), Value::Char(right)) => {
                Ok(Rc::new(Value::Boolean(*left <= *right as i32 as f64)))
            }
            (Value::Char(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Boolean(*left as i32 as f64 <= *right)))
            }
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot compare types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                left.get_line(),
                left.as_ref().clone(),
            ))),
        }
    }
    fn eval_op_gt(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;
        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        match (left_val.as_ref(), right_val.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Boolean(left > right)))
            }
            (Value::Char(left), Value::Char(right)) => Ok(Rc::new(Value::Boolean(*left > *right))),
            (Value::Number(left), Value::Char(right)) => {
                Ok(Rc::new(Value::Boolean(*left > *right as i32 as f64)))
            }
            (Value::Char(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Boolean(*left as i32 as f64 > *right)))
            }
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot compare types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                left.get_line(),
                left.as_ref().clone(),
            ))),
        }
    }
    fn eval_op_lt(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;
        let right_val = right.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            right.get_line(),
            right,
        )?;
        match (left_val.as_ref(), right_val.as_ref()) {
            (Value::Number(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Boolean(left < right)))
            }
            (Value::Char(left), Value::Char(right)) => Ok(Rc::new(Value::Boolean(
                (*left as i32 as f64) < (*right as i32 as f64),
            ))),
            (Value::Number(left), Value::Char(right)) => {
                Ok(Rc::new(Value::Boolean(*left < *right as i32 as f64)))
            }
            (Value::Char(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Boolean((*left as i32 as f64) < *right)))
            }
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot compare types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                left.get_line(),
                left.as_ref().clone(),
            ))),
        }
    }
    fn eval_op_plseq(
        left: &Box<AST>,
        right: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?;
        match left_val.as_ref() {
            Value::Variable(name) => {
                let right_val = right.get_value(scope_chain)?.unpack_and_transform(
                    scope_chain,
                    right.get_line(),
                    right,
                )?;
                if match scope_chain.is_const(&name, left.get_line()) {
                    Ok(v) => v,
                    Err(e) => return Err(Box::new(e.to_runtime_error(left.as_ref().clone()))),
                } {
                    return Err(Box::new(RuntimeError::new(
                        format!("Cannot change const variable {}", name),
                        left.get_line(),
                        left.as_ref().clone(),
                    )));
                }
                let a = match scope_chain.get_variable(&name, left.get_line()) {
                    Ok(v) => v,
                    Err(e) => return Err(Box::new(e.to_runtime_error(left.as_ref().clone()))),
                };
                let a_ptr = Rc::<Value>::as_ptr(&a) as *mut Value;
                let b = right_val.unpack_and_transform(scope_chain, right.get_line(), right)?;
                match (a.as_ref(), b.as_ref()) {
                    (Value::Number(a), Value::Number(b)) => unsafe {
                        *a_ptr = Value::Number(*a + *b);
                    },
                    (Value::Char(a), Value::Char(b)) => unsafe {
                        *a_ptr = Value::Number(*a as i32 as f64 + *b as i32 as f64);
                    },
                    (Value::Number(a), Value::Char(b)) => unsafe {
                        *a_ptr = Value::Number(*a + *b as i32 as f64);
                    },
                    (Value::Char(a), Value::Number(b)) => unsafe {
                        *a_ptr = Value::Number(*a as i32 as f64 + *b);
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
                        return Err(Box::new(RuntimeError::new(
                            format!(
                                "Cannot add types {} and {}",
                                a.pretty_type(scope_chain, left.get_line()),
                                b.pretty_type(scope_chain, right.get_line())
                            ),
                            left.get_line(),
                            left.as_ref().clone(),
                        )));
                    }
                };
                Ok(Rc::new(Value::Variable(name.clone())))
            }
            _ => Err(Box::new(RuntimeError::new(
                "Cannot assign to a non-variable in +=".into(),
                left.get_line(),
                left.as_ref().clone(),
            ))),
        }
    }
    fn eval_if(
        if_lit: &IfLiteral,
        if_lit_ast: &AST,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let cond = if_lit.cond.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            if_lit_ast.get_line(),
            if_lit_ast,
        )?;
        match cond.as_ref() {
            Value::Boolean(true) => {
                scope_chain.add_scope().to_runtime_error(if_lit_ast)?;
                for ast in if_lit.body.iter() {
                    ast.get_value(scope_chain)?;
                    match scope_chain.get_return_register() {
                        ReturnType::None => (),
                        _ => break,
                    }
                }
                scope_chain.pop_scope().to_runtime_error(if_lit_ast)?;
                // if ignores return register, so we just let it pass through
                return Ok(Rc::new(Value::Undefined));
            }
            Value::Boolean(false) => (),
            _ => {
                return Err(Box::new(RuntimeError::new(
                    "If condition must be a boolean".into(),
                    if_lit.cond.get_line(),
                    if_lit.cond.as_ref().clone(),
                )))
            }
        }
        for (_, elseif) in if_lit.elseifs.iter().enumerate() {
            let cond = elseif.0.get_value(scope_chain)?.unpack_and_transform(
                scope_chain,
                elseif.0.get_line(),
                &elseif.0,
            )?;
            match cond.as_ref() {
                Value::Boolean(true) => {
                    scope_chain.add_scope().to_runtime_error(if_lit_ast)?;
                    for ast in elseif.1.iter() {
                        ast.get_value(scope_chain)?;
                        match scope_chain.get_return_register() {
                            ReturnType::None => (),
                            _ => break,
                        }
                    }
                    scope_chain.pop_scope().to_runtime_error(if_lit_ast)?;
                    return Ok(Rc::new(Value::Undefined));
                }
                Value::Boolean(false) => (),
                _ => {
                    return Err(Box::new(RuntimeError::new(
                        "If condition must be a boolean".into(),
                        elseif.0.get_line(),
                        elseif.0.as_ref().clone(),
                    )))
                }
            }
        }
        if let Some(else_body) = &if_lit.else_body {
            scope_chain.add_scope().to_runtime_error(if_lit_ast)?;
            for ast in else_body.iter() {
                ast.get_value(scope_chain)?;
                match scope_chain.get_return_register() {
                    ReturnType::None => (),
                    _ => break,
                }
            }
            scope_chain.pop_scope().to_runtime_error(if_lit_ast)?;
        }
        return Ok(Rc::new(Value::Undefined));
    }
    fn eval_while(
        cond: &Box<AST>,
        block: &Vec<Box<AST>>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        loop {
            let cond_val = cond.get_value(scope_chain)?.unpack_and_transform(
                scope_chain,
                cond.get_line(),
                cond,
            )?;
            match cond_val.as_ref() {
                Value::Boolean(true) => {
                    scope_chain.add_scope().to_runtime_error(cond)?;
                    for ast in block.iter() {
                        ast.get_value(scope_chain)?;
                        match scope_chain.get_return_register() {
                            ReturnType::None => (),
                            _ => break,
                        }
                    }
                    scope_chain.pop_scope().to_runtime_error(cond)?;
                    match scope_chain.get_return_register() {
                        ReturnType::None => (),
                        ReturnType::Break => {
                            scope_chain.eat_return_register();
                            return Ok(Rc::new(Value::Undefined));
                        }
                        ReturnType::Continue => {
                            scope_chain.eat_return_register();
                        }
                        ReturnType::Return(v) => return Ok(v),
                    }
                }
                Value::Boolean(false) => break,
                _ => {
                    return Err(Box::new(RuntimeError::new(
                        "While condition must be a boolean".into(),
                        cond.get_line(),
                        cond.as_ref().clone(),
                    )))
                }
            }
        }
        Ok(Rc::new(Value::Undefined))
    }
    fn eval_return(
        v: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let v_val = v
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, v.get_line(), v)?;
        scope_chain
            .set_return_register(ReturnType::Return(v_val))
            .to_runtime_error(v.as_ref())?;
        Ok(Rc::new(Value::Undefined))
    }

    pub fn get_value(&self, scope_chain: &mut ScopeChain) -> Result<Rc<Value>, Box<RuntimeError>> {
        let ret = match self {
            AST::Return(v, _) => AST::eval_return(v, scope_chain),
            AST::Break(_) => {
                scope_chain
                    .set_return_register(ReturnType::Break)
                    .to_runtime_error(self)?;
                Ok(Rc::new(Value::Undefined))
            }
            AST::Continue(_) => {
                scope_chain
                    .set_return_register(ReturnType::Continue)
                    .to_runtime_error(self)?;
                Ok(Rc::new(Value::Undefined))
            }
            AST::FunctionCall(func, params, line) => {
                let func =
                    func.get_value(scope_chain)?
                        .unpack_and_transform(scope_chain, *line, self)?;
                match func.as_ref() {
                    Value::Function(func) => func.call(scope_chain, params, self, *line),
                    Value::BuiltinFunction(f, arg_len) => {
                        if arg_len != &params.len() {
                            return Err(Box::new(RuntimeError::new(
                                format!("Expected {} arguments, got {}", arg_len, params.len()),
                                *line,
                                self.clone(),
                            )));
                        }
                        // convert params to values
                        let mut params = params
                            .iter()
                            .map(|ast| {
                                ast.get_value(scope_chain)?.unpack_and_transform(
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
                            *line,
                            self.clone(),
                        )))
                    }
                }
            }
            AST::BooleanLiteral(b, _) => Ok(Rc::new(Value::Boolean(*b))),
            AST::If(if_lit, _) => AST::eval_if(if_lit, &self, scope_chain),
            AST::While(cond, block, _) => AST::eval_while(cond, block, scope_chain),
            AST::FunctionLiteral(f, _) => Ok(Rc::new(Value::Function(f.make_real(scope_chain)))),
            AST::StringLiteral(str, _) => Ok(Rc::new(Value::String(str.to_string()))),
            AST::NumberLiteral(num, _) => Ok(Rc::new(Value::Number(*num))),
            AST::VariableDeclaration(name, is_const, _) => {
                match scope_chain
                    .add_variable(name, *is_const, self.get_line())
                    .to_runtime_error(self)
                {
                    Ok(_) => Ok(Rc::new(Value::Variable(name.clone()))),
                    Err(e) => return Err(e),
                }
            }
            AST::OpAndAnd(left, right, _) => AST::eval_op_andand(left, right, scope_chain),
            AST::OpOrOr(left, right, _) => AST::eval_op_oror(left, right, scope_chain),
            AST::OpNotEq(left, right, _) => AST::eval_op_noteq(left, right, scope_chain),
            AST::OpPls(left, right, _) => AST::eval_op_pls(left, right, scope_chain),
            AST::OpPlsEq(left, right, _) => AST::eval_op_plseq(left, right, scope_chain),
            AST::OpEq(left, right, _) => AST::eval_op_eq(left, right, scope_chain),
            AST::OpGt(left, right, _) => AST::eval_op_gt(left, right, scope_chain),
            AST::OpLt(left, right, _) => AST::eval_op_lt(left, right, scope_chain),
            AST::OpGtEq(left, right, _) => AST::eval_op_gteq(left, right, scope_chain),
            AST::OpLtEq(left, right, _) => AST::eval_op_lteq(left, right, scope_chain),
            AST::CharacterLiteral(c, _) => Ok(Rc::new(Value::Char(*c))),
            AST::OpEqEq(left, right, _) => AST::eval_op_eqeq(left, right, scope_chain),
            AST::VariableAccess(name, _) => Ok(Value::Variable(name.clone()).into()),
            AST::Paren(ast, _) => ast.get_value(scope_chain),
        };

        match ret {
            Ok(v) => Ok(v),
            Err(mut e) => {
                e.add_base_ast(self.clone());
                Err(e)
            }
        }
    }

    pub fn interpret(&self, scope_chain: &mut ScopeChain) -> Result<(), Box<RuntimeError>> {
        self.get_value(scope_chain)?;
        match scope_chain.get_return_register() {
            ReturnType::Return(_) => Err(Box::new(RuntimeError::new(
                "Return statement at top level".into(),
                self.get_line(),
                self.clone(),
            ))),
            ReturnType::Continue => Err(Box::new(RuntimeError::new(
                "Continue statement at top level".into(),
                self.get_line(),
                self.clone(),
            ))),
            ReturnType::Break => Err(Box::new(RuntimeError::new(
                "Break statement at top level".into(),
                self.get_line(),
                self.clone(),
            ))),
            ReturnType::None => Ok(()),
        }
    }

    pub fn debug_pretty_print(&self) -> String {
        match self {
            AST::Return(v, _) => format!("return {}", v.debug_pretty_print()),
            AST::Break(_) => "break".to_string(),
            AST::Continue(_) => "continue".to_string(),
            AST::FunctionCall(func, params, _) => {
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
            AST::BooleanLiteral(b, _) => b.to_string(),
            AST::While(c, block, _) => {
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
            AST::If(if_lit, _) => if_lit.pretty_print(),
            AST::FunctionLiteral(func, _) => func.pretty_print(),
            AST::StringLiteral(value, _) => format!("\"{}\"", value),
            AST::NumberLiteral(value, _) => value.to_string(),
            AST::OpAndAnd(left, right, _) => {
                format!(
                    "({} && {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpOrOr(left, right, _) => {
                format!(
                    "({} || {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpPls(left, right, _) => {
                format!(
                    "({} + {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpPlsEq(left, right, _) => {
                format!(
                    "({} += {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpGt(left, right, _) => {
                format!(
                    "({} > {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpLt(left, right, _) => {
                format!(
                    "({} < {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpGtEq(left, right, _) => {
                format!(
                    "({} >= {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpLtEq(left, right, _) => {
                format!(
                    "({} <= {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::VariableDeclaration(name, is_const, _) if !*is_const => format!("var {}", name),
            AST::VariableDeclaration(name, _, _) => format!("const {}", name),
            AST::OpEq(left, right, _) => {
                format!(
                    "({} = {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::CharacterLiteral(c, _) => format!("'{}'", c),
            AST::OpEqEq(left, right, _) => {
                format!(
                    "({} == {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpNotEq(left, right, _) => {
                format!(
                    "({} != {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::VariableAccess(name, _) => name.to_string(),
            AST::Paren(ast, _) => format!("({})", ast.debug_pretty_print()),
        }
    }

    pub fn pretty_print(&self) -> String {
        match self {
            AST::Return(v, _) => format!("return {}", v.pretty_print()),
            AST::Break(_) => "break".to_string(),
            AST::Continue(_) => "continue".to_string(),
            AST::BooleanLiteral(b, _) => b.to_string(),
            AST::FunctionCall(func, params, _) => {
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
            AST::While(c, block, _) => {
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
            AST::If(if_lit, _) => if_lit.pretty_print(),
            AST::FunctionLiteral(func, _) => func.pretty_print(),
            AST::StringLiteral(value, _) => format!("\"{}\"", value),
            AST::NumberLiteral(value, _) => value.to_string(),
            AST::OpAndAnd(left, right, _) => {
                format!("{} && {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpOrOr(left, right, _) => {
                format!("{} || {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpPls(left, right, _) => {
                format!("{} + {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpPlsEq(left, right, _) => {
                format!("{} += {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpGt(left, right, _) => {
                format!("{} > {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpLt(left, right, _) => {
                format!("{} < {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpGtEq(left, right, _) => {
                format!("{} >= {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpLtEq(left, right, _) => {
                format!("{} <= {}", left.pretty_print(), right.pretty_print())
            }
            AST::VariableDeclaration(name, is_const, _) if !*is_const => format!("var {}", name),
            AST::VariableDeclaration(name, _, _) => format!("const {}", name),
            AST::OpEq(left, right, _) => {
                format!("{} = {}", left.pretty_print(), right.pretty_print())
            }
            AST::CharacterLiteral(c, _) => format!("'{}'", c),
            AST::OpEqEq(left, right, _) => {
                format!("{} == {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpNotEq(left, right, _) => {
                format!("{} != {}", left.pretty_print(), right.pretty_print())
            }
            AST::VariableAccess(name, _) => name.to_string(),
            AST::Paren(ast, _) => format!("({})", ast.pretty_print()),
        }
    }
}
