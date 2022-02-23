use crate::{
    formatter::Format,
    select::limit::{parse_limit_clause, LimitClause},
};

#[test]
fn test_parse_limit() {
    let input = "LIMIT 5";
    assert_eq!(parse_limit_clause(input), Ok(("", LimitClause::Number(5))))
}

#[test]
fn test_format_limit() {
    let input = "LIMIT    5";
    assert_eq!(parse_limit_clause(input).unwrap().1.lol(), "LIMIT 5");

    let input = "LIMIT    :1";
    assert_eq!(parse_limit_clause(input).unwrap().1.lol(), "LIMIT :1")
}
