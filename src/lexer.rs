use crate::data_type::Token;

use regex::Regex;

pub fn lexer(s_: &str) -> Result<Vec<Token>, String> {
    let mut s: String = s_.to_string();
    let mut tokens: Vec<Token> = Vec::new();
    let token_dict: Vec<(&str, &str)> = vec![
        ("NUMBER", r"-?[0-9]+(.[0-9]+)?"),
        ("PLUS", r"\+"),
        ("MINUS", r"-"),
        ("MUL", r"\*"),
        ("DIV", r"/"),
        ("LPAREN", r"\("),
        ("RPAREN", r"\)"),
        ("UNKNOWN", r"."), // for文の条件式のなかに入れたい
    ];
    loop {
        s = s.trim().to_string();
        if s.len() == 0 {
            break;
        }

        for (token, pattern) in token_dict.iter() {
            if let Some(match_item) = Regex::new(&["^", *pattern].concat()).unwrap().find(&s) {
                tokens.push(Token {
                    token_type: token.to_string(),
                    token_value: match_item.as_str().to_string(),
                });
                s = s[match_item.end()..].to_string();
                break;
            }
        }
        if tokens[tokens.len() - 1].token_type == "UNKNOWN" {
            return Err(format!(
                "Invalid token \"{}\"",
                tokens[tokens.len() - 1].token_value
            ));
        }
    }
    return Ok(tokens);
}
