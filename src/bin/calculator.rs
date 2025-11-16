use calculator_rust::lexer::lexer;
use calculator_rust::parser::parser;
use calculator_rust::token::Token;
use std::io;

fn main() {
    let mut buffer = String::new();
    io::stdin()
        .read_line(&mut buffer)
        .expect("Failed to read line.");

    let result: Result<Vec<Token>, String> = lexer(&buffer);
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
