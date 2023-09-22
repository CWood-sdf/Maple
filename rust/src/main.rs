mod lexer;
// use crate::lexer::{Lexer, Token};
mod parser;

mod scopechain;
use scopechain::ScopeChain;

use crate::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let y: Rc<i32> = Rc::new(5);
    // unsafe {
    //     let ptr: *mut i32 = Rc::<i32>::as_ptr(&y) as *mut i32;
    //     *ptr = 6;
    // }
    // println!("y: {}", y);
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
    //     // println!("{}", stmt.pretty_print());
    // }
    for (_, stmt) in ast.iter().enumerate() {
        stmt.get_value(&mut scope_chain)?;
    }
    let a_name = "b".to_string();
    println!("Time: {}us", timer.elapsed().as_micros());
    let a = scope_chain.get_variable(&a_name)?;
    println!("b: {:?}", a);

    Result::Ok(())
}
