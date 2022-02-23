use crate::formatter::Formatter;

#[test]
fn test_format() {
    let mut f = Formatter::new();
    assert_eq!(
        f.append_left_right(&"hello", &"world").output(),
        "hello world"
    )
}

#[test]
fn test_format_pad() {
    let mut f = Formatter::new();
    f.set_pad(10);
    assert_eq!(
        f.append_left_right(&"hello", &"world").output(),
        "     hello world"
    )
}

#[test]
fn test_format_padd() {
    let mut f = Formatter::new();
    f.set_pad(6);
    assert_eq!(
        f.append_left_right(&"SELECT", &"r.last_name,")
            .new_context()
            .test(&"(SELECT MAX(YEAR(championship_date))")
            .output(),
        "SELECT r.last_name,      (SELECT MAX(YEAR(championship_date))"
    )
}
