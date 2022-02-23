use crate::identifier::parse_name;
use crate::identifier::Name;
use crate::term::column::parse_column_ref;
use crate::term::column::ColumnRef;

#[test]
fn test_parse_name() {
    let input = "hello";
    assert_eq!(parse_name(input), Ok(("", Name::Name("hello".to_string()))))
}

#[test]
fn test_parse_quoted_name() {
    let input = "\"hello\"";
    assert_eq!(
        parse_name(input),
        Ok(("", Name::QuotedName("hello".to_string())))
    )
}

#[test]
fn test_column_ref() {
    let input = "user_name";
    assert_eq!(
        parse_column_ref(input),
        Ok(("", ColumnRef::Name(Name::Name("user_name".to_string()))))
    )
}

#[test]
fn test_column_ref_with_family() {
    let input = "users.user_name";
    assert_eq!(
        parse_column_ref(input),
        Ok((
            "",
            ColumnRef::WithFamily(
                Name::Name("users".to_string()),
                Name::Name("user_name".to_string())
            )
        ))
    )
}
