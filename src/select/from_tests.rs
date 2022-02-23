use crate::assert_format;
use crate::formatter::Format;
use crate::identifier::Name;
use crate::list::List;
use crate::select::from::TableExpression;
use crate::select::from::TableName;
use crate::select::parse_from_clause;
use crate::select::FromClause;

#[test]
fn parse_from() {
    let input = "FROM users";
    assert_eq!(
        parse_from_clause(input),
        Ok((
            "",
            FromClause(
                List(vec!(TableExpression(TableName::Name(Name::Name(
                    "users".to_string()
                ))))),
                None
            )
        ))
    )
}

#[test]
fn parse_from_without_as() {
    let input = " FROM Employee e1";
    assert_eq!(
        parse_from_clause(input),
        Ok((
            "",
            FromClause(
                List(vec!(TableExpression(TableName::AliasedName(
                    Name::Name("Employee".to_string()),
                    Name::Name("e1".to_string())
                )))),
                None
            )
        ))
    )
}

#[test]
fn parse_from_quoted_name() {
    let input = "FROM \"users\"";
    assert_eq!(
        parse_from_clause(input),
        Ok((
            "",
            FromClause(
                List(vec!(TableExpression(TableName::Name(Name::QuotedName(
                    "users".to_string()
                ))))),
                None
            )
        ))
    )
}

#[test]
fn parse_from_single_quoted_name() {
    let input = "FROM 'users'";
    assert_eq!(
        parse_from_clause(input),
        Ok((
            "",
            FromClause(
                List(vec!(TableExpression(TableName::Name(Name::QuotedName(
                    "users".to_string()
                ))))),
                None
            )
        ))
    )
}

#[test]
fn parse_from_as() {
    let input = "FROM users as u";
    assert_eq!(
        parse_from_clause(input),
        Ok((
            "",
            FromClause(
                List(vec!(TableExpression(TableName::AliasedName(
                    Name::Name("users".to_string()),
                    Name::Name("u".to_string()),
                )))),
                None
            )
        ))
    )
}

#[test]
fn test_format_from_join() {
    assert_format!(parse_from_clause("FROM users  u"), "FROM users AS u")
}
