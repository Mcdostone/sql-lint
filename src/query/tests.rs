use crate::assert_format;
use crate::expression::Condition;
use crate::expression::Expression;
use crate::expression::Operand;
use crate::formatter::Format;
use crate::identifier::Name;
use crate::insert::InsertIntoClause;
use crate::insert::InsertStatement;
use crate::insert::InsertValue;
use crate::insert::ValuesClause;
use crate::list::List;
use crate::numeric::Numeric;
use crate::parse_statements;
use crate::query::Query;
use crate::query::Statement;
use crate::select::clause::SelectedExpression;
use crate::select::from::TableExpression;
use crate::select::from::TableName;
use crate::select::FromClause;
use crate::table::create::TableRef;
use crate::term::value::Value;
use crate::term::Term;

use crate::select::clause::SelectClause;
use crate::select::SelectStatement;

#[test]
fn test_parse_statements() {
    let input =
        "SELECT 1 from users; INSERT INTO albums (title, release_year) VALUES ('Aliento', 2017);";
    assert_eq!(
        parse_statements(input),
        Ok((
            "",
            List(vec!(
                Statement(Query::Select(Box::new(SelectStatement {
                    select: SelectClause(
                        None,
                        List(vec!(SelectedExpression::Term(Term::Value(Value::Num(
                            Numeric::Int(1)
                        )))))
                    ),
                    from: Some(FromClause(
                        List(vec!(TableExpression(TableName::Name(Name::Name(
                            "users".to_string()
                        ))))),
                        None
                    )),
                    r#where: None,
                    group_by: None,
                    order_by: None,
                    limit: None,
                }))),
                Statement(Query::Insert(InsertStatement(
                    InsertIntoClause(
                        TableRef(None, Name::Name(String::from("albums"))),
                        Some(List(vec!(
                            Name::Name(String::from("title")),
                            Name::Name(String::from("release_year"))
                        )))
                    ),
                    ValuesClause(List(vec!(InsertValue::ParenthesisExpression(List(vec!(
                        InsertValue::Expression(Box::new(Expression::Condition(
                            Condition::Operand(Operand::Term(Term::Value(Value::String(
                                "Aliento".to_string()
                            ))))
                        ))),
                        InsertValue::Expression(Box::new(Expression::Condition(
                            Condition::Operand(Operand::Term(Term::Value(Value::Num(
                                Numeric::Int(2017)
                            ))))
                        ))),
                    ))))))
                )))
            ))
        ))
    )
}

#[test]
fn test_format_queries() {
    assert_format!(parse_statements("SELECT 1 from users; INSERT INTO albums (title, release_year) VALUES ('Aliento', 2017);"),
        "SELECT 1\n  FROM users;\n\nINSERT INTO albums (title, release_year)\nVALUES ('Aliento', 2017);"
    );
    assert_format!(parse_statements("SELECT column_name FROM table1 LEFT JOIN table2 ON table1.column_name = table2.column_name;"),
    "SELECT column_name\n  FROM table1\n       LEFT JOIN table2\n       ON table1.column_name = table2.column_name;"
    )
}
