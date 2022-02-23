use crate::term::value::parse_value;
use crate::term::value::Value;

#[test]
fn test_value_null() {
    let input = "NULL";
    assert_eq!(parse_value(input), Ok(("", Value::Null)))
}

#[test]
fn test_value_bool_true() {
    let input = "true";
    assert_eq!(parse_value(input), Ok(("", Value::Bool(true))))
}

#[test]
fn test_value_bool_false() {
    let input = "False";
    assert_eq!(parse_value(input), Ok(("", Value::Bool(false))))
}

#[test]
fn test_value_bool_string() {
    let input = "'Swiss army man'";
    assert_eq!(
        parse_value(input),
        Ok(("", Value::String("Swiss army man".to_string())))
    )
}
//#[test]
//fn test_value_numeric() {
//    let input = "3.14";
//    assert_eq!(
//        parse_value(input),
//        Ok(("", Value::Num(Numeric::Float(3.14))))
//    )
//}
