enum TokenType {
    Word,
    QuotedString,
    Blank,
    Or,
    And,
    Parenthesis,
}

pub fn parse(line: &String) -> Result<Vec<String>, &str> {
    let mut tokens = Vec::new();
    let mut cur_token = String::new();
    let mut token_type = TokenType::Blank;

    for c in line.chars() {
        match token_type {
            TokenType::Blank => {
                if !c.is_whitespace() {
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
            _ => {}
        }
    }

    if cur_token.len() > 0 {
        tokens.push(cur_token);
    }

    Ok(tokens)
}

#[test]
fn test_parse_simple() {
    let line = String::from("ls -l");

    assert_eq!(parse(&line), Ok(vec!["ls".to_string(), "-l".to_string()]));
}

// 1#[test]
// 1#[ignore]
// 1fn test_parse_double_quote() {
    // 1let line = String::from("echo \"hola mundo\"");
// 1
    // 1assert_eq!(parse(&line), Ok(vec!["echo", "\"hola mundo\""]));
// 1}
// 1
// 1#[test]
// 1#[ignore]
// 1fn test_parse_single_quote() {
    // 1let line = String::from("echo \'hola mundo\'");
// 1
    // 1assert_eq!(parse(&line), Ok(vec!["echo", "\'hola mundo\'"]));
// 1}
// 1
// 1#[test]
// 1#[ignore]
// 1fn test_parse_matches_quotes() {
    // 1let line = String::from("echo \"hola mundo\'");
// 1
    // 1assert_eq!(parse(&line), Err("Failed command line parsing"));
// 1}
