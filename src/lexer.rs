use crate::token::*;
use regex::Regex;
use std::sync::LazyLock;

// 先頭マッチ用の正規表現を作成
fn compile_pattern(pattern: &str) -> Regex {
    Regex::new(&format!("^{}", pattern)).unwrap()
}

static TOKEN_PATTERNS: LazyLock<Vec<(TokenType, Regex)>> = LazyLock::new(|| {
    vec![
        (TokenType::NUMBER, compile_pattern(r"([0-9]+)(\.[0-9]+)?")),
        (TokenType::PLUS, compile_pattern(r"\+")),
        (TokenType::MINUS, compile_pattern(r"-")),
        (TokenType::MUL, compile_pattern(r"\*")),
        (TokenType::DIV, compile_pattern(r"/")),
        (TokenType::LPAREN, compile_pattern(r"\(")),
        (TokenType::RPAREN, compile_pattern(r"\)")),
    ]
});

pub fn lexer(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut remaining = input.trim();

    while !remaining.is_empty() {
        remaining = remaining.trim_start();

        let mut matched = false;
        for (token_type, pattern) in TOKEN_PATTERNS.iter() {
            if let Some(mat) = pattern.find(remaining) {
                tokens.push(Token {
                    token_type: *token_type,
                    token_value: mat.as_str().to_string(),
                });
                remaining = &remaining[mat.end()..];
                matched = true;
                break;
            }
        }

        if !matched {
            let invalid_char = remaining.chars().take(3).collect::<String>();
            return Err(format!("Unknown token \"{}\"...", invalid_char));
        }
    }

    Ok(tokens)
}
