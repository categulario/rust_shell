pub fn parse(line: &String) -> Result<Vec<&str>, &str> {
    return Ok(line.split_whitespace().collect());
}

#[test]
fn test_parse_simple() {
    let line = String::from("ls -l");

    assert_eq!(parse(&line), Ok(vec!["ls", "-l"]));
}

#[test]
#[ignore]
fn test_parse_double_quote() {
    let line = String::from("echo \"hola mundo\"");

    assert_eq!(parse(&line), Ok(vec!["echo", "\"hola mundo\""]));
}

#[test]
#[ignore]
fn test_parse_single_quote() {
    let line = String::from("echo \'hola mundo\'");

    assert_eq!(parse(&line), Ok(vec!["echo", "\'hola mundo\'"]));
}

#[test]
#[ignore]
fn test_parse_matches_quotes() {
    let line = String::from("echo \"hola mundo\'");

    assert_eq!(parse(&line), Err("Failed command line parsing"));
}
