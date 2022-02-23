use crate::comment::{parse_multiline_comment};

#[test]
fn test_inline_comment() {
    let input = "-- hello world\nSELECT";
    assert_eq!(parse_inline_comment(&input), Ok(("SELECT", ())))
}

#[test]
fn test_multiline_comment() {
    let input = "/** hello world\n, bye*/";
    assert_eq!(parse_multiline_comment(input), Ok(("", ())))
}