enum TokenType {
    Word,
    DoubleQuotedString,
    SingleQuotedString,
    Blank,
    Or,
    And,
    Parenthesis,
}

#[derive(Debug,PartialEq)]
pub enum ParseError {
    UnterminatedQuote,
}

pub fn parse(line: &String) -> Result<Vec<String>, ParseError> {
    let mut tokens = Vec::new();
    let mut cur_token = String::new();
    let mut token_type = TokenType::Blank;

    for c in line.chars() {
        match token_type {
            TokenType::Blank => {
                if c == '"' {
                    token_type = TokenType::DoubleQuotedString;
                    cur_token.push(c);
                } else if c == '\'' {
                    token_type = TokenType::SingleQuotedString;
                    cur_token.push(c);
                } else if !c.is_whitespace() {
                    token_type = TokenType::Word;
                    cur_token.push(c);
                }
            }

            TokenType::Word => {
                if c.is_whitespace() {
                    token_type = TokenType::Blank;
                    tokens.push(cur_token.clone());
                    cur_token = String::new();
                } else if c.is_ascii_alphanumeric() {
                    cur_token.push(c);
                }
            }

            TokenType::DoubleQuotedString => {
                if c == '"' {
                    token_type = TokenType::Blank;
                    cur_token.push(c);
                    tokens.push(cur_token.clone());
                    cur_token = String::new();
                } else {
                    cur_token.push(c);
                }
            }

            TokenType::SingleQuotedString => {
                if c == '\'' {
                    token_type = TokenType::Blank;
                    cur_token.push(c);
                    tokens.push(cur_token.clone());
                    cur_token = String::new();
                } else {
                    cur_token.push(c);
                }
            }
            _ => {}
        }
    }

    match token_type {
        TokenType::SingleQuotedString |
        TokenType::DoubleQuotedString => {
            return Err(ParseError::UnterminatedQuote);
        }
        _ => {}
    }

    if cur_token.len() > 0 {
        tokens.push(cur_token);
    }

    Ok(tokens)
}

#[test]
fn test_parse_simple() {
    let line = String::from("ls -l");

    assert_eq!(parse(&line).unwrap(), vec!["ls".to_string(), "-l".to_string()]);
}

#[test]
fn test_parse_double_quote() {
    let line = String::from("echo \"hola mundo\"");

    assert_eq!(parse(&line).unwrap(), vec!["echo".to_string(), "\"hola mundo\"".to_string()]);
}

#[test]
fn test_parse_single_quote() {
    let line = String::from("echo \'hola mundo\'");

    assert_eq!(parse(&line).unwrap(), vec!["echo".to_string(), "\'hola mundo\'".to_string()]);
}

#[test]
fn test_parse_unterminated_string() {
    let line = String::from("echo \"hola mundo");

    assert_eq!(parse(&line).unwrap_err(), ParseError::UnterminatedQuote);
}
