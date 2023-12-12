use std::rc::Rc;

use crate::{
    ast::{ConvertScopeErrorResult, AST},
    error::RuntimeError,
    parser::{Unpack, Value},
    scopechain::ScopeChain,
};

fn builtin_println(
    args: Vec<Rc<Value>>,
    ast: &AST,
    scopechain: &ScopeChain,
    line: usize,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    match args[0]
        .unpack_and_transform(scopechain, line, ast)?
        .as_ref()
    {
        &Value::Object(_) => todo!(),

        &Value::String(ref s) => {
            println!("{}", s);
        }
        &Value::Number(ref n) => {
            println!("{}", n);
        }
        &Value::Boolean(ref b) => {
            println!("{}", b);
        }
        &Value::Undefined => {
            println!("undefined");
        }
        &Value::BuiltinFunction(_, len) => {
            println!("builtin_function(<{}>)", len);
        }
        &Value::Function(ref lit) => {
            println!("{}", lit.pretty_print());
        }
        &Value::Char(ref c) => {
            println!("{}", c);
        }
        &Value::Variable(_) => {
            return Err(Box::new(RuntimeError::new(
                "Cannot println variable".to_string(),
                line,
            )));
        }
        &Value::ObjectAccess(_,_) => {
            return Err(Box::new(RuntimeError::new(
                "Cannot println object access".to_string(),
                line,
            )));
        }
    };
    Ok(Rc::new(Value::Undefined))
}

fn builtin_print(
    args: Vec<Rc<Value>>,
    ast: &AST,
    scopechain: &ScopeChain,
    line: usize,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    match args[0]
        .unpack_and_transform(scopechain, line, ast)?
        .as_ref()
    {
        &Value::Object(_) => todo!(),

        &Value::String(ref s) => {
            print!("{}", s);
        }
        &Value::Number(ref n) => {
            print!("{}", n);
        }
        &Value::Boolean(ref b) => {
            print!("{}", b);
        }
        &Value::Undefined => {
            print!("undefined");
        }
        &Value::BuiltinFunction(_, len) => {
            print!("builtin_function(<{}>)", len);
        }
        &Value::Function(ref lit) => {
            print!("{}", lit.pretty_print());
        }
        &Value::Char(ref c) => {
            print!("{}", c);
        }
        &Value::ObjectAccess(_,_) => {
            return Err(Box::new(RuntimeError::new(
                "Cannot print object access".to_string(),
                line,
            )));
        }
        &Value::Variable(_) => {
            return Err(Box::new(RuntimeError::new(
                "Cannot print variable".to_string(),
                line,
            )));
        }
    };
    Ok(Rc::new(Value::Undefined))
}

fn builtin_concat(
    args: Vec<Rc<Value>>,
    ast: &AST,
    scopechain: &ScopeChain,
    line: usize,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    match (
        args[0].unpack_and_transform(scopechain, line, ast)?.as_ref(), 
        args[1].unpack_and_transform(scopechain, line, ast)?.as_ref()
    ) {
        (Value::String(ref s1), Value::String(ref s2)) => {
            Ok(Rc::new(Value::String(format!("{}{}", s1, s2))))
        }
        (Value::Char(ref c1), Value::Char(ref c2)) => {
            Ok(Rc::new(Value::String(format!("{}{}", c1, c2))))
        }
        (Value::Char(ref c1), Value::String(ref s2)) => {
            Ok(Rc::new(Value::String(format!("{}{}", c1, s2))))
        }
        (Value::String(ref s1), Value::Char(ref c2)) => {
            Ok(Rc::new(Value::String(format!("{}{}", s1, c2))))
        }
        (vl, vr) => Err(Box::new(RuntimeError::new(
            format!("Cannot concat values that are not a character or string, note: given types are {} and {}", vl.pretty_type(scopechain, line), vr.pretty_type(scopechain, line)),
            line,
        ))),
    }
}

pub fn builtin_nanos(
    _args: Vec<Rc<Value>>,
    _ast: &AST,
    _scopechain: &ScopeChain,
    _line: usize,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    use std::time::SystemTime;
    let time = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    Ok(Rc::new(Value::Number(time as f64)))
}
pub fn builtin_to_str(
    args: Vec<Rc<Value>>,
    ast: &AST,
    scopechain: &ScopeChain,
    line: usize,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    match 
        args[0].unpack_and_transform(scopechain, line, ast)?.as_ref() 
     {
        Value::String(ref s) => {
            Ok(Rc::new(Value::String(s.clone())))
        }
        Value::Char(ref c) => {
            Ok(Rc::new(Value::String(c.to_string())))
        }
        Value::Number(ref n) => {
            Ok(Rc::new(Value::String(n.to_string())))
        }
        Value::Boolean(ref b) => {
            Ok(Rc::new(Value::String(b.to_string())))
        }
        Value::Undefined => {
            Ok(Rc::new(Value::String("undefined".to_string())))
        }
        Value::BuiltinFunction(_, len) => {
            Ok(Rc::new(Value::String(format!("builtin_function(<{}>)", len))))
        }
        Value::Function(ref lit) => {
            Ok(Rc::new(Value::String(lit.pretty_print())))
        }
        Value::Object(_) => {
            Ok(Rc::new(Value::String("object".to_string())))
        }
        Value::ObjectAccess(_,_) => {
            return Err(Box::new(RuntimeError::new(
                "Cannot toStr object access".to_string(),
                line,
            )));
        }
        Value::Variable(_) => {
            return Err(Box::new(RuntimeError::new(
                "Cannot toStr variable".to_string(),
                line,
            )));
        }

    }

}
pub fn create_builtins(scope_chain: &mut ScopeChain) -> Result<(), Box<RuntimeError>> {
    let println_name = "println".to_string();
    scope_chain
        .add_variable(&println_name, true, 0)
        .to_runtime_error()?;
    scope_chain
        .set_variable(
            &println_name,
            Rc::new(Value::BuiltinFunction(builtin_println, 1)),
            0,
        )
        .to_runtime_error()?;
    let print_name = "print".to_string();
    scope_chain
        .add_variable(&print_name, true, 0)
        .to_runtime_error()?;
    scope_chain
        .set_variable(
            &print_name,
            Rc::new(Value::BuiltinFunction(builtin_print, 1)),
            0,
        )
        .to_runtime_error()?;
    let concat_name = "concat".to_string();
    scope_chain
        .add_variable(&concat_name, true, 0)
        .to_runtime_error()?;
    scope_chain
        .set_variable(
            &concat_name,
            Rc::new(Value::BuiltinFunction(builtin_concat, 2)),
            0,
        )
        .to_runtime_error()?;
    let nanos_name = "nanos".to_string();
    scope_chain
        .add_variable(&nanos_name, true, 0)
        .to_runtime_error()?;
    scope_chain
        .set_variable(
            &nanos_name,
            Rc::new(Value::BuiltinFunction(builtin_nanos, 0)),
            0,
        )
        .to_runtime_error()?;

    let to_str_name = "toStr".to_string();
    scope_chain
        .add_variable(&to_str_name, true, 0)
        .to_runtime_error()?;
    scope_chain
        .set_variable(
            &to_str_name,
            Rc::new(Value::BuiltinFunction(builtin_to_str, 1)),
            0,
        )
        .to_runtime_error()?;
    Ok(())
}
