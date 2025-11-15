use crate::token::*;

pub fn parser(mut tokens: Vec<Token>) -> Result<f64, String> {
    let val = expr(&mut tokens)?;
    if !tokens.is_empty() {
        return Err(format!("Invalid Syntax \"{}\"", tokens[0].token_value));
    }
    Ok(val)
}

struct RightReturn {
    op: Option<TokenType>,
    right: Option<f64>,
}

fn expr(tokens: &mut Vec<Token>) -> Result<f64, String> {
    let val: f64;

    let left = term(tokens)?;

    let right_return = expr_(tokens)?;

    match right_return {
        Some(rr) => {
            let op = rr.op.unwrap();
            let right = rr.right.unwrap();
            match op {
                TokenType::PLUS => {
                    val = left + right;
                }
                TokenType::MINUS => {
                    val = left - right;
                }
                _ => {
                    return Err("Unexpected Error".to_string());
                }
            }
        }
        None => {
            val = left;
        }
    }
    Ok(val)
}

fn expr_(tokens: &mut Vec<Token>) -> Result<Option<RightReturn>, String> {
    if tokens.is_empty() {
        return Ok(None);
    }
    let op: TokenType;
    let right: f64;

    match tokens[0].token_type {
        TokenType::PLUS => {
            tokens.remove(0);
            op = TokenType::PLUS;
        }
        TokenType::MINUS => {
            tokens.remove(0);
            op = TokenType::MINUS;
        }
        _ => return Ok(None),
    }

    let pre_right = term(tokens)?;
    let right_right_return = expr_(tokens)?;
    match right_right_return {
        Some(rr) => {
            let next_op = rr.op.unwrap();
            let next_right = rr.right.unwrap();
            match next_op {
                TokenType::PLUS => {
                    right = pre_right + next_right;
                }
                TokenType::MINUS => {
                    right = pre_right - next_right;
                }
                _ => {
                    return Err("Unexpected Error".to_string());
                }
            }
        }
        None => {
            right = pre_right;
        }
    }

    Ok(Some(RightReturn {
        op: Some(op),
        right: Some(right),
    }))
}

fn term(tokens: &mut Vec<Token>) -> Result<f64, String> {
    let val: f64;

    let left = num(tokens)?;

    let right_return = term_(tokens)?;

    match right_return {
        Some(rr) => {
            let op = rr.op.unwrap();
            let right = rr.right.unwrap();
            match op {
                TokenType::MUL => {
                    val = left * right;
                }
                TokenType::DIV => {
                    val = left / right;
                }
                _ => {
                    return Err("Unexpected Error".to_string());
                }
            }
        }
        None => {
            val = left;
        }
    }
    Ok(val)
}

fn term_(tokens: &mut Vec<Token>) -> Result<Option<RightReturn>, String> {
    if tokens.is_empty() {
        return Ok(None);
    }
    let op: TokenType;
    let right: f64;

    match tokens[0].token_type {
        TokenType::MUL => {
            tokens.remove(0);
            op = TokenType::MUL;
        }
        TokenType::DIV => {
            tokens.remove(0);
            op = TokenType::DIV;
        }
        _ => return Ok(None),
    }

    let pre_right = num(tokens)?;
    let right_right_return = term_(tokens)?;
    match right_right_return {
        Some(rr) => {
            let next_op = rr.op.unwrap();
            let next_right = rr.right.unwrap();
            match next_op {
                TokenType::MUL => {
                    right = pre_right * next_right;
                }
                TokenType::DIV => {
                    right = pre_right / next_right;
                }
                _ => {
                    return Err("Unexpected Error".to_string());
                }
            }
        }
        None => {
            right = pre_right;
        }
    }

    Ok(Some(RightReturn {
        op: Some(op),
        right: Some(right),
    }))
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
