
pub struct Token {
    content: String,
    r#type: TokenType
}

pub enum TokenType {
    Text,
    Html
}