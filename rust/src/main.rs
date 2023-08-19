mod lexer;
use crate::lexer::{Lexer, Token};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let contents: String = "var a = 1".to_string();

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
    Result::Ok(())
}
