mod lexer;
// use crate::lexer::{Lexer, Token};
mod ast;
mod builtins;
mod error;
mod parser;

mod scopechain;
use std::error::Error;

use builtins::create_builtins;
use scopechain::{ReturnType, ScopeChain};

use crate::{error::MapleError, parser::Parser};

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
    let args = std::env::args().collect::<Vec<String>>();
    let filename;
    let file_arg = "--file".to_string();
    if args.contains(&file_arg) {
        let index = args.iter().position(|x| x == &file_arg).unwrap();
        if index + 1 < args.len() {
            filename = args[index + 1].clone();
        } else {
            println!("No file specified");
            return Ok(());
        }
    } else {
        filename = "./maple.mpl".to_string();
    }
    let time_arg = "--time".to_string();
    if args.contains(&time_arg) {
        let contents: String = std::fs::read_to_string(filename)?;

        let mut times: Vec<f64> = Vec::new();
        let mut amount = 0;
        let timer = std::time::Instant::now();
        times.reserve(1000000);
        let start = std::time::Instant::now();
        while amount < 1000000 {
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
    } else {
        let contents: String = std::fs::read_to_string(filename)?;
        let mut parser = Parser::new(contents);
        let mut scope_chain: ScopeChain = ScopeChain::new();

        match create_builtins(&mut scope_chain) {
            Ok(_) => {}
            Err(e) => {
                println!("Error: {}", e.get_msg());
                return Result::Err(e.get_msg().into());
            }
        };
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
                Ok(ReturnType::None) => {}
                Ok(_) => {
                    break;
                }
                Err(e) => {
                    println!("Error: {}", e.get_msg());
                    break;
                    // return Result::Err(e.get_msg().into());
                }
            };
        }
    }

    Result::Ok(())
}
