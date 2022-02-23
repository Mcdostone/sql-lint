use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::identifier::parse_name;
use crate::identifier::parse_schema_qualified_name;
use crate::identifier::Name;
use crate::identifier::SchemaQualifiedName;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::ws::ws;
use nom::branch::alt;
use nom::character::complete::digit1;
use nom::character::complete::u8;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::multi::many0;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct Sequence(pub SchemaQualifiedName, pub Vec<SequenceGeneratorOption>);

#[derive(Debug, PartialEq, Clone)]
pub struct AlterSequence(pub Sequence);

#[derive(Debug, PartialEq, Clone)]
pub struct DropSequence(pub Name);

#[derive(Debug, PartialEq, Clone)]
pub enum SequenceGeneratorOption {
    Startwith(u8),
    IncrementBy(u8),
    NoMinValue,
    NoMaxValue,
    MinValue(u8),
    MaxValue(u8),
    Cache(u8),
}

pub fn sequence_generator_option(input: &str) -> IResult<&str, SequenceGeneratorOption> {
    alt((
        map(
            tuple((
                pair(parse_keyword(Keyword::Start), parse_keyword(Keyword::With)),
                pp,
            )),
            |(_, i)| SequenceGeneratorOption::Startwith(i),
        ),
        map(
            tuple((
                pair(
                    parse_keyword(Keyword::Increment),
                    parse_keyword(Keyword::By),
                ),
                pp,
            )),
            |(_, i)| SequenceGeneratorOption::IncrementBy(i),
        ),
        map(tuple((parse_keyword(Keyword::Cache), pp)), |(_, i)| {
            SequenceGeneratorOption::Cache(i)
        }),
        map(
            pair(parse_keyword(Keyword::No), parse_keyword(Keyword::Minvalue)),
            |_| SequenceGeneratorOption::NoMinValue,
        ),
        map(pair(parse_keyword(Keyword::Minvalue), ws(u8)), |(_, i)| {
            SequenceGeneratorOption::MinValue(i)
        }),
        map(
            pair(parse_keyword(Keyword::No), parse_keyword(Keyword::Maxvalue)),
            |(_, _i)| SequenceGeneratorOption::NoMaxValue,
        ),
        map(pair(parse_keyword(Keyword::Maxvalue), ws(u8)), |(_, i)| {
            SequenceGeneratorOption::MaxValue(i)
        }),
    ))(input)
}

fn pp(input: &str) -> IResult<&str, u8> {
    map_res(recognize(digit1), str::parse)(input)
}

impl Format for SequenceGeneratorOption {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Startwith(i) => f.append(&format!("START WITH {i}")),
            Self::IncrementBy(i) => f.append(&format!("INCREMENT BY {i}")),
            Self::NoMinValue => f.append(&"NO MINVALUE".to_string()),
            Self::NoMaxValue => f.append(&"NO MAXVALUE".to_string()),
            Self::MinValue(i) => f.append(&format!("MINVALUE {i}")),
            Self::MaxValue(i) => f.append(&format!("MAXVALUE {i}")),
            Self::Cache(i) => f.append(&format!("CACHE {i}")),
        }
    }
}

impl Format for Sequence {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&Keyword::Create)
            .ws()
            .append(&Keyword::Sequence)
            .ws()
            .append(&self.0);
        f.set_pad(4);
        for i in self.1.iter() {
            f.new_line().indent(i);
        }
        f
    }
}

impl fmt::Display for DropSequence {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "DROP SEQUENCE {}", self.0)
    }
}

pub fn parse_sequence(input: &str) -> IResult<&str, Sequence> {
    map(
        tuple((
            parse_keyword(Keyword::Create),
            parse_keyword(Keyword::Sequence),
            parse_schema_qualified_name,
            many0(ws(sequence_generator_option)),
        )),
        |(_, _, n, o)| Sequence(n, o),
    )(input)
}

pub fn parse_alter_sequence(input: &str) -> IResult<&str, AlterSequence> {
    map(
        tuple((
            parse_keyword(Keyword::Alter),
            parse_keyword(Keyword::Sequence),
            parse_schema_qualified_name,
            many0(ws(sequence_generator_option)),
        )),
        |(_, _, n, o)| AlterSequence(Sequence(n, o)),
    )(input)
}

pub fn parse_drop_sequence(input: &str) -> IResult<&str, DropSequence> {
    map(
        tuple((
            parse_keyword(Keyword::Drop),
            parse_keyword(Keyword::Sequence),
            parse_name,
        )),
        |(_, _, n)| DropSequence(n),
    )(input)
}

#[cfg(test)]
mod tests;
