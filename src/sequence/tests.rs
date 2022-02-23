use crate::formatter::Format;
use crate::sequence::parse_alter_sequence;
use crate::sequence::parse_drop_sequence;
use crate::sequence::parse_sequence;
use crate::sequence::AlterSequence;
use crate::sequence::DropSequence;
use crate::sequence::Name;
use crate::sequence::SchemaQualifiedName;
use crate::sequence::Sequence;
use crate::sequence::SequenceGeneratorOption;

#[test]
fn test_sequence() {
    let input = "CREATE SEQUENCE stars";
    assert_eq!(
        parse_sequence(input),
        Ok((
            "",
            Sequence(
                SchemaQualifiedName(None, Name::Name("stars".to_string())),
                vec!()
            )
        ))
    )
}

#[test]
fn test_sequence_options() {
    let input = "CREATE SEQUENCE stars
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1";
    assert_eq!(
        parse_sequence(input),
        Ok((
            "",
            Sequence(
                SchemaQualifiedName(None, Name::Name("stars".to_string())),
                vec!(
                    SequenceGeneratorOption::Startwith(1),
                    SequenceGeneratorOption::IncrementBy(1),
                    SequenceGeneratorOption::NoMinValue,
                    SequenceGeneratorOption::NoMaxValue,
                    SequenceGeneratorOption::Cache(1),
                )
            )
        ))
    )
}

#[test]
fn test_alter_sequence() {
    let input = "ALTER SEQUENCE stars INCREMENT BY 2";
    assert_eq!(
        parse_alter_sequence(input),
        Ok((
            "",
            AlterSequence(Sequence(
                SchemaQualifiedName(None, Name::Name("stars".to_string())),
                vec!(SequenceGeneratorOption::IncrementBy(2),)
            ))
        ))
    )
}

#[test]
fn test_drop_sequence() {
    let input = "DROP SEQUENCE stars";
    assert_eq!(
        parse_drop_sequence(input),
        Ok(("", DropSequence(Name::Name("stars".to_string()))))
    )
}

#[test]
fn test_format_sequence() {
    let (_, t) = parse_sequence(
        "CREATE SEQUENCE stars START WITH 1 INCREMENT BY 1 NO MINVALUE NO MAXVALUE CACHE 1",
    )
    .unwrap();
    assert_eq!(
        t.lol(),
        "CREATE SEQUENCE 'stars'\n    START WITH 1\n    INCREMENT BY 1\n    NO MINVALUE\n    NO MAXVALUE\n    CACHE 1"
    )
}

#[test]
fn test_format_sequence_max_min() {
    let (_, t) = parse_sequence("CREATE SEQUENCE stars MINVALUE  1 MAXVALUE 10").unwrap();
    assert_eq!(
        t.lol(),
        "CREATE SEQUENCE 'stars'\n    MINVALUE 1\n    MAXVALUE 10"
    )
}
