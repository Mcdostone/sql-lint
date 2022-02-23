use crate::formatter::Format;
use crate::{
    identifier::Name,
    table::{
        create::TableRef,
        drop_table::{parse_drop_table, DropTable},
    },
};

use crate::assert_format;

#[test]
fn test_drop_table() {
    let input = "DROP TABLE users";
    assert_eq!(
        parse_drop_table(input),
        Ok((
            "",
            DropTable(TableRef(None, Name::Name("users".to_string())), None)
        ))
    )
}

#[test]
fn test_format_drop_table() {
    assert_format!(
        parse_drop_table("drop TABLE users cascade"),
        "DROP TABLE users CASCADE"
    );
    assert_format!(
        parse_drop_table("drop TABLE users restrict"),
        "DROP TABLE users RESTRICT"
    );
    assert_format!(parse_drop_table("drop TABLE users"), "DROP TABLE users");
}
