use crate::assert_format_not_empty;
use crate::formatter::Format;
use crate::table::parse_table;

#[test]
fn test_parse_table() {
    assert!((parse_table("ALTER TABLE movies ADD COLUMN producer VARCHAR(255)").is_ok()));
    assert!((parse_table("CREATE TABLE downloads ( PRIMARY KEY (id) )").is_ok()))
}

#[test]
fn test_format_table() {
    assert_format_not_empty!(parse_table(
        "ALTER TABLE movies ADD COLUMN producer VARCHAR(255)"
    ));
    assert_format_not_empty!((parse_table("CREATE TABLE downloads (PRIMARY KEY (id))")))
}
