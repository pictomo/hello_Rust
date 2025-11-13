use crate::token::Token;
use crate::token::TokenType;

pub fn parser(mut tokens: Vec<Token>) -> Result<(), String> {
    expr(&mut tokens)?;
    if !tokens.is_empty() {
        return Err(format!("Invalid Syntax \"{}\"", tokens[0].token_value));
    }
    Ok(())
}

fn checktoken(token_type: &str) {}

fn expr(tokens: &mut Vec<Token>) -> Result<(), String> {
    term(tokens)?;
    expr_(tokens)?;
    Ok(())
}

fn expr_(tokens: &mut Vec<Token>) -> Result<(), String> {
    if tokens.is_empty() {
        return Ok(());
    }

    match tokens[0].token_type {
        TokenType::PLUS => {
            tokens.remove(0);
        }
        TokenType::MINUS => {
            tokens.remove(0);
        }
        _ => return Ok(()),
    }

    term(tokens)?;
    expr_(tokens)?;

    Ok(())
}

fn term(tokens: &mut Vec<Token>) -> Result<(), String> {
    num(tokens)?;
    term_(tokens)?;
    Ok(())
}

fn term_(tokens: &mut Vec<Token>) -> Result<(), String> {
    if tokens.is_empty() {
        return Ok(());
    }

    match tokens[0].token_type {
        TokenType::MUL => {
            tokens.remove(0);
        }
        TokenType::DIV => {
            tokens.remove(0);
        }
        _ => {
            return Ok(());
        }
    }

    num(tokens)?;
    term_(tokens)?;
    Ok(())
}

fn num(tokens: &mut Vec<Token>) -> Result<(), String> {
    match tokens[0].token_type {
        TokenType::NUMBER => {
            tokens.remove(0);
        }
        TokenType::LPAREN => {
            tokens.remove(0);
            expr(tokens)?;
            match tokens[0].token_type {
                TokenType::RPAREN => {
                    tokens.remove(0);
                }
                _ => {
                    return Err(format!("Invalid Syntax \"{}\"", tokens[0].token_value));
                }
            }
        }
        _ => {
            return Err(format!("Invalid Syntax \"{}\"", tokens[0].token_value));
        }
    }
    Ok(())
}
