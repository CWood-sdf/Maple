#![allow(dead_code)]
use crate::parser::Value;
use crate::ScopeChain;
use std::error::Error;

use std::rc::Rc;

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
