mod lexer;
// use crate::lexer::{Lexer, Token};
mod ast;
mod parser;

mod scopechain;
use std::error::Error;

use scopechain::ScopeChain;

use crate::parser::Parser;
fn time_interpreter(contents: String, demo: bool) -> Result<u128, Box<dyn Error>> {
    let timer = std::time::Instant::now();
    let mut parser = Parser::new(contents);
    let mut scope_chain: ScopeChain = ScopeChain::new();

    let ast = match parser.parse(true) {
        Ok(ast) => ast,
        Err(e) => {
            println!("Error: {}", e);
            return Result::Err(e);
        }
    };
    // for (_, stmt) in ast.iter().enumerate() {
    //     println!("{}", stmt.pretty_print());
    // }
    for (_, stmt) in ast.iter().enumerate() {
        stmt.get_value(&mut scope_chain)?;
    }
    // println!("Time: {}us", timer.elapsed().as_micros());
    if demo {
        // let a_name = "a".to_string();
        // let a = scope_chain.get_variable(&a_name)?;
        // println!("a: {:?}", a);
        let b_name = "b".to_string();
        let b = scope_chain.get_variable(&b_name)?;
        println!("b: {:?}", b);
        // let c_name = "c".to_string();
        // let c = scope_chain.get_variable(&c_name)?;
        // println!("c: {:?}", c);
    }
    Ok(timer.elapsed().as_micros())
}

trait Square<T> {
    fn square(&self) -> T;
}
impl Square<f64> for f64 {
    fn square(&self) -> f64 {
        self * self
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = std::fs::read_to_string("./maple.mpl")?;
    let mut file_out = "".to_string();

    let mut times: Vec<u128> = Vec::new();
    let amount = 50000;
    for i in 0..amount {
        times.push(time_interpreter(contents.clone(), i == amount - 1)?);
    }
    let mut total: f64 = 0.0;
    let times_copy = times.clone();
    let mut min = times[0];
    let mut max = times[0];
    let mut i = 0;
    let top80 = (amount as f64 * 0.8) as usize;
    times.sort();
    for time in times {
        if i < top80 {
            total += time as f64;
        }
        if time < min {
            min = time;
        }
        if time > max {
            max = time;
        }
        file_out = format!("{}\n{}", file_out, time);
        i += 1;
    }
    total /= top80 as f64;
    let mut std_dev: f64 = 0.0;
    for time in times_copy {
        std_dev += (time as f64 - total).square();
    }
    std_dev = (std_dev / amount as f64).sqrt();

    println!(
        "Mean of top 80%: {}us, 󰘫: {}, min: {}, max: {}",
        total, std_dev, min, max
    );
    std::fs::write("./times.csv", file_out)?;

    Result::Ok(())
}
