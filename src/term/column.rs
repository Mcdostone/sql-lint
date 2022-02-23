use crate::identifier::Name;
use nom::branch::alt;
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum ColumnRef {
    Name(Name),
    WithFamily(Name, Name),
}

impl fmt::Display for ColumnRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Name(name) => write!(f, "{}", name),
            Self::WithFamily(family, name) => write!(f, "{}.{}", family, name),
        }
    }
}

pub fn parse_column_ref(input: &str) -> IResult<&str, ColumnRef> {
    alt((
        map(
            separated_pair(parse_name, char('.'), parse_name),
            |(family, name)| ColumnRef::WithFamily(family, name),
        ),
        map(parse_name, ColumnRef::Name),
    ))(input)
}

pub fn parse_name(input: &str) -> IResult<&str, Name> {
    alt((
        map(delimited(char('"'), alphanumeric1, char('"')), |s: &str| {
            Name::QuotedName(s.to_string())
        }),
        map(
            recognize(many1(alt((alphanumeric1, recognize(one_of("_")))))),
            |s: &str| Name::Name(s.to_string()),
        ),
    ))(input)
}
