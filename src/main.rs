use self::lexer::{Lexer, TokenType};

mod lexer;
mod parser;
mod syntax_tree;

fn main() {
    let mut lexer = Lexer::new("local example = 1000");

    loop {
        let token = lexer.next();

        println!("{:?} '{}'", token.token_type, token.contents);

        if matches!(token.token_type, TokenType::EndOfFile) {
            break;
        }
    }
}
