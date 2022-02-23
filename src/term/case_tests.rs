use crate::expression::Condition;
use crate::expression::Expression;
use crate::expression::Operand;
use crate::identifier::Name;
use crate::term::case::parse_case;
use crate::term::case::Case;
use crate::term::case::Else;
use crate::term::case::When;
use crate::term::column::ColumnRef;
use crate::term::value::Value;
use crate::term::Term;

#[test]
fn test_case() {
    let input = "CASE word WHEN 'hello' THEN 'bonjour' WHEN 'world' THEN 'monde' END";
    assert_eq!(
        parse_case(input),
        Ok((
            "",
            Case(
                Some(Box::new(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "word".to_string()
                ))))),
                vec!(
                    When(
                        Expression::Condition(Condition::Operand(Operand::Term(Term::Value(
                            Value::String("hello".to_string())
                        )))),
                        Term::Value(Value::String("bonjour".to_string()))
                    ),
                    When(
                        Expression::Condition(Condition::Operand(Operand::Term(Term::Value(
                            Value::String("world".to_string())
                        )))),
                        Term::Value(Value::String("monde".to_string()))
                    )
                ),
                None
            )
        ))
    )
}

#[test]
fn test_case_else() {
    let input = "CASE word WHEN 'hello' THEN 'bonjour' ELSE 'bon día' END";
    assert_eq!(
        parse_case(input),
        Ok((
            "",
            Case(
                Some(Box::new(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "word".to_string()
                ))))),
                vec!(When(
                    Expression::Condition(Condition::Operand(Operand::Term(Term::Value(
                        Value::String("hello".to_string())
                    )))),
                    Term::Value(Value::String("bonjour".to_string()))
                )),
                Some(Box::new(Else(Term::Value(Value::String(
                    "bon día".to_string()
                )))))
            )
        ))
    )
}
