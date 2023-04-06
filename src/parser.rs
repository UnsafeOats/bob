use crate::lexer::Token;

#[derive(Debug, PartialEq)]
pub enum Statement {
    Read(String, String),
    Write(String, String),
    Print(String),
    Append(String, String, String),
}

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Statement>, String> {
    let mut statements = Vec::new();
    let mut tokens_iter = tokens.into_iter().peekable();

    while let Some(token) = tokens_iter.next() {
        println!("Token: {:?}", token);
        let statement = match token {
            Token::Read => {
                let src = get_identifier(&mut tokens_iter)?;
                tokens_iter.next(); // Consume '->'
                let dest = get_identifier(&mut tokens_iter)?;
                tokens_iter.next(); // Consume EOL
                Statement::Read(src, dest)
            }
            Token::Write => {
                let path = get_identifier(&mut tokens_iter)?;
                let content = match tokens_iter.next() {
                    Some(Token::StringLiteral(s)) => s,
                    Some(Token::Identifier(s)) => s,
                    Some(anything) => {
                        return Err(format!("Expected a string literal, got {:?}", anything))
                    }
                    _ => return Err("Expected a string literal.".to_string()),
                };
                tokens_iter.next(); // Consume EOL
                Statement::Write(path, content)
            }
            Token::Print => {
                let content = match tokens_iter.next() {
                    Some(Token::StringLiteral(s)) => s,
                    Some(Token::Identifier(s)) => s,
                    Some(anything) => {
                        return Err(format!("Expected a string literal, got {:?}", anything))
                    }
                    _ => return Err("Expected a string literal.".to_string()),
                };
                tokens_iter.next(); // Consume EOL
                Statement::Print(content)
            }
            Token::Append => {
                let src1 = get_identifier(&mut tokens_iter)?;
                let src2 = match tokens_iter.next() {
                    Some(Token::StringLiteral(s)) => s,
                    Some(Token::Identifier(s)) => s,
                    Some(anything) => {
                        return Err(format!("Expected a string literal, got {:?}", anything))
                    }
                    _ => return Err("Expected a string literal.".to_string()),
                };
                tokens_iter.next(); // Consume '->'
                let dest = get_identifier(&mut tokens_iter)?;
                tokens_iter.next(); // Consume EOL
                Statement::Append(src1, src2, dest)
            }
            Token::Eol => continue,
            _ => return Err("Unexpected token.".to_string()),
        };
        statements.push(statement);
    }
    Ok(statements)
}

fn get_identifier<I>(tokens_iter: &mut I) -> Result<String, String>
where
    I: Iterator<Item = Token>,
{
    match tokens_iter.next() {
        Some(Token::Identifier(s)) => Ok(s),
        _ => Err("Expected an identifier.".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_basic() {
        use Token::*;

        let tokens = vec![
            Read,
            Identifier("input.txt".to_string()),
            Arrow,
            Identifier("content".to_string()),
            Eol,
            Write,
            Identifier("output.txt".to_string()),
            StringLiteral("Hello, World!".to_string()),
            Eol,
            Print,
            StringLiteral("Hello, World!".to_string()),
            Eol,
            Append,
            Identifier("var1".to_string()),
            StringLiteral("var2".to_string()),
            Arrow,
            Identifier("result".to_string()),
            Eol,
        ];

        let ast = parse(tokens).unwrap();
        let expected = vec![
            Statement::Read("input.txt".to_string(), "content".to_string()),
            Statement::Write("output.txt".to_string(), "Hello, World!".to_string()),
            Statement::Print("Hello, World!".to_string()),
            Statement::Append("var1".to_string(), "var2".to_string(), "result".to_string()),
        ];
        assert_eq!(ast, expected);
    }
}
