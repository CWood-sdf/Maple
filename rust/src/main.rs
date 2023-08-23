mod lexer;
use crate::lexer::{Lexer, Token};
mod parser;
use crate::parser::Parser;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents: String = "var a = '\\\\'".to_string();

    let contents_copy = contents.clone();
    println!("Contents: {}", contents);
    let mut lexer = Lexer::new(contents);
    loop {
        let token = lexer.get_next_token()?;
        println!("{:?}", token);
        if token == Token::EOF {
            break;
        }
    }
    println!("Done");
    let mut parser = Parser::new(contents_copy);

    let _ast = parser.parse()?;
    Result::Ok(())
}
