use crate::assert_format;
use crate::{formatter::Format, select::clause::parse_select_clause};

#[test]
fn test_format_selected_expression() {
    assert_format!(parse_select_clause("SELECT  *"), "SELECT *");
    assert_format!(parse_select_clause("SELECT  hello"), "SELECT hello");
    assert_format!(parse_select_clause("SELECT  users.*"), "SELECT users.*");
    assert_format!(
        parse_select_clause("SELECT  users.name, email"),
        "SELECT users.name, email"
    );

    assert_format!(parse_select_clause("SELECT  count(*)"), "SELECT COUNT(*)");
}
