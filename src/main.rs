use self::lexer::{Lexer, TokenType};

mod lexer;

fn main() {
    let mut lexer = Lexer::new("_av4");

    loop {
        let token = lexer.next();

        println!("{:?} '{}'", token.token_type, token.contents);

        if matches!(token.token_type, TokenType::EndOfFile) {
            break;
        }
    }
}
