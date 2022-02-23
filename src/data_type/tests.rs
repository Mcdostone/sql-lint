use crate::{
    data_type::{parse_data_type, DataType, DateTimeOption, DateTimeType, PredefinedType},
    formatter::Format,
    list::List,
    numeric::Numeric,
    term::value::Value,
};

#[test]
fn test_parse_varchar() {
    let input = "VARCHAR";
    assert_eq!(
        parse_data_type(input),
        Ok(("", DataType(PredefinedType::Varchar, None)))
    )
}

#[test]
fn test_parse_varchar_100() {
    let input = "VARCHAR(100)";
    assert_eq!(
        parse_data_type(input),
        Ok((
            "",
            DataType(
                PredefinedType::Varchar,
                Some(List(vec!(Value::Num(Numeric::Int(100)))))
            )
        ))
    )
}

#[test]
fn test_format_data_type() {
    let input = "varchar(100)";
    let (_, t) = parse_data_type(input).unwrap();
    assert_eq!(t.output(), "VARCHAR(100)")
}
#[test]
fn test_parse_datetime() {
    let input = "TIMESTAMP WITHOUT TIME zone";
    assert_eq!(
        parse_data_type(input),
        Ok((
            "",
            DataType(
                PredefinedType::DateTime(DateTimeType::Timestamp(DateTimeOption(
                    None,
                    Some(false)
                ))),
                None
            )
        ))
    )
}

#[test]
fn test_format_datetime() {
    let (_, t) = parse_data_type("timestamp     WITHOUT TIME zone").unwrap();
    assert_eq!(t.output(), "TIMESTAMP WITHOUT TIME ZONE")
}

#[test]
fn test_format_time() {
    let (_, t) = parse_data_type("time(6)     with TIME zone").unwrap();
    assert_eq!(t.output(), "TIME(6) WITH TIME ZONE")
}

#[test]
fn test_format_date() {
    let (_, t) = parse_data_type("date").unwrap();
    assert_eq!(t.output(), "DATE")
}

#[test]
fn test_format_string() {
    assert_eq!("hello".to_string().output(), "hello")
}

#[test]
fn test_default_time() {
    let options: DateTimeType = Default::default();
    assert_eq!(options, DateTimeType::Date)
}
