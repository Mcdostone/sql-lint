use crate::assert_format;
use crate::formatter::Format;
use crate::function::{parse_aggregate_function, parse_function, AggregateFunction, Function};
use crate::identifier::Name;
use crate::list::List;
use crate::term::column::ColumnRef;
use crate::term::Term;

#[test]
fn test_parse_function() {
    let input = "AVG(mark)";
    assert_eq!(
        parse_function(input),
        Ok((
            "",
            Function(
                "AVG".to_string(),
                List(vec!(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "mark".to_string()
                ))))),
            )
        ))
    )
}

#[test]
fn test_parse_aggregate_function() {
    let input = "COUNT(*)";
    assert_eq!(
        parse_aggregate_function(input),
        Ok(("", AggregateFunction::CountAll))
    )
}

#[test]
fn test_format_function() {
    assert_format!(parse_function("DO(1, '',true)"), "DO(1, '', true)");
    assert_format!(parse_aggregate_function("COUNT(*)"), "COUNT(*)")
}