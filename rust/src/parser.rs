use crate::lexer::{Lexer, Token};
pub enum AST {
    StringLiteral(Token),
}

pub struct Parser {
    lexer: Lexer,
}
impl Parser {
    pub fn new(contents: String) -> Parser {
        let lexer = Lexer::new(contents);
        Parser { lexer }
    }
    pub fn parse(&mut self) -> Result<AST, Box<dyn std::error::Error>> {
        let _token = self.lexer.get_next_token()?;
        Ok(AST::StringLiteral(Token::String("".to_string())))
    }
}
#[cfg(test)]
mod test_parser {}
