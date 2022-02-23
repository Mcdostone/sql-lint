use crate::data_type::PredefinedType;
use crate::formatter::Format;
use crate::identifier::Name;
use crate::identifier::SchemaQualifiedName;
use crate::list::List;
use crate::r#type::parse_user_defined_type_definition;
use crate::r#type::Member;
use crate::r#type::UserDefinedTypeDefinition;

#[test]
fn test_parse_user_defined_type_definition() {
    let input = "CREATE TYPE status AS ENUM ('beta','deprecated','stable')";
    assert_eq!(
        parse_user_defined_type_definition(input),
        Ok((
            "",
            UserDefinedTypeDefinition(
                SchemaQualifiedName(None, Name::Name("status".to_string())),
                Some(PredefinedType::Enum),
                List(vec!(
                    Member("'beta'".to_string()),
                    Member("'deprecated'".to_string()),
                    Member("'stable'".to_string())
                )),
            )
        ))
    )
}

#[test]
fn test_format_user_defined_type_definition_as() {
    let input = "CREATE TYPE status AS ENUM ('beta','deprecated','stable')";
    let (_, t) = parse_user_defined_type_definition(input).unwrap();
    assert_eq!(
        t.lol(),
        "CREATE TYPE 'status' AS ENUM (\n    'beta',\n    'deprecated',\n    'stable'\n)"
    )
}
