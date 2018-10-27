use std::iter::Peekable;

#[derive(Debug,PartialEq)]
pub enum TokenType {
    Word(String),
    DoubleQuotedString(String),
    SingleQuotedString(String),
    Or,
    And,
    Parenthesis(char),
    Pipe,
    Semicolon,
}

#[derive(Debug,PartialEq)]
pub enum ParseError {
    UnterminatedQuote,
    IncompleteAnd,
    InvalidCharacter,
}

fn is_word(c: char) -> bool {
    return c.is_alphanumeric() || c == '-';
}

fn get_word<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> String {
    let mut result = String::new();

    while let Some(&c) = iter.peek() {
        if !is_word(c) {
            break;
        }

        result.push(c);
        iter.next();
    }

    result
}

fn get_double_quoted_string<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<String, ParseError> {
    let mut result = String::from("\"");
    iter.next();

    while let Some(&c) = iter.peek() {
        if c == '"' {
            result.push(c);
            iter.next();
            return Ok(result);
        }

        result.push(c);
        iter.next();
    }

    Err(ParseError::UnterminatedQuote)
}

fn get_single_quoted_string<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<String, ParseError> {
    let mut result = String::from("\'");
    iter.next();

    while let Some(&c) = iter.peek() {
        if c == '\'' {
            result.push(c);
            iter.next();
            return Ok(result);
        }

        result.push(c);
        iter.next();
    }

    Err(ParseError::UnterminatedQuote)
}

fn get_and<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> Result<TokenType, ParseError> {
    iter.next();

    match iter.peek() {
        Some(&'&') => {
            iter.next();
            Ok(TokenType::And)
        }
        _ => Err(ParseError::IncompleteAnd),
    }
}

fn get_or_pipe<T: Iterator<Item = char>>(iter: &mut Peekable<T>) -> TokenType {
    iter.next();

    match iter.peek() {
        Some(&'|') => {
            iter.next();
            TokenType::Or
        }
        _ => TokenType::Pipe,
    }
}

pub fn parse(line: &String) -> Result<Vec<TokenType>, ParseError> {
    let mut tokens = Vec::new();

    let mut it = line.chars().peekable();

    while let Some(&c) = it.peek() {
        if c == '"' {
            let q = get_double_quoted_string(&mut it)?;
            tokens.push(TokenType::DoubleQuotedString(q));
        } else if c == '\'' {
            let q = get_single_quoted_string(&mut it)?;
            tokens.push(TokenType::SingleQuotedString(q));
        } else if c == '&' {
            let q = get_and(&mut it)?;
            tokens.push(q);
        } else if c == '|' {
            let q = get_or_pipe(&mut it);
            tokens.push(q);
        } else if c == ';' {
            tokens.push(TokenType::Semicolon);
            it.next();
        } else if c == '(' {
            tokens.push(TokenType::Parenthesis(c));
            it.next();
        } else if c == ')' {
            tokens.push(TokenType::Parenthesis(c));
            it.next();
        } else if is_word(c) {
            let q = get_word(&mut it);
            tokens.push(TokenType::Word(q));
        } else if c.is_whitespace() {
            it.next();
        } else {
            return Err(ParseError::InvalidCharacter);
        }
    }

    Ok(tokens)
}

#[test]
fn test_parse_simple() {
    let line = String::from("ls -l");

    assert_eq!(parse(&line).unwrap(), vec![
        TokenType::Word("ls".to_string()), TokenType::Word("-l".to_string())
    ]);
}

#[test]
fn test_parse_double_quote() {
    let line = String::from("echo \"hola mundo\"");

    assert_eq!(parse(&line).unwrap(), vec![
        TokenType::Word("echo".to_string()), TokenType::DoubleQuotedString("\"hola mundo\"".to_string())
    ]);
}

#[test]
fn test_parse_single_quote() {
    let line = String::from("echo \'hola mundo\'");

    assert_eq!(parse(&line).unwrap(), vec![
        TokenType::Word("echo".to_string()), TokenType::SingleQuotedString("\'hola mundo\'".to_string())
    ]);
}

#[test]
fn test_parse_semicolon() {
    let line = String::from("ls;cd");

    assert_eq!(parse(&line).unwrap(), vec![
        TokenType::Word("ls".to_string()), TokenType::Semicolon, TokenType::Word("cd".to_string())
    ]);
}

#[test]
fn test_parse_and() {
    let line = String::from("ls&&cd");

    assert_eq!(parse(&line).unwrap(), vec![
        TokenType::Word("ls".to_string()), TokenType::And, TokenType::Word("cd".to_string())
    ]);
}

#[test]
fn test_parse_or() {
    let line = String::from("ls||cd");

    assert_eq!(parse(&line).unwrap(), vec![
        TokenType::Word("ls".to_string()), TokenType::Or, TokenType::Word("cd".to_string())
    ]);
}

#[test]
fn test_parse_pipe() {
    let line = String::from("ls | grep foo");

    assert_eq!(parse(&line).unwrap(), vec![
        TokenType::Word("ls".to_string()), TokenType::Pipe, TokenType::Word("grep".to_string()), TokenType::Word("foo".to_string())
    ]);
}

#[test]
fn test_parse_parenthesis() {
    let line = String::from("()");

    assert_eq!(parse(&line).unwrap(), vec![
            TokenType::Parenthesis('('), TokenType::Parenthesis(')')
    ]);
}

#[test]
fn test_parse_unterminated_double_quote() {
    let line = String::from("echo \"hola mundo");

    assert_eq!(parse(&line).unwrap_err(), ParseError::UnterminatedQuote);
}

#[test]
fn test_parse_unterminated_single_quote() {
    let line = String::from("echo \'hola mundo");

    assert_eq!(parse(&line).unwrap_err(), ParseError::UnterminatedQuote);
}
