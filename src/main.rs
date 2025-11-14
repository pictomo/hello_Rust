mod lexer;
mod parser;
mod token;

use lexer::lexer;
use parser::parser;
use token::Token;

fn main() {
    let mut text: &str = " (32 + -1.992) + 56 * 09 / 30.0 - 12\n";

    let mut text: &str = "(3*4-6/3)/2\n"; // 5

    // let mut text: &str = " 32  -1.992  3. "; // invalid token test

    // let mut text: &str = " (32 + -1.992) + 56 * 09 / 30.0 - + -12 "; // invalid syntax test

    let result: Result<Vec<Token>, String> = lexer(text);
    match result {
        Ok(tokens) => {
            // for token in tokens.iter() {
            //     println!("{:?} {}", token.token_type, token.token_value)
            // }
            match parser(tokens) {
                Ok(val) => {
                    println!("{}", val);
                }
                Err(e) => {
                    eprintln!("Error: In parser: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Err(e) => {
            eprintln!("Error: In lexer: {}", e);
            std::process::exit(1);
        }
    }
}
