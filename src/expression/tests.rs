use crate::assert_format;
use crate::expression::condition::condition;
use crate::expression::condition::Compare;
use crate::expression::condition::Condition;
use crate::expression::condition::Operand;
use crate::expression::condition::RightOperand;
use crate::expression::parse_expressions;
use crate::expression::parse_in_predicate_value;
use crate::expression::InPredicateValue;
use crate::formatter::Format;
use crate::identifier::Name;
use crate::list::List;
use crate::numeric::Numeric;
use crate::term::column::ColumnRef;
use crate::term::value::Value;
use crate::term::Term;

#[test]
fn test_like() {
    let input = "title LIKE '%hello'";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "title".to_string()
                )))),
                RightOperand::Like(Operand::Term(Term::Value(Value::String(
                    "%hello".to_string()
                ))))
            )
        ))
    )
}

#[test]
fn test_between() {
    let input = "age BETWEEN 5 AND 10";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "age".to_string()
                )))),
                RightOperand::Between(
                    Operand::Term(Term::Value(Value::Num(Numeric::Int(5)))),
                    Operand::Term(Term::Value(Value::Num(Numeric::Int(10))))
                )
            )
        ))
    )
}

#[test]
fn test_in() {
    let input = "word IN ('hello', 'world')";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "word".to_string()
                )))),
                RightOperand::In(InPredicateValue::InValueList(List(vec!(
                    Operand::Term(Term::Value(Value::String("hello".to_string()))),
                    Operand::Term(Term::Value(Value::String("world".to_string())))
                ))))
            )
        ))
    )
}

#[test]
fn test_is_null() {
    let input = "word IS NULL";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "word".to_string()
                )))),
                RightOperand::Null(true)
            )
        ))
    )
}

#[test]
fn test_equal() {
    let input = "cmd = 'kill -9 -1'";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "cmd".to_string()
                )))),
                RightOperand::Compare(
                    Compare::Equal,
                    Operand::Term(Term::Value(Value::String("kill -9 -1".to_string())))
                )
            )
        ))
    )
}

#[test]
fn test_not_equal() {
    let input = "artist != 'Danit'";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "artist".to_string()
                )))),
                RightOperand::Compare(
                    Compare::NotEqual,
                    Operand::Term(Term::Value(Value::String("Danit".to_string())))
                )
            )
        ))
    )
}

#[test]
fn test_greater_than() {
    let input = "sponsors > 0";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "sponsors".to_string()
                )))),
                RightOperand::Compare(
                    Compare::GreaterThan,
                    Operand::Term(Term::Value(Value::Num(Numeric::Int(0))))
                )
            )
        ))
    )
}

#[test]
fn test_lower_than() {
    let input = "sponsors < 0";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "sponsors".to_string()
                )))),
                RightOperand::Compare(
                    Compare::LowerThan,
                    Operand::Term(Term::Value(Value::Num(Numeric::Int(0))))
                )
            )
        ))
    )
}

#[test]
fn test_greater_than_or_equal() {
    let input = "sponsors >= 0";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "sponsors".to_string()
                )))),
                RightOperand::Compare(
                    Compare::GreaterOrEqual,
                    Operand::Term(Term::Value(Value::Num(Numeric::Int(0))))
                )
            )
        ))
    )
}

#[test]
fn test_lower_than_or_equal() {
    let input = "sponsors <= 0";
    assert_eq!(
        condition(input),
        Ok((
            "",
            Condition::BinaryExpression(
                Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "sponsors".to_string()
                )))),
                RightOperand::Compare(
                    Compare::LowerOrEqual,
                    Operand::Term(Term::Value(Value::Num(Numeric::Int(0))))
                )
            )
        ))
    )
}

#[test]
fn test_format_in_predicate_value() {
    assert_format!(parse_in_predicate_value("(1, 2   ,3 )"), "(1, 2, 3)")
}

#[test]
fn test_format_in_predicate_value_subquery() {
    assert_format!(
        parse_in_predicate_value("(SELECT id from users)"),
        "\n (SELECT id\n          FROM users)"
    )
}

#[test]
fn test_format_expression() {
    assert_format!(
        parse_expressions("1 = 1 AND 'X' != 'Y' OR user in ('admin', 'root')"),
        "1 = 1\nAND 'X' != 'Y'\nOR user IN ('admin', 'root')"
    )
}

#[test]
fn test_format_between() {
    assert_format!(
        parse_expressions("id between 5 and 10"),
        "id BETWEEN 5 AND 10"
    )
}

#[test]
fn test_format_like() {
    assert_format!(
        parse_expressions("movie like '%dead%'"),
        "movie LIKE '%dead%'"
    )
}

#[test]
fn test_format_not_null() {
    assert_format!(parse_expressions("album IS not null"), "album IS NOT NULL")
}
