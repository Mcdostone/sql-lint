use crate::{formatter::Format, select::clause::parse_select_clause};

#[test]
fn test_format_selected_expression() {
    assert_eq!(
        parse_select_clause("SELECT  *").unwrap().1.lol(),
        "SELECT *"
    );
    assert_eq!(
        parse_select_clause("SELECT  hello").unwrap().1.lol(),
        "SELECT hello"
    );
    assert_eq!(
        parse_select_clause("SELECT  users.*").unwrap().1.lol(),
        "SELECT users.*"
    );
    assert_eq!(
        parse_select_clause("SELECT  users.name, email")
            .unwrap()
            .1
            .lol(),
        "SELECT users.name, email"
    );
}
