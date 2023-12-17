#![allow(dead_code)]
use crate::error::ScopeError;
use crate::parser::Value;
use crate::parser::Variable;

use std::error::Error;
use std::rc::Rc;

#[derive(Debug, Clone, PartialEq)]
pub struct Scope {
    // this must never be an Rc because it slows down prog by ~20%
    variables: Vec<Variable>,
    is_fn: bool,
    is_global: bool,
}
impl Scope {
    fn new(is_fn: bool) -> Scope {
        Scope {
            variables: vec![],
            is_fn,
            is_global: false,
        }
    }
    fn global() -> Scope {
        Scope {
            variables: vec![],
            is_fn: false,
            is_global: true,
        }
    }
    fn set_variable(&mut self, name: &String, value: Rc<Value>) -> Result<bool, Box<dyn Error>> {
        for var in self.variables.iter_mut() {
            if var.name == *name {
                if var.is_const && *var.value != Value::Undefined {
                    return Ok(false);
                }

                // if the variable has been defined, the copy the given value into the variable
                // this preserves the reference keeping behavior of += and other mutate operators
                if *var.value != Value::Undefined {
                    let val_ptr = Rc::<Value>::as_ptr(&var.value) as *mut Value;
                    unsafe {
                        *val_ptr = value.as_ref().clone();
                    }
                } else {
                    var.value = value.clone();
                }
                return Ok(true);
            }
        }
        Err(format!("Variable {} not found", name).into())
    }
    fn is_const(&self, name: &String) -> Result<bool, Box<dyn Error>> {
        for var in self.variables.iter() {
            if var.name == *name {
                return Ok(var.is_const);
            }
        }
        Err(format!("Variable {} not found", name).into())
    }
    fn get_variable(&self, name: &String) -> Result<Rc<Value>, Box<dyn Error>> {
        for var in self.variables.iter() {
            if var.name == *name {
                return Ok(var.value.clone());
            }
        }
        Err(format!("Variable {} not found", name).into())
    }
    fn add_variable(
        &mut self,
        name: &String,
        is_const: bool,
        line: usize,
    ) -> Result<(), ScopeError> {
        for var in self.variables.iter() {
            if var.name == *name {
                return Err(ScopeError::new(
                    format!("Variable {} already exists", name),
                    line,
                ));
            }
        }
        self.variables.push(Variable {
            name: name.clone(),
            value: Rc::new(Value::Undefined),
            is_const,
        });
        Ok(())
    }
    fn has_variable(&self, name: &String) -> bool {
        for var in self.variables.iter() {
            if var.name == *name {
                return true;
            }
        }
        false
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum ReturnType {
    Continue,
    Break,
    Return(Rc<Value>),
    None,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ScopeChain {
    scopes: Vec<Scope>,
    return_register: ReturnType,
}
impl ScopeChain {
    pub fn new() -> ScopeChain {
        // initialize with a global scope
        ScopeChain {
            scopes: vec![Scope::global()],
            return_register: ReturnType::None,
        }
    }
    pub fn get_closure(&self) -> ScopeChain {
        let mut scopes = vec![];
        for scope in self.scopes.iter().rev() {
            if scope.is_fn || scope.is_global {
                break;
            }
            scopes.push(scope.clone());
        }
        scopes.reverse();
        ScopeChain {
            scopes,
            return_register: ReturnType::None,
        }
    }
    pub fn add_fn_scope(&mut self, closure: &ScopeChain) {
        let mut i = 0;
        for scope in closure.scopes.iter() {
            self.scopes.push(scope.clone());
            if i == 0 {
                self.scopes.last_mut().unwrap().is_fn = true;
            }
            i += 1;
        }

        if closure.scopes.len() == 0 {
            self.scopes.push(Scope::new(true));
        } else {
            self.scopes.push(Scope::new(false));
        }
    }
    pub fn add_scope(&mut self) -> Result<(), ScopeError> {
        self.scopes.push(Scope::new(false));
        Ok(())
    }
    pub fn pop_scope(&mut self) -> Result<(), ScopeError> {
        self.scopes.pop();
        Ok(())
    }
    pub fn pop_fn_scope(&mut self, line: usize) -> Result<(), ScopeError> {
        if self.scopes.len() == 1 {
            return Err(ScopeError::new("Cannot pop global scope".into(), line));
        }
        while !self.scopes.last().unwrap().is_fn {
            self.scopes.pop();
        }
        self.scopes.pop();
        Ok(())
    }
    pub fn set_return_register(&mut self, value: ReturnType) -> Result<(), ScopeError> {
        if self.return_register != ReturnType::None {
            return Err(ScopeError::new(
                format!("Return register already set to {:?}", self.return_register).into(),
                0,
            ));
        }
        self.return_register = value;
        Ok(())
    }
    pub fn get_return_register(&self) -> ReturnType {
        self.return_register.clone()
    }
    pub fn eat_return_register(&mut self) -> ReturnType {
        let ret = self.return_register.clone();
        self.return_register = ReturnType::None;
        ret
    }
    pub fn get_variable(&self, name: &String, line: usize) -> Result<Rc<Value>, ScopeError> {
        let mut i = self.scopes.len() - 1;
        loop {
            let scope = &self.scopes[i];
            match scope.get_variable(&name) {
                Ok(value) => return Ok(value),
                Err(_) => (),
            }
            if scope.is_fn && i != 0 {
                i = 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        Err(ScopeError::new(
            format!("Variable {} not found", name).into(),
            line,
        ))
    }
    pub fn is_const(&self, name: &String, line: usize) -> Result<bool, ScopeError> {
        let mut i = self.scopes.len() - 1;
        loop {
            let scope = &self.scopes[i];
            match scope.is_const(&name) {
                Ok(value) => return Ok(value),
                Err(_) => (),
            }
            if scope.is_fn && i != 0 {
                i = 1;
            }
            if i == 0 {
                break;
            }
            i -= 1;
        }
        Err(ScopeError::new(
            format!("Variable {} not found", name).into(),
            line,
        ))
    }
    pub fn add_variable(
        &mut self,
        name: &String,
        is_const: bool,
        line: usize,
    ) -> Result<(), ScopeError> {
        match self.scopes.last_mut() {
            Some(scope) => scope.add_variable(&name, is_const, line)?,
            None => return Err(ScopeError::new("No scope to add variable to".into(), 0)),
        }
        Ok(())
    }
    pub fn set_variable(
        &mut self,
        name: &String,
        value: Rc<Value>,
        line: usize,
    ) -> Result<(), ScopeError> {
        let mut i: isize = self.scopes.len() as isize - 1;
        while i >= 0 {
            let scope = &mut self.scopes[i as usize];
            let can_set = scope.set_variable(&name, value.clone());
            if scope.is_fn && i != 0 {
                i = 1;
            }
            i -= 1;
            if can_set.is_err() {
                continue;
            }
            if !can_set.unwrap() {
                return Err(ScopeError::new(
                    format!("Cannot change const variable {}", name).into(),
                    line,
                ));
            } else {
                return Ok(());
            }
        }
        Err(ScopeError::new(
            format!("Variable {} not found", name).into(),
            line,
        ))
    }
}
