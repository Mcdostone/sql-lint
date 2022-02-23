use crate::formatter::Format;
use crate::identifier::Name;
use crate::numeric::Numeric;
use crate::set::parse_set_statement;
use crate::set::SetStatement;
use crate::term::value::Value;

#[test]
fn test_parse_set_statement() {
    let input = "SET idle_in_transaction_session_timeout = 0";
    assert_eq!(
        parse_set_statement(input),
        Ok((
            "",
            SetStatement(
                Name::Name("idle_in_transaction_session_timeout".to_string()),
                Value::Num(Numeric::Int(0))
            )
        ))
    )
}

#[test]
fn test_format_set_statement() {
    let (_, t) = parse_set_statement("SET    idle_in_transaction_session_timeout = 0").unwrap();
    assert_eq!(t.output(), "SET idle_in_transaction_session_timeout = 0")
}
