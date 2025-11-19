use crate::token::*;

pub fn parser(mut tokens: Vec<Token>) -> Result<f64, String> {
    let val = expr(&mut tokens)?;
    if !tokens.is_empty() {
        return Err(format!("Invalid Syntax \"{}\"", tokens[0].token_value));
    }
    Ok(val)
}

fn expr(tokens: &mut Vec<Token>) -> Result<f64, String> {
    let left = term(tokens)?;

    let val = expr_(tokens, left)?;

    Ok(val)
}

fn expr_(tokens: &mut Vec<Token>, left_val: f64) -> Result<f64, String> {
    if tokens.is_empty() {
        return Ok(left_val);
    }
    let op: TokenType;

    match tokens[0].token_type {
        TokenType::PLUS => {
            tokens.remove(0);
            op = TokenType::PLUS;
        }
        TokenType::MINUS => {
            tokens.remove(0);
            op = TokenType::MINUS;
        }
        _ => return Ok(left_val),
    }

    let right_val = term(tokens)?;
    let val: f64;
    match op {
        TokenType::PLUS => {
            val = expr_(tokens, left_val + right_val)?;
        }
        TokenType::MINUS => {
            val = expr_(tokens, left_val - right_val)?;
        }
        _ => {
            return Err("Unexpected Error".to_string());
        }
    }

    Ok(val)
}

fn term(tokens: &mut Vec<Token>) -> Result<f64, String> {
    let left = num(tokens)?;

    let val = term_(tokens, left)?;

    Ok(val)
}

fn term_(tokens: &mut Vec<Token>, left_val: f64) -> Result<f64, String> {
    if tokens.is_empty() {
        return Ok(left_val);
    }
    let op: TokenType;

    match tokens[0].token_type {
        TokenType::MUL => {
            tokens.remove(0);
            op = TokenType::MUL;
        }
        TokenType::DIV => {
            tokens.remove(0);
            op = TokenType::DIV;
        }
        _ => return Ok(left_val),
    }

    let right_val = num(tokens)?;
    let val: f64;
    match op {
        TokenType::MUL => {
            val = term_(tokens, left_val * right_val)?;
        }
        TokenType::DIV => {
            val = term_(tokens, left_val / right_val)?;
        }
        _ => {
            return Err("Unexpected Error".to_string());
        }
    }

    Ok(val)
}

fn num(tokens: &mut Vec<Token>) -> Result<f64, String> {
    let val: f64;

    match tokens[0].token_type {
        TokenType::MINUS => {
            tokens.remove(0);
            match tokens[0].token_type {
                TokenType::NUMBER => {
                    val = -tokens.remove(0).token_value.parse::<f64>().unwrap();
                }
                _ => {
                    return Err(format!("Invalid Syntax \"{}\"", tokens[0].token_value));
                }
            }
        }
        TokenType::NUMBER => {
            val = tokens.remove(0).token_value.parse().unwrap();
        }
        TokenType::LPAREN => {
            tokens.remove(0);
            val = expr(tokens)?;
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
    Ok(val)
}
