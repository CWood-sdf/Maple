#![allow(dead_code)]
use crate::error::{RuntimeError, ScopeError};
use crate::lexer::Token;
use crate::parser::{Object, ObjectKey, Unpack, Value};
use crate::scopechain::ReturnType;
use crate::ScopeChain;

use std::fs;
use std::rc::Rc;

pub trait ConvertScopeErrorResult<T> {
    fn to_runtime_error(&self) -> Result<&T, Box<RuntimeError>>;
}

impl<T> ConvertScopeErrorResult<T> for Result<T, ScopeError>
where
    T: Clone,
{
    fn to_runtime_error(&self) -> Result<&T, Box<RuntimeError>> {
        match self {
            Ok(v) => Ok(v),
            Err(e) => Err(Box::new(e.to_runtime_error())),
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
                .add_variable(param, false, line)
                .to_runtime_error()?;
            let param_value = params_value[i].clone()?;
            scope_chain
                .set_variable(param, param_value, line)
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
pub enum ASTType {
    Import(String),
    DotAccess(Box<AST>, String),
    BracketAccess(Box<AST>, Box<AST>),
    ObjectLiteral(Vec<(ObjectKey, Box<AST>)>),
    CharacterLiteral(char),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    Paren(Box<AST>),
    VariableDeclaration(String, bool),
    FunctionLiteral(FunctionLiteral),
    FunctionCall(Box<AST>, Vec<Box<AST>>),
    If(IfLiteral),
    While(Box<AST>, Vec<Box<AST>>),
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
            ))),
        }
    }
    fn eval_op_mns_prefix(
        left: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;
        match left_val.as_ref() {
            Value::Number(left) => Ok(Rc::new(Value::Number(-left))),
            Value::Char(left) => Ok(Rc::new(Value::Number(-(*left as i32 as f64)))),
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot negate type {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                ),
                left.get_line(),
            ))),
        }
    }
    fn eval_op_not(
        left: &Box<AST>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<RuntimeError>> {
        let left_val = left.get_value(scope_chain)?.unpack_and_transform(
            scope_chain,
            left.get_line(),
            left,
        )?;
        match left_val.as_ref() {
            Value::Number(left) => Ok(Rc::new(Value::Boolean(*left == 0.0))),
            Value::Char(left) => Ok(Rc::new(Value::Boolean(*left == '\0'))),
            Value::Boolean(left) => Ok(Rc::new(Value::Boolean(!*left))),
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot negate type {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                ),
                left.get_line(),
            ))),
        }
    }
    fn eval_op_mns(
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
            (Value::Number(left), Value::Number(right)) => Ok(Rc::new(Value::Number(left - right))),
            (Value::Char(left), Value::Char(right)) => Ok(Rc::new(Value::Number(
                *left as i32 as f64 - *right as i32 as f64,
            ))),
            (Value::Number(left), Value::Char(right)) => {
                Ok(Rc::new(Value::Number(*left - *right as i32 as f64)))
            }
            (Value::Char(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Number(*left as i32 as f64 - *right)))
            }
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot subtract types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                left.get_line(),
            ))),
        }
    }
    fn eval_op_times(
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
            (Value::Number(left), Value::Number(right)) => Ok(Rc::new(Value::Number(left * right))),
            (Value::Char(left), Value::Char(right)) => Ok(Rc::new(Value::Number(
                *left as i32 as f64 * *right as i32 as f64,
            ))),
            (Value::Number(left), Value::Char(right)) => {
                Ok(Rc::new(Value::Number(*left * *right as i32 as f64)))
            }
            (Value::Char(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Number(*left as i32 as f64 * *right)))
            }
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot multiply types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                left.get_line(),
            ))),
        }
    }
    fn eval_op_div(
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
            (Value::Number(left), Value::Number(right)) => Ok(Rc::new(Value::Number(left / right))),
            (Value::Char(left), Value::Char(right)) => Ok(Rc::new(Value::Number(
                *left as i32 as f64 / *right as i32 as f64,
            ))),
            (Value::Number(left), Value::Char(right)) => {
                Ok(Rc::new(Value::Number(*left / *right as i32 as f64)))
            }
            (Value::Char(left), Value::Number(right)) => {
                Ok(Rc::new(Value::Number(*left as i32 as f64 / *right)))
            }
            _ => Err(Box::new(RuntimeError::new(
                format!(
                    "Cannot divide types {} and {}",
                    left_val.pretty_type(scope_chain, left.get_line()),
                    right_val.pretty_type(scope_chain, right.get_line())
                ),
                left.get_line(),
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
                    Err(e) => return Err(Box::new(e.to_runtime_error())),
                };
                Ok(left_val)
            }
            Value::ObjectAccess(orgobj, key) => {
                let obj = match orgobj.as_ref() {
                    Value::Object(obj) => obj as *const Object as *mut Object,
                    _ => {
                        return Err(Box::new(RuntimeError::new(
                            format!("Cannot access field of non-object",),
                            left.get_line(),
                        )))
                    }
                };
                unsafe {
                    obj.as_mut()
                        .expect("how we get a null pointer in 3 lines")
                        .set(key.clone(), right_val);
                }
                Ok(orgobj.clone())
            }
            _ => Err(Box::new(RuntimeError::new(
                "Cannot assign to a non-variable".into(),
                left.get_line(),
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
            Value::ObjectAccess(obj, key) => {
                let right_val = right.get_value(scope_chain)?.unpack_and_transform(
                    scope_chain,
                    right.get_line(),
                    right,
                )?;
                let o = match obj.as_ref() {
                    Value::Object(obj) => obj,
                    _ => {
                        return Err(Box::new(RuntimeError::new(
                            format!("Cannot access field of non-object",),
                            left.get_line(),
                        )))
                    }
                };
                let a = (*o).get(key.clone(), left.get_line())?;
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
                        )));
                    }
                };
                Ok(left_val)
            }
            Value::Variable(name) => {
                let right_val = right.get_value(scope_chain)?.unpack_and_transform(
                    scope_chain,
                    right.get_line(),
                    right,
                )?;
                if match scope_chain.is_const(&name, left.get_line()) {
                    Ok(v) => v,
                    Err(e) => return Err(Box::new(e.to_runtime_error())),
                } {
                    return Err(Box::new(RuntimeError::new(
                        format!("Cannot change const variable {}", name),
                        left.get_line(),
                    )));
                }
                let a = match scope_chain.get_variable(&name, left.get_line()) {
                    Ok(v) => v,
                    Err(e) => return Err(Box::new(e.to_runtime_error())),
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
                        )));
                    }
                };
                Ok(Rc::new(Value::Variable(name.clone())))
            }
            _ => Err(Box::new(RuntimeError::new(
                "Cannot assign to a non-variable in +=".into(),
                left.get_line(),
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
                scope_chain.add_scope().to_runtime_error()?;
                for ast in if_lit.body.iter() {
                    ast.get_value(scope_chain)?;
                    match scope_chain.get_return_register() {
                        ReturnType::None => (),
                        _ => break,
                    }
                }
                scope_chain.pop_scope().to_runtime_error()?;
                // if ignores return register, so we just let it pass through
                return Ok(Rc::new(Value::Undefined));
            }
            Value::Boolean(false) => (),
            _ => {
                return Err(Box::new(
                    RuntimeError::new(
                        "If condition must be a boolean".into(),
                        if_lit.cond.get_line(),
                    )
                    .add_base_ast(if_lit.cond.as_ref().clone()),
                ))
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
                    scope_chain.add_scope().to_runtime_error()?;
                    for ast in elseif.1.iter() {
                        ast.get_value(scope_chain)?;
                        match scope_chain.get_return_register() {
                            ReturnType::None => (),
                            _ => break,
                        }
                    }
                    scope_chain.pop_scope().to_runtime_error()?;
                    return Ok(Rc::new(Value::Undefined));
                }
                Value::Boolean(false) => (),
                _ => {
                    return Err(Box::new(
                        RuntimeError::new(
                            "If condition must be a boolean".into(),
                            elseif.0.get_line(),
                        )
                        .add_base_ast(elseif.0.as_ref().clone()),
                    ))
                }
            }
        }
        if let Some(else_body) = &if_lit.else_body {
            scope_chain.add_scope().to_runtime_error()?;
            for ast in else_body.iter() {
                ast.get_value(scope_chain)?;
                match scope_chain.get_return_register() {
                    ReturnType::None => (),
                    _ => break,
                }
            }
            scope_chain.pop_scope().to_runtime_error()?;
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
                    scope_chain.add_scope().to_runtime_error()?;
                    for ast in block.iter() {
                        ast.get_value(scope_chain)?;
                        match scope_chain.get_return_register() {
                            ReturnType::None => (),
                            _ => break,
                        }
                    }
                    scope_chain.pop_scope().to_runtime_error()?;
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
            .to_runtime_error()?;
        Ok(Rc::new(Value::Undefined))
    }
    fn eval_import(filename: String) -> Result<Rc<Value>, Box<RuntimeError>> {
        let contents = match fs::read_to_string(filename.clone()) {
            Ok(v) => v,
            Err(e) => {
                return Err(Box::new(RuntimeError::new(
                    format!("Cannot read file {}: {}", filename, e),
                    0,
                )))
            }
        };
        let mut parser = crate::parser::Parser::new(contents);
        let mut scope_chain: ScopeChain = ScopeChain::new();

        match crate::builtins::create_builtins(&mut scope_chain) {
            Ok(_) => {}
            Err(e) => {
                return Err(Box::new(RuntimeError::new(
                    format!("Error creating builtins: {}", e),
                    0,
                )));
            }
        };
        let ast = match parser.parse(true) {
            Ok(ast) => ast,
            Err(e) => {
                return Err(Box::new(RuntimeError::new(
                    format!("Error parsing file {}: {}", filename, e),
                    0,
                )));
            }
        };

        // for (_, stmt) in ast.iter().enumerate() {
        //     println!("{}", stmt.pretty_print());
        // }
        //
        let mut ret = Rc::new(Value::Undefined);
        for (_, stmt) in ast.iter().enumerate() {
            match stmt.interpret(&mut scope_chain) {
                Ok(ReturnType::None) => {}
                Ok(ReturnType::Return(v)) => {
                    ret = v;
                    break;
                }
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    return Err(e);
                }
            };
        }
        return Ok(ret);
    }

    pub fn get_value(&self, scope_chain: &mut ScopeChain) -> Result<Rc<Value>, Box<RuntimeError>> {
        let line = self.get_line();
        let ret = match &self.t {
            ASTType::Import(s) => AST::eval_import(s.clone()),
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
            ASTType::Return(v) => AST::eval_return(v, scope_chain),
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
            ASTType::If(if_lit) => AST::eval_if(if_lit, &self, scope_chain),
            ASTType::While(cond, block) => AST::eval_while(cond, block, scope_chain),
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
            ASTType::OpAndAnd(left, right) => AST::eval_op_andand(left, right, scope_chain),
            ASTType::OpOrOr(left, right) => AST::eval_op_oror(left, right, scope_chain),
            ASTType::OpNotEq(left, right) => AST::eval_op_noteq(left, right, scope_chain),
            ASTType::OpNot(right) => AST::eval_op_not(right, scope_chain),
            ASTType::OpPls(left, right) => AST::eval_op_pls(left, right, scope_chain),
            ASTType::OpMns(left, right) => AST::eval_op_mns(left, right, scope_chain),
            ASTType::OpTimes(left, right) => AST::eval_op_times(left, right, scope_chain),
            ASTType::OpDiv(left, right) => AST::eval_op_div(left, right, scope_chain),
            ASTType::OpMnsPrefix(left) => AST::eval_op_mns_prefix(left, scope_chain),
            ASTType::OpPlsEq(left, right) => AST::eval_op_plseq(left, right, scope_chain),
            ASTType::OpEq(left, right) => AST::eval_op_eq(left, right, scope_chain),
            ASTType::OpGt(left, right) => AST::eval_op_gt(left, right, scope_chain),
            ASTType::OpLt(left, right) => AST::eval_op_lt(left, right, scope_chain),
            ASTType::OpGtEq(left, right) => AST::eval_op_gteq(left, right, scope_chain),
            ASTType::OpLtEq(left, right) => AST::eval_op_lteq(left, right, scope_chain),
            ASTType::CharacterLiteral(c) => Ok(Rc::new(Value::Char(*c))),
            ASTType::OpEqEq(left, right) => AST::eval_op_eqeq(left, right, scope_chain),
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
