use std::rc::Rc;

use crate::{
    ast::{ConvertScopeErrorResult, AST},
    error::RuntimeError,
    parser::{Unpack, Value, Object, ObjectKey},
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

pub fn builtin_arr_len(
    args: Vec<Rc<Value>>,
    ast: &AST,
    scopechain: &ScopeChain,
    line: usize,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    match args[0]
        .unpack_and_transform(scopechain, line, ast)?
        .as_ref()
    {
        Value::Object(ref obj) => {
            let len = obj.fields.iter().filter(|v| match v.0 {ObjectKey::Number(_) => true, ObjectKey::String(_) => false}).map(|_| 1).collect::<Vec<i32>>().len();
            Ok(Rc::new(Value::Number(len as f64)))
        }
        _ => {
            return Err(Box::new(RuntimeError::new(
                "Cannot get length of non-object".to_string(),
                line,
            )));
        }
    }
}

pub fn builtin_str_len(
    args: Vec<Rc<Value>>,
    ast: &AST,
    scopechain: &ScopeChain,
    line: usize,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    match args[0]
        .unpack_and_transform(scopechain, line, ast)?
        .as_ref()
    {
        Value::String(ref s) => {
            Ok(Rc::new(Value::Number(s.len() as f64)))
        }
        _ => {
            return Err(Box::new(RuntimeError::new(
                "Cannot get length of non-string".to_string(),
                line,
            )));
        }
    }
}

pub fn make_builtin_std(scope_chain: &mut ScopeChain) -> Result<(), Box<RuntimeError>> {
    let mut std_obj = Object { fields: vec![] };
    let mut std_io_obj = Object { fields: vec![] };
    std_io_obj.set(
        ObjectKey::String("println".to_string()),
        Rc::new(Value::BuiltinFunction(builtin_println, 1)),
    );
    std_io_obj.set(
        ObjectKey::String("print".to_string()),
        Rc::new(Value::BuiltinFunction(builtin_print, 1)),
    );
    let std_io_rc = Rc::new(Value::Object(std_io_obj));
    std_obj.set(
        ObjectKey::String("io".to_string()),
        std_io_rc.clone(),
    );
    let mut std_time_obj = Object { fields: vec![] };
    std_time_obj.set(
        ObjectKey::String("nanos".to_string()),
        Rc::new(Value::BuiltinFunction(builtin_nanos, 0)),
    );
    let std_time_rc = Rc::new(Value::Object(std_time_obj));
    std_obj.set(
        ObjectKey::String("time".to_string()),
        std_time_rc.clone(),
    );

    let mut std_arr_obj = Object { fields: vec![] };
    std_arr_obj.set(
        ObjectKey::String("len".to_string()),
        Rc::new(Value::BuiltinFunction(builtin_arr_len, 1)),
    );
    let std_arr_rc = Rc::new(Value::Object(std_arr_obj));
    std_obj.set(
        ObjectKey::String("arr".to_string()),
        std_arr_rc.clone(),
    );

    let mut std_str_obj = Object { fields: vec![] };
    std_str_obj.set(
        ObjectKey::String("len".to_string()),
        Rc::new(Value::BuiltinFunction(builtin_str_len, 1)),
    );
    let std_str_rc = Rc::new(Value::Object(std_str_obj));
    std_obj.set(
        ObjectKey::String("str".to_string()),
        std_str_rc.clone(),
    );
    
    let std_rc = Rc::new(Value::Object(std_obj));
    let std_name = "std".to_string();
    scope_chain
        .add_variable(&std_name, true, 0)
        .to_runtime_error()?;
    scope_chain
        .set_variable(&std_name, std_rc, 0)
        .to_runtime_error()?;
    Ok(())
}
pub fn add_builtin_fn(scope_chain: &mut ScopeChain, name: &str, len: usize, func: fn(Vec<Rc<Value>>, &AST, &ScopeChain, usize) -> Result<Rc<Value>, Box<RuntimeError>>) -> Result<(), Box<RuntimeError>> {
    let name = name.to_string();
    scope_chain
        .add_variable(&name, true, 0)
        .to_runtime_error()?;
    scope_chain
        .set_variable(
            &name,
            Rc::new(Value::BuiltinFunction(func, len)),
            0,
        )
        .to_runtime_error()?;
    Ok(())
}
pub fn create_builtins(scope_chain: &mut ScopeChain) -> Result<(), Box<RuntimeError>> {
    add_builtin_fn(scope_chain, "concat", 2, builtin_concat)?;
    add_builtin_fn(scope_chain, "toStr", 1, builtin_to_str)?;
    make_builtin_std(scope_chain)?;
    Ok(())
}
