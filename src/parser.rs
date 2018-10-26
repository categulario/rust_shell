pub fn parse<'a>(line: &'a String) -> Vec<&'a str> {
    return line.split_whitespace().collect();
}

#[test]
fn test_parse_simple() {
    let line = String::from("ls -l");

    assert_eq!(parse(&line), ["ls", "-l"]);
}
