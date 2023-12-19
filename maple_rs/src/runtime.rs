use crate::ast::Block;
use crate::ast::ConvertScopeErrorResult;
use crate::ast::IfLiteral;
use crate::parser::Object;
use crate::parser::Unpack;
use crate::{error::RuntimeError, scopechain::ReturnType};
use std::fs;
use std::rc::Rc;

use crate::{ast::AST, parser::Value, scopechain::ScopeChain};

pub fn eval_op_andand(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;

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
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
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
pub fn eval_op_oror(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;

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
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
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
pub fn eval_op_mns_prefix(
    left: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
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
pub fn eval_op_not(
    left: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
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
pub fn eval_op_mns(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
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
pub fn eval_op_times(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
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
pub fn eval_op_div(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
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
pub fn eval_op_pls(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
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
pub fn eval_op_noteq(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
    Ok(Rc::new(Value::Boolean(left_val != right_val)))
}
pub fn eval_op_eqeq(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
    Ok(Rc::new(Value::Boolean(left_val == right_val)))
}
pub fn eval_op_eq(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val = left.get_value(scope_chain)?;

    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
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
                    .set_ref(key, right_val);
            }
            Ok(orgobj.clone())
        }
        _ => Err(Box::new(RuntimeError::new(
            "Cannot assign to a non-variable".into(),
            left.get_line(),
        ))),
    }
}
pub fn eval_op_gteq(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
    match (left_val.as_ref(), right_val.as_ref()) {
        (Value::Number(left), Value::Number(right)) => Ok(Rc::new(Value::Boolean(left >= right))),
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
pub fn eval_op_lteq(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
    match (left_val.as_ref(), right_val.as_ref()) {
        (Value::Number(left), Value::Number(right)) => Ok(Rc::new(Value::Boolean(left <= right))),
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
pub fn eval_op_gt(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
    match (left_val.as_ref(), right_val.as_ref()) {
        (Value::Number(left), Value::Number(right)) => Ok(Rc::new(Value::Boolean(left > right))),
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
pub fn eval_op_lt(
    left: &Box<AST>,
    right: &Box<AST>,
    scope_chain: &mut ScopeChain,
) -> Result<Rc<Value>, Box<RuntimeError>> {
    let left_val =
        left.get_value(scope_chain)?
            .unpack_and_transform(scope_chain, left.get_line(), left)?;
    let right_val =
        right
            .get_value(scope_chain)?
            .unpack_and_transform(scope_chain, right.get_line(), right)?;
    match (left_val.as_ref(), right_val.as_ref()) {
        (Value::Number(left), Value::Number(right)) => Ok(Rc::new(Value::Boolean(left < right))),
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
pub fn eval_op_plseq(
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
pub fn eval_if(
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
                    RuntimeError::new("If condition must be a boolean".into(), elseif.0.get_line())
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
pub fn eval_while(
    cond: &Box<AST>,
    block: &Block,
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
pub fn eval_return(
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
pub fn eval_import(filename: String) -> Result<Rc<Value>, Box<RuntimeError>> {
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
