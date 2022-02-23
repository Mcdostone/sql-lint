use crate::expression::Compare;
use crate::expression::Condition;
use crate::expression::Expression;
use crate::expression::Operand;
use crate::expression::RightOperand;
use crate::identifier::Name;
use crate::list::List;
use crate::select::from::TableExpression;
use crate::select::from::TableName;
use crate::select::join::join_type;
use crate::select::join::parse_join_clause;
use crate::select::join::JoinClause;
use crate::select::join::JoinSpecification;
use crate::select::join::JoinType;
use crate::select::join::OuterJoin;
use crate::select::join::OuterJoinType;
use crate::term::column::ColumnRef;
use crate::term::Term;

#[test]
fn test_join_type_full() {
    let input = "FULL";
    assert_eq!(
        join_type(input),
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
        join_type(input),
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
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::Default,
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::On(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "l".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                                "m".to_string()
                            ))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_natural_join() {
    let input = "NATURAL JOIN staff ON l = m";
    assert_eq!(
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::Natural,
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::On(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "l".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                                "m".to_string()
                            ))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_inner_join() {
    let input = "INNER JOIN staff ON l = m";
    assert_eq!(
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::Inner,
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::On(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "l".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                                "m".to_string()
                            ))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_cross_join() {
    let input = "CROSS JOIN staff ON l = m";
    assert_eq!(
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::Cross,
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::On(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "l".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                                "m".to_string()
                            ))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_left_join() {
    let input = "LEFT JOIN staff ON l = m";
    assert_eq!(
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Left, false)),
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::On(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "l".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                                "m".to_string()
                            ))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_right_join() {
    let input = "RIGHT JOIN staff ON l = m";
    assert_eq!(
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Right, false)),
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::On(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "l".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                                "m".to_string()
                            ))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_left_outer_join() {
    let input = "LEFT OUTER JOIN staff ON l = m";
    assert_eq!(
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Left, true)),
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::On(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "l".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                                "m".to_string()
                            ))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_right_outer_join() {
    let input = "RIGHT OUTER JOIN staff ON l = m";
    assert_eq!(
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Right, true)),
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::On(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "l".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                                "m".to_string()
                            ))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_full_outer_join() {
    let input = "FULL OUTER JOIN staff ON l = m";
    assert_eq!(
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::QualifedJoin(OuterJoin(OuterJoinType::Full, true)),
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::On(List(vec!(Expression::Condition(
                    Condition::BinaryExpression(
                        Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                            "l".to_string()
                        )))),
                        RightOperand::Compare(
                            Compare::Equal,
                            Operand::Term(Term::ColumnRef(ColumnRef::Name(Name::Name(
                                "m".to_string()
                            ))))
                        )
                    )
                )))))
            )
        ))
    )
}

#[test]
fn test_join_using() {
    let input = "JOIN staff USING(id)";
    assert_eq!(
        parse_join_clause(input),
        Ok((
            "",
            JoinClause(
                JoinType::Default,
                Box::new(TableExpression(TableName::Name(Name::Name(
                    "staff".to_string()
                )))),
                Some(JoinSpecification::Using(List(vec!(Expression::Condition(
                    Condition::Operand(Operand::Term(Term::ColumnRef(ColumnRef::Name(
                        Name::Name("id".to_string())
                    ))))
                )))))
            )
        ))
    )
}
