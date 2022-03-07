use crate::expression::Condition;
use crate::expression::Expression;
use crate::expression::Operand;
use crate::function::Function;
use crate::identifier::Name;
use crate::list::List;
use crate::numeric::Numeric;
use crate::term::case::Case;
use crate::term::case::When;
use crate::term::column::ColumnRef;
use crate::term::parse_term;
use crate::term::value::Value;
use crate::term::AggregateFunction;
use crate::term::BindParameter;
use crate::term::CaseExpression;
use crate::term::Term;

#[test]
fn test_term_value() {
    let input = "'hello'";
    assert_eq!(
        parse_term(input),
        Ok(("", Term::Value(Value::String("hello".to_string()))))
    )
}

#[test]
fn test_term_case() {
    let input = "CASE year WHEN 2001 THEN 0   END";
    assert_eq!(
        parse_term(input),
        Ok((
            "",
            Term::Case(CaseExpression::Simple(Case(
                Box::new(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "year".to_string()
                )))),
                vec!(When(
                    Expression::Condition(Condition::Operand(Operand::Term(Term::Value(
                        Value::Num(Numeric::Int(2001))
                    )))),
                    Term::Value(Value::Num(Numeric::Int(0)))
                )),
                None
            )),)
        ))
    )
}

#[test]
fn test_term_bind_parameter() {
    let input = ":1";
    assert_eq!(
        parse_term(input),
        Ok(("", Term::BindParameter(BindParameter::Index(1)),))
    )
}

#[test]
fn test_term_column_ref() {
    let input = "songs.title";
    assert_eq!(
        parse_term(input),
        Ok((
            "",
            Term::ColumnRef(ColumnRef::WithFamily(
                Name::Name("songs".to_string()),
                Name::Name("title".to_string())
            ))
        ))
    )
}

#[test]
fn test_term_function() {
    let input = "AVG(price)";
    assert_eq!(
        parse_term(input),
        Ok((
            "",
            Term::Function(AggregateFunction::Function(Function(
                "AVG".to_string(),
                List(vec!(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "price".to_string()
                )))))
            )))
        ))
    )
}

#[test]
fn test_term_aliased_term() {
    let input = "apiKey AS api_key";
    assert_eq!(
        parse_term(input),
        Ok((
            "",
            Term::AliasedTerm(
                Box::new(Term::ColumnRef(ColumnRef::Name(Name::Name(
                    "apiKey".to_string()
                )))),
                Name::Name("api_key".to_string())
            )
        ))
    )
}
