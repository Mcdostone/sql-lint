use crate::numeric::parse_numeric;
use crate::numeric::Numeric;

#[test]
fn test_numeric_int() {
    let input = "5";
    assert_eq!(parse_numeric(input), Ok(("", Numeric::Int(5))))
}

#[test]
fn test_numeric_int_negative() {
    let input = "-5";
    assert_eq!(parse_numeric(input), Ok(("", Numeric::Int(-5))))
}

#[test]
fn test_numeric_long() {
    let input = "5.5";
    assert_eq!(parse_numeric(input), Ok(("", Numeric::Float(5.5))))
}

#[test]
fn test_numeric_decimal() {
    let input = "5.555";
    assert_eq!(parse_numeric(input), Ok(("", Numeric::Float(5.555))))
}
