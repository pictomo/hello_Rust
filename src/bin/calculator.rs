// cargo run --bin calculator

use calculator_rust::lexer::lexer;
use calculator_rust::parser::parser;
use calculator_rust::token::Token;
use std::io;

fn main() {
    println!(
        "Type expression and press Enter (Ctrl+C to exit)\n\
    e.g. 3 + (0.50 - 1) * 4 / -6\n\
    "
    );
    loop {
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line.");

        let result: Result<Vec<Token>, String> = lexer(&buffer);
        match result {
            Ok(tokens) => match parser(tokens) {
                Ok(val) => {
                    println!("= {}", val);
                }
                Err(e) => {
                    eprintln!("Ops! {}", e);
                }
            },
            Err(e) => {
                eprintln!("Ops! {}", e);
            }
        }
        println!();
    }
}
