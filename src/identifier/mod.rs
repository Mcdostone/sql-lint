use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::list::List;
use nom::branch::alt;
use nom::character::complete::alphanumeric1;
use nom::character::complete::char;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::sequence::delimited;
use nom::sequence::separated_pair;
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Name {
    Name(String),
    QuotedName(String),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SchemaQualifiedName(pub Option<Name>, pub Name);

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Name(name) => write!(f, "{}", name),
            Self::QuotedName(name) => write!(f, "'{}'", name),
        }
    }
}

pub fn parse_name(input: &str) -> IResult<&str, Name> {
    alt((
        map(delimited(char('"'), alphanumeric1, char('"')), |s: &str| {
            Name::QuotedName(s.to_string())
        }),
        map(
            delimited(char('\''), alphanumeric1, char('\'')),
            |s: &str| Name::QuotedName(s.to_string()),
        ),
        map(
            recognize(many1(alt((alphanumeric1, recognize(one_of("_")))))),
            |s: &str| Name::Name(s.to_string()),
        ),
    ))(input)
}

pub fn parse_schema_qualified_name(input: &str) -> IResult<&str, SchemaQualifiedName> {
    alt((
        map(
            separated_pair(opt(parse_name), char('.'), parse_name),
            |(l, r)| SchemaQualifiedName(l, r),
        ),
        map(parse_name, |n| SchemaQualifiedName(None, n)),
    ))(input)
}

impl fmt::Display for SchemaQualifiedName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(name) => write!(f, "{}.{}", name, self.1),
            None => write!(f, "'{}'", self.1),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Delimitedidentifier {
    Name(Name),
    Quoted(Name),
}

pub fn parse_delimited_identifier(input: &str) -> IResult<&str, Delimitedidentifier> {
    alt((
        map(delimited(char('"'), parse_name, char('"')), |n| {
            Delimitedidentifier::Quoted(n)
        }),
        map(parse_name, Delimitedidentifier::Name),
    ))(input)
}

impl Format for Delimitedidentifier {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Name(n) => f.append_format(n),
            Self::Quoted(n) => f.append_str("\"").append_format(n).append_str("\""),
        }
    }
}

impl fmt::Display for List<Name> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}

#[cfg(test)]
mod tests;
