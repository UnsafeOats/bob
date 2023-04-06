#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Read,
    Write,
    Print,
    Append,
    Identifier(String),
    StringLiteral(String),
    IntLiteral(i64),
    FloatLiteral(f64),
    Arrow,
    Eol,
}

pub fn tokenize(source: &str) -> Result<Vec<Token>, String> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    while let Some(ch) = chars.next() {
        let token = match ch {
            ' ' | '\t' | '\r' => continue,
            '\n' => {
                tokens.push(Token::Eol);
                continue;
            }
            '#' => {
                while let Some(c) = chars.peek() {
                    if *c == '\n' {
                        break;
                    }
                    chars.next();
                }
                continue;
            }
            '-' => {
                let next_char = chars.next();
                if next_char == Some('>') {
                    Token::Arrow
                } else {
                    return Err(format!("Unexpected character: '{:?}'", next_char));
                }
            }
            '"' => {
                let mut s = String::new();
                while let Some(c) = chars.next() {
                    match c {
                        '\\' => {
                            if let Some(next_char) = chars.next() {
                                match next_char {
                                    'n' => s.push('\n'),
                                    't' => s.push('\t'),
                                    'r' => s.push('\r'),
                                    '"' => s.push('"'),
                                    '\\' => s.push('\\'),
                                    _ => {
                                        return Err(format!(
                                            "Invalid escape sequence: \\{}",
                                            next_char
                                        ))
                                    }
                                }
                            } else {
                                return Err("Unexpected end of input.".to_string());
                            }
                        }
                        '"' => break,
                        _ => s.push(c),
                    }
                }
                tokens.push(Token::StringLiteral(s));
                continue;
            }
            '.' => {
                if let Some(&ch) = chars.peek() {
                    if ch.is_numeric() {
                        let mut number = String::new();
                        number.push(ch);
                        chars.next();

                        while let Some(ch) = chars.peek() {
                            if ch.is_numeric() {
                                number.push(*ch);
                                chars.next();
                            } else {
                                break;
                            }
                        }

                        match number.parse::<f64>() {
                            Ok(n) => Token::FloatLiteral(n),
                            Err(_) => return Err(format!("Invalid number: {}", number)),
                        }
                    } else {
                        Token::Identifier(".".to_string())
                    }
                } else {
                    Token::Identifier(".".to_string())
                }
            }
            '0'..='9' => {
                let mut number = String::new();
                number.push(ch);

                while let Some(c) = chars.peek() {
                    if c.is_numeric() {
                        number.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }

                let mut is_float = false;
                if let Some('.') = chars.peek() {
                    is_float = true;
                    number.push('.');
                    chars.next();

                    while let Some(c) = chars.peek() {
                        if c.is_numeric() {
                            number.push(*c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                }

                if is_float {
                    tokens.push(Token::FloatLiteral(number.parse::<f64>().unwrap()));
                } else {
                    tokens.push(Token::IntLiteral(number.parse::<i64>().unwrap()));
                }
                continue;
            }
            'a'..='z' | 'A'..='Z' | '_' => {
                let mut identifier = String::new();
                identifier.push(ch);

                while let Some(c) = chars.peek() {
                    if c.is_alphanumeric() || c == &'_' || c == &'.' {
                        identifier.push(*c);
                        chars.next();
                    } else {
                        break;
                    }
                }
                match identifier.as_str() {
                    "READ" => Token::Read,
                    "WRITE" => Token::Write,
                    "PRINT" => Token::Print,
                    "APPEND" => Token::Append,
                    _ => Token::Identifier(identifier),
                }
            }
            _ => return Err(format!("Unexpected character: '{}'.", ch)),
        };
        tokens.push(token);
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lexer_basic() {
        let source = r#"READ input.txt -> content
WRITE output.txt "Hello, World!"
PRINT "Hello, World!"
APPEND var1 var2 -> result
"#;

        let tokens = tokenize(source).unwrap();
        let expected = vec![
            Token::Read,
            Token::Identifier("input.txt".to_string()),
            Token::Arrow,
            Token::Identifier("content".to_string()),
            Token::Eol,
            Token::Write,
            Token::Identifier("output.txt".to_string()),
            Token::StringLiteral("Hello, World!".to_string()),
            Token::Eol,
            Token::Print,
            Token::StringLiteral("Hello, World!".to_string()),
            Token::Eol,
            Token::Append,
            Token::Identifier("var1".to_string()),
            Token::Identifier("var2".to_string()),
            Token::Arrow,
            Token::Identifier("result".to_string()),
            Token::Eol,
        ];
        assert_eq!(tokens, expected);
    }
}
