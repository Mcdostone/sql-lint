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
    let input = "CREATE TABLE movies (id integer constraint unique UNIQUE)";
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
                        Some(ConstraintNameDefinition(Name::Name("unique".to_string()))),
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
        parse_create_table("CREATE TABLE movies (id integer constraint unique UNIQUE, name varchar not null)"),
        "CREATE TABLE movies (\n        id INTEGER CONSTRAINT unique UNIQUE,\n        name VARCHAR NOT NULL\n)"
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

/*
#[test]
fn test_alter_table_add() {
    let input = "ALTER TABLE languages ADD url VARCHAR(255)";
    assert_eq!(
        parse_alter_table(&input),
        Ok((
            "",
            AlterTable(
                TableRef(None, Name::Name("languages".to_string())),
                vec!(AlterTableAction::AddColumnDefinition(ColumnDef(
                    Name::Name("url".to_string()),
                    DataType::Varchar(Some(255)),
                    None
                )))
            )
        ))
    )
}

#[test]
fn test_alter_table_drop() {
    let input = "ALTER TABLE languages DROP url";
    assert_eq!(
        alter_table(&input),
        Ok((
            "",
            AlterTable(
                TableRef(None, Name::Name("languages".to_string())),
                vec!(AlterTableAction::DropColumnDefinition(
                    Name::Name("url".to_string()),
                    None
                ))
            )
        ))
    )
}
*/
