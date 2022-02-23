use crate::format;

#[macro_export]
macro_rules! assert_format {
    ($actual:expr, $expected:expr) => {
        assert_eq!($actual.unwrap().1.lol(), $expected)
    };
}

#[macro_export]
macro_rules! assert_format_not_empty {
    ($actual:expr) => {
        assert!(!$actual.unwrap().1.lol().is_empty())
    };
}

#[test]
fn test_format() {
    assert_eq!(format("select 1;"), Ok("SELECT 1;".to_string()));
    let cannot_parse = "CREATE EXTENSION IF NOT EXISTS pgcrypto WITH SCHEMA public;";
    assert!(format(cannot_parse).is_err())
}
