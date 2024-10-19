use regex::Regex;

fn main() -> Result<(), String> {
    let text: &str = " 32  -1.992 \n";
    let text_invalid: &str = " 32  -1.992  3. ";
    let result: Result<Vec<String>, &str> = lexer(text);
    match result {
        Ok(tokens) => {
            for token in tokens {
                println!("{}", token)
            }
        }
        Err(e) => {
            return Err(format!("In lexer: {}", e.to_string()));
        }
    }

    Ok(())
}

fn lexer(s_: &str) -> Result<Vec<String>, &str> {
    let mut s: String = s_.to_string();
    let re_head_num: Regex = Regex::new(r"^-?[0-9]+(.[0-9]+)?").unwrap();
    let mut tokens: Vec<String> = Vec::new();
    loop {
        s = s.trim().to_string();
        if s.len() == 0 {
            break;
        }
        if let Some(token) = re_head_num.find(&s) {
            tokens.push(token.as_str().to_string());
            s = s[token.end()..].to_string();
        } else {
            return Err("Unexpected pattern.");
        }
    }
    return Ok(tokens);
}
