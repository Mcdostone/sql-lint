use crate::formatter::Format;
use crate::function::{parse_function, Function};
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
fn test_format_function() {
    let (_, t) = parse_function("DO(1, '',true)").unwrap();
    assert_eq!(t.output(), "DO(1, '', true)")
}
