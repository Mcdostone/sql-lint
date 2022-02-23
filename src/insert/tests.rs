use crate::expression::Condition;
use crate::expression::Expression;
use crate::expression::Operand;
use crate::formatter::Format;
use crate::identifier::Name;
use crate::insert::parse_insert_into_statement;
use crate::insert::InsertIntoClause;
use crate::insert::InsertStatement;
use crate::insert::InsertValue;
use crate::insert::ValuesClause;
use crate::list::List;
use crate::numeric::Numeric;
use crate::table::create::TableRef;
use crate::term::value::Value;
use crate::term::Term;

#[test]
fn test_insert() {
    let input = "INSERT INTO movies (id, title) VALUES(1, 'Coup de torchon')";
    assert_eq!(
        parse_insert_into_statement(input),
        Ok((
            "",
            InsertStatement(
                InsertIntoClause(
                    TableRef(None, Name::Name(String::from("movies"))),
                    Some(List(vec!(
                        Name::Name(String::from("id")),
                        Name::Name(String::from("title"))
                    )))
                ),
                ValuesClause(List(vec!(InsertValue::ParenthesisExpression(List(vec!(
                    InsertValue::Expression(Box::new(Expression::Condition(Condition::Operand(
                        Operand::Term(Term::Value(Value::Num(Numeric::Int(1))))
                    )))),
                    InsertValue::Expression(Box::new(Expression::Condition(Condition::Operand(
                        Operand::Term(Term::Value(Value::String("Coup de torchon".to_string())))
                    ))))
                ))))))
            )
        ))
    )
}

#[test]
fn test_insert_into_without_columns() {
    let input = "INSERT INTO movies VALUES(1, 'Fils de plouc')";
    assert_eq!(
        parse_insert_into_statement(input),
        Ok((
            "",
            InsertStatement(
                InsertIntoClause(TableRef(None, Name::Name(String::from("movies"))), None),
                ValuesClause(List(vec!(InsertValue::ParenthesisExpression(List(vec!(
                    InsertValue::Expression(Box::new(Expression::Condition(Condition::Operand(
                        Operand::Term(Term::Value(Value::Num(Numeric::Int(1))))
                    )))),
                    InsertValue::Expression(Box::new(Expression::Condition(Condition::Operand(
                        Operand::Term(Term::Value(Value::String("Fils de plouc".to_string())))
                    ))))
                ))))))
            )
        ))
    )
}

#[test]
fn test_insert_into_with_schema() {
    let input =
        "INSERT INTO emule.movies VALUES(1, 'The french dispatch'), (2, 'Bo Nunham inside')";
    assert_eq!(
        parse_insert_into_statement(input),
        Ok((
            "",
            InsertStatement(
                InsertIntoClause(
                    TableRef(
                        Some(Name::Name("emule".to_string())),
                        Name::Name(String::from("movies"))
                    ),
                    None
                ),
                ValuesClause(List(vec!(
                    InsertValue::ParenthesisExpression(List(vec!(
                        InsertValue::Expression(Box::new(Expression::Condition(
                            Condition::Operand(Operand::Term(Term::Value(Value::Num(
                                Numeric::Int(1)
                            ))))
                        ))),
                        InsertValue::Expression(Box::new(Expression::Condition(
                            Condition::Operand(Operand::Term(Term::Value(Value::String(
                                "The french dispatch".to_string()
                            ))))
                        )))
                    ))),
                    InsertValue::ParenthesisExpression(List(vec!(
                        InsertValue::Expression(Box::new(Expression::Condition(
                            Condition::Operand(Operand::Term(Term::Value(Value::Num(
                                Numeric::Int(2)
                            ))))
                        ))),
                        InsertValue::Expression(Box::new(Expression::Condition(
                            Condition::Operand(Operand::Term(Term::Value(Value::String(
                                "Bo Nunham inside".to_string()
                            ))))
                        )))
                    )))
                )))
            )
        ))
    )
}

#[test]
fn test_format_insert_statement() {
    let (_, t) = parse_insert_into_statement(
        "INSERT INTO emule.movies VALUES(1, 'The french dispatch'), (2, 'Bo Nunham inside')",
    )
    .unwrap();
    assert_eq!(
        t.output(),
        "INSERT INTO emule.movies\nVALUES (1, 'The french dispatch')\n       (2, 'Bo Nunham inside')"
    )
}

#[test]
fn test_format_insert_statement_with_columns() {
    let (_, t) = parse_insert_into_statement(
        "INSERT INTO twitch.leaks(name, password) VALUES('dd', 'azerty'), ('gg', '1984')",
    )
    .unwrap();
    assert_eq!(
        t.output(),
        "INSERT INTO twitch.leaks (name, password)\nVALUES ('dd', 'azerty')\n       ('gg', '1984')"
    )
}
