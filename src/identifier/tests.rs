use crate::{
    formatter::Format,
    identifier::{
        parse_delimited_identifier, parse_name, parse_schema_qualified_name, Delimitedidentifier,
        Name, SchemaQualifiedName,
    },
};

#[test]
fn test_parse_name() {
    assert_eq!(
        parse_name("username"),
        Ok(("", Name::Name("username".to_string())))
    )
}

#[test]
fn test_parse_quoted_name() {
    assert_eq!(
        parse_name("'username'"),
        Ok(("", Name::QuotedName("username".to_string())))
    )
}

#[test]
fn test_parse_schema_qualified_name() {
    assert_eq!(
        parse_schema_qualified_name("schema.table"),
        Ok((
            "",
            SchemaQualifiedName(
                Some(Name::Name("schema".to_string())),
                Name::Name("table".to_string())
            )
        ))
    )
}

#[test]
fn test_parse_delimited_name() {
    assert_eq!(
        parse_delimited_identifier("\"test\""),
        Ok((
            "",
            Delimitedidentifier::Quoted(Name::Name("test".to_string()))
        ))
    )
}

#[test]
fn test_format_delimited_name() {
    let (_, t) = parse_delimited_identifier("identifier").unwrap();
    assert_eq!(t.output(), "identifier".to_string())
}
