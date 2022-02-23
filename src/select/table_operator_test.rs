use crate::{
    formatter::Format,
    list::List,
    numeric::Numeric,
    select::{
        clause::{SelectClause, SelectedExpression},
        table_operator::{combined_tables, CombinedTables, QueryTerm, TableOperator},
        SelectStatement,
    },
    term::{value::Value, Term},
};

#[test]
fn test_union() {
    let input = "SELECT 1 UNION SELECT 2";
    assert_eq!(
        combined_tables(input),
        Ok((
            "",
            CombinedTables(
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(1))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                })),
                TableOperator::Union(false),
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(2))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                }))
            )
        ))
    )
}

#[test]
fn test_union_all() {
    let input = "SELECT 1 UNION ALL SELECT 2";
    assert_eq!(
        combined_tables(input),
        Ok((
            "",
            CombinedTables(
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(1))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                })),
                TableOperator::Union(true),
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(2))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                }))
            )
        ))
    )
}

#[test]
fn test_intersect() {
    let input = "SELECT 1 INTERSECT SELECT 2";
    assert_eq!(
        combined_tables(input),
        Ok((
            "",
            CombinedTables(
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(1))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                })),
                TableOperator::Intersect,
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(2))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                }))
            )
        ))
    )
}

#[test]
fn test_minus() {
    let input = "SELECT 1 MINUS SELECT 2";
    assert_eq!(
        combined_tables(input),
        Ok((
            "",
            CombinedTables(
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(1))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                })),
                TableOperator::Minus,
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(2))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                }))
            )
        ))
    )
}

#[test]
fn test_except() {
    let input = "SELECT 1 EXCEPT SELECT 2";
    assert_eq!(
        combined_tables(input),
        Ok((
            "",
            CombinedTables(
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(1))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                })),
                TableOperator::Except,
                QueryTerm::Select(Box::new(SelectStatement {
                    select: SelectClause(List(vec!(SelectedExpression::Term(Term::Value(
                        Value::Num(Numeric::Int(2))
                    ))))),
                    from: None,
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None
                }))
            )
        ))
    )
}

#[test]
fn test_format_combined_tables() {
    assert_eq!(
        combined_tables("SELECT 1 EXCEPT SELECT 2")
            .unwrap()
            .1
            .output(),
        "SELECT 1\n\nEXCEPT\n\nSELECT 2"
    );
    assert_eq!(
        combined_tables("SELECT 1 Union SELECT 2")
            .unwrap()
            .1
            .output(),
        "SELECT 1\n\n UNION\n\nSELECT 2"
    );
    assert_eq!(
        combined_tables("SELECT 1 Union all SELECT 2")
            .unwrap()
            .1
            .output(),
        "SELECT 1\n\n UNION ALL\n\nSELECT 2"
    );
    assert_eq!(
        combined_tables("(SELECT 1) minus (SELECT 2)")
            .unwrap()
            .1
            .output(),
        "(SELECT 1)\n\nMINUS\n\n(SELECT 2)"
    );
    assert_eq!(
        combined_tables("SELECT 1 intersect SELECT 2")
            .unwrap()
            .1
            .output(),
        "SELECT 1\n\nINTERSECT\n\nSELECT 2"
    );
}
