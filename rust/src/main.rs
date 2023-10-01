mod lexer;
// use crate::lexer::{Lexer, Token};
mod ast;
mod parser;

mod scopechain;
use std::error::Error;

use scopechain::ScopeChain;

use crate::parser::Parser;

fn main() -> Result<(), Box<dyn Error>> {
    let contents: String = std::fs::read_to_string("./maple.mpl")?;

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
    println!("Time: {}us", timer.elapsed().as_micros());
    let a_name = "a".to_string();
    let a = scope_chain.get_variable(&a_name)?;
    println!("a: {:?}", a);
    let b_name = "b".to_string();
    let b = scope_chain.get_variable(&b_name)?;
    println!("b: {:?}", b);
    let c_name = "c".to_string();
    let c = scope_chain.get_variable(&c_name)?;
    println!("c: {:?}", c);

    Result::Ok(())
}
