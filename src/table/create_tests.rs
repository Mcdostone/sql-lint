use crate::assert_format;
use crate::formatter::Format;
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
                List(vec!(TableElement::ColumnDef(ColumnDef(
                    Delimitedidentifier::Name(Name::Name("id".to_string())),
                    DataType(PredefinedType::Integer, None),
                    None,
                    None
                ))))
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
                List(vec!(TableElement::ColumnDef(ColumnDef(
                    Delimitedidentifier::Name(Name::Name("id".to_string())),
                    DataType(PredefinedType::Integer, None),
                    None,
                    Some(ColumnConstraintDefinition(
                        None,
                        ColumnConstraint::Unique(UniqueSpecification::PrimaryKey)
                    ))
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
                List(vec!(TableElement::ColumnDef(ColumnDef(
                    Delimitedidentifier::Name(Name::Name("id".to_string())),
                    DataType(PredefinedType::Integer, None),
                    None,
                    Some(ColumnConstraintDefinition(
                        Some(ConstraintNameDefinition(Name::Name("c_unique".to_string()))),
                        ColumnConstraint::Unique(UniqueSpecification::Unique)
                    ))
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
