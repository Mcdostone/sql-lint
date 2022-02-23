use crate::expression::Compare;
use crate::expression::Condition;
use crate::expression::Expression;
use crate::expression::Operand;
use crate::expression::RightOperand;
use crate::expression::WhereClause;
use crate::formatter::Format;
use crate::identifier::Name;
use crate::list::List;
use crate::numeric::Numeric;
use crate::table::create::TableRef;
use crate::term::column::ColumnRef;
use crate::term::value::Value;
use crate::term::Term;
use crate::update::parse_update_statement;
use crate::update::SetClause;
use crate::update::SetExpression;
use crate::update::UpdateClause;
use crate::update::UpdateStatement;

#[test]
fn test_update() {
    let input = "UPDATE movies SET description = ''";
    assert_eq!(
        parse_update_statement(input),
        Ok((
            "",
            UpdateStatement(
                UpdateClause(TableRef(None, Name::Name(String::from("movies")))),
                SetClause(List(vec!(SetExpression(
                    Name::Name("description".to_string()),
                    Expression::Condition(Condition::Operand(Operand::Term(Term::Value(
                        Value::String("".to_string())
                    ))))
                )))),
                None
            )
        ))
    )
}

#[test]
fn test_update_variant() {
    let input = "UPDATE movies SET title = 'Kaamelott', description = 'il revient pas pour trier les lentilles' WHERE id = 3";
    assert_eq!(
        parse_update_statement(input),
        Ok((
            "",
            UpdateStatement(
                UpdateClause(TableRef(None, Name::Name(String::from("movies")))),
                SetClause(List(vec!(
                    SetExpression(
                        Name::Name("title".to_string()),
                        Expression::Condition(Condition::Operand(Operand::Term(Term::Value(
                            Value::String("Kaamelott".to_string())
                        ))))
                    ),
                    SetExpression(
                        Name::Name("description".to_string()),
                        Expression::Condition(Condition::Operand(Operand::Term(Term::Value(
                            Value::String("il revient pas pour trier les lentilles".to_string())
                        ))))
                    )
                ))),
                Some(WhereClause(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "id".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::Value(Value::Num(Numeric::Int(3))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_update_when() {
    let input = "UPDATE movies SET title = 'Coup de tête' WHERE id = 1";
    assert_eq!(
        parse_update_statement(input),
        Ok((
            "",
            UpdateStatement(
                UpdateClause(TableRef(None, Name::Name(String::from("movies")))),
                SetClause(List(vec!(SetExpression(
                    Name::Name("title".to_string()),
                    Expression::Condition(Condition::Operand(Operand::Term(Term::Value(
                        Value::String("Coup de tête".to_string())
                    ))))
                )))),
                Some(WhereClause(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "id".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::Value(Value::Num(Numeric::Int(1))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_format_update_statement() {
    let input = "UPDATE movies SET description = ''";
    let (_, t) = parse_update_statement(input).unwrap();
    assert_eq!(t.output(), "UPDATE movies\n   SET description = ''")
}

#[test]
fn test_format_update_statement_where() {
    let input = "UPDATE movies SET description = '', title = 'Matrix'   WHERE id = 5";
    let (_, t) = parse_update_statement(input).unwrap();
    assert_eq!(
        t.output(),
        "UPDATE movies\n   SET description = '',\n       title = 'Matrix'\n WHERE id = 5"
    )
}
