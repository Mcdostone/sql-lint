use crate::expression::{Compare, RightOperand};
use crate::formatter::Format;
use crate::numeric::Numeric;
use crate::select::group::{GroupingElement, HavingClause};
use crate::term::value::Value;
use crate::{
    expression::{Condition, Expression, Operand},
    identifier::Name,
    list::List,
    select::group::{parse_group_by_clause, GroupByClause},
    term::{column::ColumnRef, Term},
};

#[test]
fn test_group_by() {
    let input = "GROUP BY year, title";
    assert_eq!(
        parse_group_by_clause(input),
        Ok((
            "",
            GroupByClause(
                List(vec!(
                    GroupingElement::ColumnRef(ColumnRef::Name(Name::Name("year".to_string()))),
                    GroupingElement::ColumnRef(ColumnRef::Name(Name::Name("title".to_string())))
                )),
                None
            )
        ))
    )
}

#[test]
fn test_group_by_having() {
    let input = "GROUP BY year, title HAVING year > 2001";
    assert_eq!(
        parse_group_by_clause(input),
        Ok((
            "",
            GroupByClause(
                List(vec!(
                    GroupingElement::ColumnRef(ColumnRef::Name(Name::Name("year".to_string()))),
                    GroupingElement::ColumnRef(ColumnRef::Name(Name::Name("title".to_string())))
                )),
                Some(HavingClause(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "year".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::GreaterThan,
                            Operand::Term(Term::Value(Value::Num(Numeric::Int(2001))))
                        )
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_format_group_by() {
    let (_, t) = parse_group_by_clause("group by age HAVING email Like '%proton%'").unwrap();
    assert_eq!(t.lol(), "GROUP BY age\nHAVING email LIKE '%proton%'");

    let (_, t) = parse_group_by_clause("group by age, lastname").unwrap();
    assert_eq!(t.lol(), "GROUP BY age, lastname")
}
