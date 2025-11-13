pub struct Token {
    pub token_type: TokenType,
    pub token_value: String,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TokenType {
    PLUS,
    MINUS,
    MUL,
    DIV,
    LPAREN,
    RPAREN,
    NUMBER,
}
