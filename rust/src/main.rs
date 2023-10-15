mod lexer;
// use crate::lexer::{Lexer, Token};
mod ast;
mod error;
mod parser;

mod scopechain;
use std::{error::Error, rc::Rc};

use ast::{ConvertScopeErrorResult, AST};
use error::RuntimeError;
use parser::{Unpack, Value};
use scopechain::ScopeChain;

use crate::{error::MapleError, parser::Parser};

fn create_builtins(scope_chain: &mut ScopeChain) -> Result<(), Box<RuntimeError>> {
    let ast: AST = AST::Break(0);
    let println_name = "println".to_string();
    scope_chain
        .add_variable(&println_name, true, 0)
        .to_runtime_error(&ast)?;
    scope_chain
        .set_variable(
            &println_name,
            Rc::new(Value::BuiltinFunction(
                |args, ast, scopechain, line| {
                    match args[0]
                        .unpack_and_transform(scopechain, line, ast)?
                        .as_ref()
                    {
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
                                ast.clone(),
                            )));
                        }
                    };
                    Ok(Rc::new(Value::Undefined))
                },
                1,
            )),
            0,
        )
        .to_runtime_error(&ast)?;
    let print_name = "print".to_string();
    scope_chain
        .add_variable(&print_name, true, 0)
        .to_runtime_error(&ast)?;
    scope_chain
        .set_variable(
            &print_name,
            Rc::new(Value::BuiltinFunction(
                |args, ast, scopechain, line| {
                    match args[0]
                        .unpack_and_transform(scopechain, line, ast)?
                        .as_ref()
                    {
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
                        &Value::Variable(_) => {
                            return Err(Box::new(RuntimeError::new(
                                "Cannot print variable".to_string(),
                                line,
                                ast.clone(),
                            )));
                        }
                    };
                    Ok(Rc::new(Value::Undefined))
                },
                1,
            )),
            0,
        )
        .to_runtime_error(&ast)?;
    let concat_name = "concat".to_string();
    let concat_fn = |args: Vec<Rc<Value>>, ast: &AST, scopechain: &ScopeChain, line| {
        match (args[0].unpack_and_transform(scopechain, line, ast)?.as_ref(), args[1].unpack_and_transform(scopechain, line, ast)?.as_ref()) {
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
                        ast.clone(),
                    ))),
                }
    };
    scope_chain
        .add_variable(&concat_name, true, 0)
        .to_runtime_error(&ast)?;
    scope_chain
        .set_variable(
            &concat_name,
            Rc::new(Value::BuiltinFunction(concat_fn, 2)),
            0,
        )
        .to_runtime_error(&ast)?;
    Ok(())
}
fn time_interpreter(contents: String, demo: bool) -> Result<f64, Box<dyn Error>> {
    let mut parser = Parser::new(contents);
    let mut scope_chain: ScopeChain = ScopeChain::new();

    match create_builtins(&mut scope_chain) {
        Ok(_) => {}
        Err(e) => {
            println!("Error: {}", e.get_msg());
            return Result::Err(e.get_msg().into());
        }
    };
    let timer = std::time::Instant::now();
    let ast = match parser.parse(true) {
        Ok(ast) => ast,
        Err(e) => {
            println!("Error: {}", e.get_msg());
            return Result::Err(e.get_msg().into());
        }
    };

    // for (_, stmt) in ast.iter().enumerate() {
    //     println!("{}", stmt.pretty_print());
    // }
    for (_, stmt) in ast.iter().enumerate() {
        match stmt.interpret(&mut scope_chain) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e.get_msg());
                return Result::Err(e.get_msg().into());
            }
        };
    }
    // println!("Time: {}us", timer.elapsed().as_micros());
    if demo {
        // let a_name = "a".to_string();
        // let a = scope_chain.get_variable(&a_name)?;
        // println!("a: {:?}", a);
        let b_name = "b".to_string();
        let b = scope_chain.get_variable(&b_name, 0)?;
        println!("b: {:?}", b);
        // let c_name = "c".to_string();
        // let c = scope_chain.get_variable(&c_name)?;
        // println!("c: {:?}", c);
    }
    Ok(timer.elapsed().as_nanos() as f64 / 1000.0)
}

trait Square<T> {
    fn square(&self) -> T;
}
impl Square<f64> for f64 {
    fn square(&self) -> f64 {
        self * self
    }
}

trait ToShortDigits<T> {
    fn to_short_digits(&self, decimals: u32) -> T;
}

impl ToShortDigits<f64> for f64 {
    fn to_short_digits(&self, decimals: u32) -> f64 {
        return (self * 10i32.pow(decimals) as f64).round() / 10i32.pow(decimals) as f64;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = std::fs::read_to_string("./maple.mpl")?;

    let mut times: Vec<f64> = Vec::new();
    let mut amount = 0;
    let timer = std::time::Instant::now();
    times.reserve(1000000);
    let start = std::time::Instant::now();
    while amount < 1 {
        let time = match time_interpreter(contents.clone(), amount == 0) {
            Ok(time) => time,
            Err(_) => {
                return Ok(());
            }
        };
        times.push(time);
        amount += 1;

        if timer.elapsed().as_secs_f64() * 1000.0 >= 500.0 {
            break;
        }
    }
    println!("Time: {}ms", start.elapsed().as_millis(),);
    let mut total: f64 = 0.0;
    // let times_copy = times.clone();
    let mut min = times[0];
    let mut max = times[0];
    let mut i = 0;
    let top80 = (amount as f64 * 0.8) as usize;
    times.sort_by(|a, b| a.partial_cmp(b).unwrap());
    for time in &times {
        let time = *time;
        if i < top80 {
            total += time as f64;
        }
        if time < min {
            min = time;
        }
        if time > max {
            max = time;
        }
        i += 1;
    }
    total /= top80 as f64;
    let mut std_dev: f64 = 0.0;
    for time in &times {
        let time = *time;
        std_dev += (time as f64 - total).square();
    }
    std_dev = (std_dev / amount as f64).sqrt();

    println!(
        "Mean of top 80%: {}us, 󰘫: {}, min: {}, max: {}",
        total.to_short_digits(3),
        std_dev.to_short_digits(3),
        min,
        max
    );
    // std::fs::write("./times.csv", file_out)?;

    println!("Total time: {}ms", timer.elapsed().as_millis());

    Result::Ok(())
}
