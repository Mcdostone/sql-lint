use crate::assert_format;
use crate::expression::Compare;
use crate::expression::Condition;
use crate::expression::Expression;
use crate::expression::Operand;
use crate::expression::RightOperand;
use crate::expression::WhereClause;
use crate::formatter::Format;
use crate::numeric::Numeric;
use crate::query::Query;
use crate::select::clause::SelectClause;
use crate::select::clause::SelectedExpression;
use crate::select::from::TableExpression;
use crate::select::from::TableName;
use crate::select::FromClause;
use crate::select::SelectStatement;
use crate::table::create::Subquery;
use crate::table::create::TableContentsSource;
use crate::term::value::Value;
use crate::term::Term;
use crate::{
    data_type::{DataType, PredefinedType},
    identifier::{Delimitedidentifier, Name},
    list::List,
    table::{
        constraint::{
            ColumnConstraint, ColumnConstraintDefinition, ConstraintNameDefinition,
            UniqueSpecification,
        },
        create::{parse_create_table, ColumnDef, CreateTableStatement, TableElement, TableRef},
    },
};

#[test]
fn test_create_table() {
    let input = "CREATE TABLE movies (id integer)";
    assert_eq!(
        parse_create_table(input),
        Ok((
            "",
            CreateTableStatement(
                TableRef(None, Name::Name("movies".to_string())),
                TableContentsSource::TableElementList(List(vec!(TableElement::ColumnDef(
                    ColumnDef(
                        Delimitedidentifier::Name(Name::Name("id".to_string())),
                        DataType(PredefinedType::Integer, None),
                        None,
                        None
                    )
                ))))
            )
        ))
    )
}

#[test]
fn test_create_table_as() {
    let input = "create table saucisse as (select * from food where 1 = 2)";
    assert_eq!(
        parse_create_table(input),
        Ok((
            "",
            CreateTableStatement(
                TableRef(None, Name::Name("saucisse".to_string())),
                TableContentsSource::As(Subquery(Box::new(Query::Select(Box::new(
                    SelectStatement {
                        select: SelectClause(None, List(vec!(SelectedExpression::All))),
                        from: Some(FromClause(
                            List(vec!(TableExpression(TableName::Name(Name::Name(
                                "food".to_string()
                            ))))),
                            None
                        )),
                        r#where: Some(WhereClause(List(vec!(Expression::Condition(
                            Condition::BinaryExpression(
                                Operand::Term(Term::Value(Value::Num(Numeric::Int(1)))),
                                RightOperand::Compare(
                                    Compare::Equal,
                                    Operand::Term(Term::Value(Value::Num(Numeric::Int(2)))),
                                )
                            )
                        ))))),
                        group_by: None,
                        order_by: None,
                        limit: None
                    }
                )))))
            )
        ))
    )
}

#[test]
fn test_create_table_primary_key() {
    let input = "CREATE TABLE movies (id integer primary key)";
    assert_eq!(
        parse_create_table(input),
        Ok((
            "",
            CreateTableStatement(
                TableRef(None, Name::Name("movies".to_string())),
                TableContentsSource::TableElementList(List(vec!(TableElement::ColumnDef(
                    ColumnDef(
                        Delimitedidentifier::Name(Name::Name("id".to_string())),
                        DataType(PredefinedType::Integer, None),
                        None,
                        Some(ColumnConstraintDefinition(
                            None,
                            ColumnConstraint::Unique(UniqueSpecification::PrimaryKey)
                        ))
                    )
                ))))
            )
        ))
    )
}

#[test]
fn test_create_table_constraint_name() {
    let input = "CREATE TABLE movies (id integer constraint c_unique UNIQUE)";
    assert_eq!(
        parse_create_table(input),
        Ok((
            "",
            CreateTableStatement(
                TableRef(None, Name::Name("movies".to_string())),
                TableContentsSource::TableElementList(List(vec!(TableElement::ColumnDef(
                    ColumnDef(
                        Delimitedidentifier::Name(Name::Name("id".to_string())),
                        DataType(PredefinedType::Integer, None),
                        None,
                        Some(ColumnConstraintDefinition(
                            Some(ConstraintNameDefinition(Name::Name("c_unique".to_string()))),
                            ColumnConstraint::Unique(UniqueSpecification::Unique)
                        ))
                    )
                ))))
            )
        ))
    )
}

#[test]
fn assert_format_create_table() {
    assert_format!(
        parse_create_table("CREATE TABLE movies (id integer constraint c UNIQUE, name varchar not null)"),
        "CREATE TABLE movies (\n        id INTEGER CONSTRAINT c UNIQUE,\n        name VARCHAR NOT NULL\n)"
    );

    assert_format!(
        parse_create_table("CREATE TABLE movies (id integer, PRIMARY KEY(id) )"),
        "CREATE TABLE movies (\n        id INTEGER,\n        PRIMARY KEY(id)\n)"
    );
    assert_format!(
        parse_create_table("CREATE TABLE movies (id integer, year integer DEFAULT 2022)"),
        "CREATE TABLE movies (\n        id INTEGER,\n        year INTEGER DEFAULT 2022\n)"
    )
}
