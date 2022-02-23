use crate::select::column::{ColumnRef, Name};
use crate::select::condition::{Compare, Condition, Expression, Operand, RightOperand};
use crate::select::from::{TableExpression, TableName};
use crate::select::join::{
    join_statement, join_type, JoinSpecification, JoinStatement, JoinType, OuterJoin, OuterJoinType,
};
use crate::select::term::Term;
use crate::select::update::List;

#[test]
fn test_join_type_full() {
    let input = "FULL";
    assert_eq!(
        join_type(&input),
        Ok((
            "",
            JoinType::QualifedJoin(OuterJoin(OuterJoinType::Full, false))
        ))
    )
}

#[test]
fn test_join_type_full_outer() {
    let input = "FULL OUTER";
    assert_eq!(
        join_type(&input),
        Ok((
            "",
            JoinType::QualifedJoin(OuterJoin(OuterJoinType::Full, true))
        ))
    )
}

#[test]
fn test_join() {
    let input = "JOIN staff ON l = m";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::Default,
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::On(Expression::Condition(Condition::BinaryExpression(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "l".to_string()
                    )))),
                    RightOperand::Compare(
                        Compare::Equal,
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "m".to_string()
                        ))))
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_natural_join() {
    let input = "NATURAL JOIN staff ON l = m";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::Natural,
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::On(Expression::Condition(Condition::BinaryExpression(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "l".to_string()
                    )))),
                    RightOperand::Compare(
                        Compare::Equal,
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "m".to_string()
                        ))))
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_inner_join() {
    let input = "INNER JOIN staff ON l = m";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::Inner,
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::On(Expression::Condition(Condition::BinaryExpression(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "l".to_string()
                    )))),
                    RightOperand::Compare(
                        Compare::Equal,
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "m".to_string()
                        ))))
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_cross_join() {
    let input = "CROSS JOIN staff ON l = m";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::Cross,
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::On(Expression::Condition(Condition::BinaryExpression(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "l".to_string()
                    )))),
                    RightOperand::Compare(
                        Compare::Equal,
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "m".to_string()
                        ))))
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_left_join() {
    let input = "LEFT JOIN staff ON l = m";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Left, false)),
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::On(Expression::Condition(Condition::BinaryExpression(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "l".to_string()
                    )))),
                    RightOperand::Compare(
                        Compare::Equal,
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "m".to_string()
                        ))))
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_right_join() {
    let input = "RIGHT JOIN staff ON l = m";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Right, false)),
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::On(Expression::Condition(Condition::BinaryExpression(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "l".to_string()
                    )))),
                    RightOperand::Compare(
                        Compare::Equal,
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "m".to_string()
                        ))))
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_left_outer_join() {
    let input = "LEFT OUTER JOIN staff ON l = m";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Left, true)),
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::On(Expression::Condition(Condition::BinaryExpression(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "l".to_string()
                    )))),
                    RightOperand::Compare(
                        Compare::Equal,
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "m".to_string()
                        ))))
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_right_outer_join() {
    let input = "RIGHT OUTER JOIN staff ON l = m";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Right, true)),
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::On(Expression::Condition(Condition::BinaryExpression(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "l".to_string()
                    )))),
                    RightOperand::Compare(
                        Compare::Equal,
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "m".to_string()
                        ))))
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_full_outer_join() {
    let input = "FULL OUTER JOIN staff ON l = m";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Full, true)),
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::On(Expression::Condition(Condition::BinaryExpression(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "l".to_string()
                    )))),
                    RightOperand::Compare(
                        Compare::Equal,
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "m".to_string()
                        ))))
                    )
                )))
            )
        ))
    )
}

#[test]
fn test_join_using() {
    let input = "JOIN staff USING(id)";
    assert_eq!(
        join_statement(&input),
        Ok((
            "",
            JoinStatement(
                JoinType::Default,
                Box::new(TableExpression(
                    TableName::Name(Name::Name("staff".to_string())),
                    None
                )),
                JoinSpecification::Using(List(vec!(Expression::Condition(Condition::Operand(
                    Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                        "id".to_string()
                    ))))
                )))))
            )
        ))
    )
}
