use super::lexer::Token;

#[derive(Debug)]
pub enum Command {
    Search {
        name: Option<String>,
        in_path: String,
    },
    Delete {
        name: Option<String>,
        in_path: String,
    },
    Add {
        name: String,
        content: Option<String>,
    },
}

pub fn parse(tokens: Vec<Token>) -> Result<Command, String> {
    let mut iter = tokens.into_iter().peekable();

    if let Some(Token::Command(cmd)) = iter.next() {
        match cmd.as_str() {
            "SEARCH" => parse_search(&mut iter),
            "DELETE" => parse_delete(&mut iter),
            "ADD" => parse_add(&mut iter),
            _ => Err(format!("Unknown command: {}", cmd)),
        }
    } else {
        Err("Expected a command".to_string())
    }
}

// Hàm phụ để parse các command cụ thể
fn parse_search(
    iter: &mut std::iter::Peekable<std::vec::IntoIter<Token>>,
) -> Result<Command, String> {
    let mut name = None;
    let mut in_path = None;

    while let Some(token) = iter.next() {
        match token {
            Token::Identifier(key) if key == "name" => {
                if let Some(Token::StringLiteral(value)) = iter.next() {
                    name = Some(value);
                } else {
                    return Err("Expected a string literal after 'name'".to_string());
                }
            }
            Token::Identifier(key) if key == "IN" => {
                if let Some(Token::StringLiteral(value)) = iter.next() {
                    in_path = Some(value);
                } else {
                    return Err("Expected a string literal after 'IN'".to_string());
                }
            }
            Token::Semicolon => break,
            _ => return Err("Unexpected token in SEARCH command".to_string()),
        }
    }

    if let Some(in_path) = in_path {
        Ok(Command::Search { name, in_path })
    } else {
        Err("SEARCH command requires an 'IN' clause".to_string())
    }
}
