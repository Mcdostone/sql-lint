use crate::assert_format;

use crate::data_type::DataType;
use crate::data_type::PredefinedType;
use crate::formatter::Format;
use crate::identifier::Delimitedidentifier;
use crate::identifier::Name;
use crate::list::List;
use crate::numeric::Numeric;
use crate::table::alter::parse_alter_table;
use crate::table::alter::AlterTable;
use crate::table::alter::AlterTableAction;
use crate::table::create::ColumnDef;
use crate::table::create::TableRef;
use crate::term::value::Value;

#[test]
fn test_parse_alter_table_statement() {
    let input = "ALTER TABLE movies ADD COLUMN producer VARCHAR(255)";
    assert_eq!(
        parse_alter_table(input),
        Ok((
            "",
            AlterTable(
                TableRef(None, Name::Name("movies".to_string())),
                vec!(AlterTableAction::AddColumnDefinition(ColumnDef(
                    Delimitedidentifier::Name(Name::Name("producer".to_string())),
                    DataType(
                        PredefinedType::Varchar,
                        Some(List(vec!(Value::Num(Numeric::Int(255)))))
                    ),
                    None,
                    None
                )))
            )
        ))
    )
}

#[test]
fn test_format_alter_table_statement() {
    assert_format!(
        parse_alter_table("ALTER TABLE movies ADD COLUMN producer VARCHAR(255)"),
        "ALTER TABLE movies\n        ADD COLUMN producer VARCHAR(255)"
    );
    assert_format!(
        parse_alter_table("ALTER TABLE movies DROP COLUMN producer"),
        "ALTER TABLE movies\n        DROP COLUMN producer"
    );
    assert_format!(
        parse_alter_table("ALTER TABLE movies DROP COLUMN producer CASCAde"),
        "ALTER TABLE movies\n        DROP COLUMN producer CASCADE"
    );
    assert_format!(
        parse_alter_table("ALTER TABLE software ADD CONSTRAINT fk_rails_b554d2cae5 FOREIGN KEY (version_id) REFERENCES public.versions(id)"),
        "ALTER TABLE software\n        ADD CONSTRAINT fk_rails_b554d2cae5 FOREIGN KEY (version_id) REFERENCES public.versions(id)"
    );
    assert_format!(
        parse_alter_table("ALTER TABLE public.users ALTER COLUMN age SET DEFAULT 18"),
        "ALTER TABLE public.users\n        ALTER COLUMN age SET DEFAULT 18"
    )
}
