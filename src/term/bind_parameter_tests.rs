use crate::formatter::Format;
use crate::term::parse_bind_parameter;
use crate::term::BindParameter;

#[test]
fn test_parse_placeholder() {
    assert_eq!(
        parse_bind_parameter("?"),
        Ok(("", BindParameter::Placeholder))
    );
}

#[test]
fn test_parse_bind_parameter_with_index() {
    assert_eq!(
        parse_bind_parameter(":1"),
        Ok(("", BindParameter::Index(1)))
    );
}

#[test]
fn test_format_bind_parameter() {
    assert_eq!(BindParameter::Index(1).lol(), ":1");
    assert_eq!(BindParameter::Placeholder.lol(), "?");
}
