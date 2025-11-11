mod lexer;
mod token;

use lexer::lexer;
use token::Token;

fn main() -> Result<(), String> {
    // let text: &str = " 32  -1.992 +\n";
    let text: &str = " 32  -1.992  3. ";
    let result: Result<Vec<Token>, String> = lexer(text);
    match result {
        Ok(tokens) => {
            for token in tokens {
                println!("{} {}", token.token_type, token.token_value)
            }
        }
        Err(e) => {
            return Err(format!("In lexer: {}", e));
        }
    }

    Ok(())
}
