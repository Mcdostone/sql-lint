use crate::{formatter::Format, list::List};

#[test]
fn test_format_list() {
    assert_eq!(
        List(vec!("hello".to_string(), "world".to_string())).lol(),
        "hello, world"
    )
}

#[test]
fn test_list_string() {
    assert_eq!(List(vec!("hello", "world")).to_string(), "hello, world")
}

