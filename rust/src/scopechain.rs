#![allow(dead_code)]
use crate::parser::Value;
use crate::parser::Variable;

use std::rc::Rc;

#[derive(Debug)]
pub struct Scope {
    variables: Vec<Variable>,
}
impl Scope {
    fn new() -> Scope {
        Scope { variables: vec![] }
    }
    fn set_variable(
        &mut self,
        name: &String,
        value: Rc<Value>,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        for var in self.variables.iter_mut() {
            if var.name == *name {
                if var.is_const && *var.value != Value::Undefined {
                    return Ok(false);
                }

                var.value = value.clone();
                return Ok(true);
            }
        }
        Err(format!("Variable {} not found", name).into())
    }
    fn get_variable(&self, name: &String) -> Result<Rc<Value>, Box<dyn std::error::Error>> {
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
    ) -> Result<(), Box<dyn std::error::Error>> {
        for var in self.variables.iter() {
            if var.name == *name {
                return Err(format!("Variable {} already exists", name).into());
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
#[derive(Debug)]
pub struct ScopeChain {
    scopes: Vec<Scope>,
}
impl ScopeChain {
    pub fn new() -> ScopeChain {
        // initialize with a global scope
        ScopeChain {
            scopes: vec![Scope::new()],
        }
    }
    pub fn add_scope(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.scopes.push(Scope::new());
        Ok(())
    }
    pub fn pop_scope(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.scopes.pop();
        Ok(())
    }
    pub fn get_variable(&self, name: &String) -> Result<Rc<Value>, Box<dyn std::error::Error>> {
        for scope in self.scopes.iter().rev() {
            match scope.get_variable(&name) {
                Ok(value) => return Ok(value),
                Err(_) => (),
            }
        }
        Err(format!("Variable {} not found", name).into())
    }
    pub fn add_variable(
        &mut self,
        name: &String,
        is_const: bool,
    ) -> Result<(), Box<dyn std::error::Error>> {
        self.scopes
            .last_mut()
            .unwrap()
            .add_variable(&name, is_const)
    }
    pub fn set_variable(
        &mut self,
        name: &String,
        value: Rc<Value>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for scope in self.scopes.iter_mut().rev() {
            let can_set = scope.set_variable(&name, value.clone())?;
            if !can_set {
                return Err(format!("Cannot set const variable {}", name).into());
            } else {
                return Ok(());
            }
        }
        Err(format!("Variable {} not found", name).into())
    }
}
