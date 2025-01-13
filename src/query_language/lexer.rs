#[derive(Debug, PartialEq)]
pub enum Token {
    Command(String),
    Identifier(String),
    Colon,
    Comma,
    Semicolon,
    StringLiteral(String),
    NumberLiteral(f64),
}

pub fn lex(input: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    while let Some(&c) = chars.peek() {
        match c {
            ':' => {
                chars.next();
                tokens.push(Token::Colon);
            }
            ',' => {
                chars.next();
                tokens.push(Token::Comma);
            }
            ';' => {
                chars.next();
                tokens.push(Token::Semicolon);
            }
            '"' => {
                chars.next(); // Bỏ dấu mở ngoặc kép
                let string: String = chars.by_ref().take_while(|&ch| ch != '"').collect();
                chars.next(); // Bỏ dấu đóng ngoặc kép
                tokens.push(Token::StringLiteral(string));
            }
            c if c.is_alphabetic() => {
                let identifier: String = chars
                    .by_ref()
                    .take_while(|&ch| ch.is_alphanumeric() || ch == '_')
                    .collect();
                tokens.push(Token::Command(identifier));
            }
            c if c.is_whitespace() => {
                chars.next();
            }
            _ => return Err(format!("Unexpected character: {}", c)),
        }
    }

    Ok(tokens)
}
