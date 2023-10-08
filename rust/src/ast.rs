#![allow(dead_code)]
use crate::parser::{Unpack, Value};
use crate::ScopeChain;
use std::error::Error;

use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct BuiltinFunction {
    pub param_count: usize,
    pub func: fn(Vec<Rc<Value>>) -> Result<Rc<Value>, Box<dyn Error>>,
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
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        scope_chain.add_fn_scope(&self.closure)?;
        if params.len() != self.params.len() {
            return Err(format!(
                "Expected {} parameters, got {}",
                self.params.len(),
                params.len()
            )
            .into());
        }
        for (i, param) in self.params.iter().enumerate() {
            scope_chain.add_variable(param, false)?;
            let param_value = params[i].get_value(scope_chain)?;
            let param_unpacked = param_value.unpack(scope_chain)?;
            scope_chain.set_variable(param, param_unpacked)?;
        }
        for ast in self.body.iter() {
            ast.get_value(scope_chain)?;
        }
        scope_chain.pop_fn_scope()?;
        Ok(Rc::new(Value::Function(self.clone())))
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
    ) -> Result<IfLiteral, Box<dyn Error>> {
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
#[derive(Debug, Clone, PartialEq)]
pub enum AST {
    CharacterLiteral(char),
    StringLiteral(String),
    NumberLiteral(f64),
    BooleanLiteral(bool),
    // really only here for pretty printing
    Paren(Box<AST>),
    VariableDeclaration(String, bool),
    FunctionLiteral(FunctionLiteral),
    FunctionCall(Box<AST>, Vec<Box<AST>>),
    If(IfLiteral),
    While(Box<AST>, Vec<Box<AST>>),
    OpPls(Box<AST>, Box<AST>),
    OpEq(Box<AST>, Box<AST>),
    OpEqEq(Box<AST>, Box<AST>),
    OpPlsEq(Box<AST>, Box<AST>),
    OpNotEq(Box<AST>, Box<AST>),
    OpAndAnd(Box<AST>, Box<AST>),
    OpOrOr(Box<AST>, Box<AST>),
    OpGt(Box<AST>, Box<AST>),
    OpLt(Box<AST>, Box<AST>),
    OpGtEq(Box<AST>, Box<AST>),
    OpLtEq(Box<AST>, Box<AST>),
    VariableAccess(String),
}

impl AST {
    fn eval_op_andand(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn std::error::Error>> {
        let left = left.unpack(scope_chain)?;
        match left.as_ref() {
            Value::Boolean(false) => return Ok(Rc::new(Value::Boolean(false))),
            Value::Boolean(true) => {}
            _ => {
                return Err(format!(
                    "Cannot && types {} and {}",
                    left.pretty_type(scope_chain),
                    right.pretty_type(scope_chain)
                )
                .into())
            }
        };
        let right = right.unpack(scope_chain)?;
        match right.as_ref() {
            Value::Boolean(false) => Ok(Rc::new(Value::Boolean(false))),
            Value::Boolean(true) => Ok(Rc::new(Value::Boolean(true))),
            _ => Err(format!(
                "Cannot && types {} and {}",
                left.pretty_type(scope_chain),
                right.pretty_type(scope_chain)
            )
            .into()),
        }
    }
    fn eval_op_oror(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn std::error::Error>> {
        let left = left.unpack(scope_chain)?;
        match left.as_ref() {
            Value::Boolean(true) => return Ok(Rc::new(Value::Boolean(true))),
            Value::Boolean(false) => {}
            _ => {
                return Err(format!(
                    "Cannot || types {} and {}",
                    left.pretty_type(scope_chain),
                    right.pretty_type(scope_chain)
                )
                .into())
            }
        };
        let right = right.unpack(scope_chain)?;
        match right.as_ref() {
            Value::Boolean(false) => Ok(Rc::new(Value::Boolean(false))),
            Value::Boolean(true) => Ok(Rc::new(Value::Boolean(true))),
            _ => Err(format!(
                "Cannot || types {} and {}",
                left.pretty_type(scope_chain),
                right.pretty_type(scope_chain)
            )
            .into()),
        }
    }
    fn eval_op_pls(
        right: Rc<Value>,
        left: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn std::error::Error>> {
        let left = left.unpack(scope_chain)?;
        let right = right.unpack(scope_chain)?;
        match (left.as_ref(), right.as_ref()) {
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
            _ => Err(format!(
                "Cannot add types {} and {}",
                left.pretty_type(scope_chain),
                right.pretty_type(scope_chain)
            )
            .into()),
        }
    }
    fn eval_op_noteq(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        let left = left.unpack(scope_chain)?;
        let right = right.unpack(scope_chain)?;
        Ok(Rc::new(Value::Boolean(left != right)))
    }
    fn eval_op_eqeq(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        let left = left.unpack(scope_chain)?;
        let right = right.unpack(scope_chain)?;
        Ok(Rc::new(Value::Boolean(left == right)))
    }
    fn eval_op_eq(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        match left.as_ref() {
            Value::Variable(name) => {
                scope_chain.set_variable(&name, right.unpack(scope_chain)?)?;
                Ok(left)
            }
            _ => Err("Cannot assign to a non-variable".into()),
        }
    }
    fn eval_op_gteq(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        let left = left.unpack(scope_chain)?;
        let right = right.unpack(scope_chain)?;
        match (left.as_ref(), right.as_ref()) {
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
            _ => Err(format!(
                "Cannot compare types {} and {}",
                left.pretty_type(scope_chain),
                right.pretty_type(scope_chain)
            )
            .into()),
        }
    }
    fn eval_op_lteq(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        let left = left.unpack(scope_chain)?;
        let right = right.unpack(scope_chain)?;
        match (left.as_ref(), right.as_ref()) {
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
            _ => Err(format!(
                "Cannot compare types {} and {}",
                left.pretty_type(scope_chain),
                right.pretty_type(scope_chain)
            )
            .into()),
        }
    }
    fn eval_op_gt(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        let left = left.unpack(scope_chain)?;
        let right = right.unpack(scope_chain)?;
        match (left.as_ref(), right.as_ref()) {
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
            _ => Err(format!(
                "Cannot compare types {} and {}",
                left.pretty_type(scope_chain),
                right.pretty_type(scope_chain)
            )
            .into()),
        }
    }
    fn eval_op_lt(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        let left = left.unpack(scope_chain)?;
        let right = right.unpack(scope_chain)?;
        match (left.as_ref(), right.as_ref()) {
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
            _ => Err(format!(
                "Cannot compare types {} and {}",
                left.pretty_type(scope_chain),
                right.pretty_type(scope_chain)
            )
            .into()),
        }
    }
    fn eval_op_plseq(
        left: Rc<Value>,
        right: Rc<Value>,
        scope_chain: &ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        match left.as_ref() {
            Value::Variable(name) => {
                if scope_chain.is_const(&name)? {
                    return Err(format!("Cannot change const variable {}", name).into());
                }
                let a = scope_chain.get_variable(&name)?;
                let a_ptr = Rc::<Value>::as_ptr(&a) as *mut Value;
                let b = right.unpack(scope_chain)?;
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
    fn eval_if(
        if_lit: &IfLiteral,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        let cond = if_lit.cond.get_value(scope_chain)?.unpack(scope_chain)?;
        match cond.as_ref() {
            Value::Boolean(true) => {
                scope_chain.add_scope()?;
                for ast in if_lit.body.iter() {
                    ast.get_value(scope_chain)?;
                }
                scope_chain.pop_scope()?;
                return Ok(Rc::new(Value::Undefined));
            }
            Value::Boolean(false) => (),
            _ => return Err("If condition must be a boolean".into()),
        }
        for (_, elseif) in if_lit.elseifs.iter().enumerate() {
            let cond = elseif.0.get_value(scope_chain)?.unpack(scope_chain)?;
            match cond.as_ref() {
                Value::Boolean(true) => {
                    scope_chain.add_scope()?;
                    for ast in elseif.1.iter() {
                        ast.get_value(scope_chain)?;
                    }
                    scope_chain.pop_scope()?;
                    return Ok(Rc::new(Value::Undefined));
                }
                Value::Boolean(false) => (),
                _ => return Err("If condition must be a boolean".into()),
            }
        }
        if let Some(else_body) = &if_lit.else_body {
            scope_chain.add_scope()?;
            for ast in else_body.iter() {
                ast.get_value(scope_chain)?;
            }
            scope_chain.pop_scope()?;
        }
        return Ok(Rc::new(Value::Undefined));
    }
    fn eval_while(
        cond: &Box<AST>,
        block: &Vec<Box<AST>>,
        scope_chain: &mut ScopeChain,
    ) -> Result<Rc<Value>, Box<dyn Error>> {
        loop {
            let cond = cond.get_value(scope_chain)?.unpack(scope_chain)?;
            match cond.as_ref() {
                Value::Boolean(true) => {
                    scope_chain.add_scope()?;
                    for ast in block.iter() {
                        ast.get_value(scope_chain)?;
                    }
                    scope_chain.pop_scope()?;
                }
                Value::Boolean(false) => break,
                _ => return Err("While condition must be a boolean".into()),
            }
        }
        Ok(Rc::new(Value::Undefined))
    }
    pub fn get_value(&self, scope_chain: &mut ScopeChain) -> Result<Rc<Value>, Box<dyn Error>> {
        Ok(match self {
            AST::FunctionCall(func, params) => {
                let func = func.get_value(scope_chain)?.unpack(scope_chain)?;
                match func.as_ref() {
                    Value::Function(func) => func.call(scope_chain, params)?,
                    _ => return Err("Cannot call a non-function".into()),
                }
            }
            AST::BooleanLiteral(b) => Rc::new(Value::Boolean(*b)),
            AST::If(if_lit) => AST::eval_if(if_lit, scope_chain)?,
            AST::While(cond, block) => AST::eval_while(cond, block, scope_chain)?,
            AST::FunctionLiteral(f) => Rc::new(Value::Function(f.make_real(scope_chain))),
            AST::StringLiteral(str) => Rc::new(Value::String(str.to_string())),
            AST::NumberLiteral(num) => Rc::new(Value::Number(*num)),
            AST::VariableDeclaration(name, is_const) => {
                scope_chain.add_variable(name, *is_const)?;
                Rc::new(Value::Variable(name.clone()))
            }
            AST::OpAndAnd(left, right) => AST::eval_op_andand(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpOrOr(left, right) => AST::eval_op_oror(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpNotEq(left, right) => AST::eval_op_noteq(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpPls(left, right) => AST::eval_op_pls(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpPlsEq(left, right) => AST::eval_op_plseq(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpEq(left, right) => AST::eval_op_eq(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpGt(left, right) => AST::eval_op_gt(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpLt(left, right) => AST::eval_op_lt(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpGtEq(left, right) => AST::eval_op_gteq(
                left.get_value(scope_chain)?,
                right.get_value(scope_chain)?,
                scope_chain,
            )?,
            AST::OpLtEq(left, right) => AST::eval_op_lteq(
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
            AST::FunctionCall(func, params) => {
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
            AST::BooleanLiteral(b) => b.to_string(),
            AST::While(c, block) => {
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
            AST::If(if_lit) => if_lit.pretty_print(),
            AST::FunctionLiteral(func) => func.pretty_print(),
            AST::StringLiteral(value) => format!("\"{}\"", value),
            AST::NumberLiteral(value) => value.to_string(),
            AST::OpAndAnd(left, right) => {
                format!(
                    "({} && {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpOrOr(left, right) => {
                format!(
                    "({} || {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
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
            AST::OpGt(left, right) => {
                format!(
                    "({} > {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpLt(left, right) => {
                format!(
                    "({} < {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpGtEq(left, right) => {
                format!(
                    "({} >= {})",
                    left.debug_pretty_print(),
                    right.debug_pretty_print()
                )
            }
            AST::OpLtEq(left, right) => {
                format!(
                    "({} <= {})",
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
            AST::OpNotEq(left, right) => {
                format!(
                    "({} != {})",
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
            AST::BooleanLiteral(b) => b.to_string(),
            AST::FunctionCall(func, params) => {
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
            AST::While(c, block) => {
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
            AST::If(if_lit) => if_lit.pretty_print(),
            AST::FunctionLiteral(func) => func.pretty_print(),
            AST::StringLiteral(value) => format!("\"{}\"", value),
            AST::NumberLiteral(value) => value.to_string(),
            AST::OpAndAnd(left, right) => {
                format!("{} && {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpOrOr(left, right) => {
                format!("{} || {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpPls(left, right) => {
                format!("{} + {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpPlsEq(left, right) => {
                format!("{} += {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpGt(left, right) => {
                format!("{} > {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpLt(left, right) => {
                format!("{} < {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpGtEq(left, right) => {
                format!("{} >= {}", left.pretty_print(), right.pretty_print())
            }
            AST::OpLtEq(left, right) => {
                format!("{} <= {}", left.pretty_print(), right.pretty_print())
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
            AST::OpNotEq(left, right) => {
                format!("{} != {}", left.pretty_print(), right.pretty_print())
            }
            AST::VariableAccess(name) => name.to_string(),
            AST::Paren(ast) => format!("({})", ast.pretty_print()),
        }
    }
}
