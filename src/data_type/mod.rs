use crate::character::parse_comma;
use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::term::value::parse_value;
use crate::term::value::Value;
use crate::ws::ws;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::character::complete::u8;

use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::opt;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;

use std::str::FromStr;

#[derive(Debug, EnumString, IntoStaticStr, Clone, Copy, PartialEq)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum PredefinedType {
    Character,
    Char,
    CharacterVarying,
    CharVarying,
    Varchar,
    Binary,
    Serial, // Not ANSI
    BinaryVarying,
    Varbinary,
    BinaryLargeObject,
    Blob,
    DateTime(DateTimeType),
    Numeric,
    Decimal,
    Dec,
    Float,
    Date,
    Time,
    Timestamp,
    Smallint,
    Integer,
    Boolean,
    Int,
    Json, // Not ANSI
    Bigint,
    Enum,
}

#[derive(Debug, EnumString, IntoStaticStr, Clone, Copy, PartialEq)]
#[strum(ascii_case_insensitive, serialize_all = "title_case")]
pub enum DateTimeType {
    Date,
    Time(DateTimeOption),
    Timestamp(DateTimeOption),
}

impl std::default::Default for DateTimeType {
    fn default() -> Self {
        Self::Date
    }
}

pub fn parse_with_or_without_timezone(input: &str) -> IResult<&str, Option<bool>> {
    opt(alt((
        value(
            true,
            tuple((
                parse_keyword(Keyword::With),
                parse_keyword(Keyword::Time),
                parse_keyword(Keyword::Zone),
            )),
        ),
        value(
            false,
            tuple((
                parse_keyword(Keyword::Without),
                parse_keyword(Keyword::Time),
                parse_keyword(Keyword::Zone),
            )),
        ),
    )))(input)
}

pub fn parse_time_precision(input: &str) -> IResult<&str, Option<u8>> {
    opt(delimited(
        parse_left_parenthesis,
        ws(u8),
        parse_right_parenthesis,
    ))(input)
}

pub fn parse_datetime_type(input: &str) -> IResult<&str, DateTimeType> {
    alt((
        value(DateTimeType::Date, parse_keyword(Keyword::Date)),
        map(
            tuple((
                parse_keyword(Keyword::Timestamp),
                parse_time_precision,
                parse_with_or_without_timezone,
            )),
            |(_, p, t)| DateTimeType::Timestamp(DateTimeOption(p, t)),
        ),
        map(
            tuple((
                parse_keyword(Keyword::Time),
                parse_time_precision,
                parse_with_or_without_timezone,
            )),
            |(_, p, t)| DateTimeType::Time(DateTimeOption(p, t)),
        ),
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct DataType(pub PredefinedType, pub Option<List<Value>>);

pub fn parse_predefined_type(input: &str) -> IResult<&str, PredefinedType> {
    alt((
        map(parse_datetime_type, PredefinedType::DateTime),
        map(
            pair(
                ws(parse_keyword(Keyword::Character)),
                ws(parse_keyword(Keyword::Varying)),
            ),
            |(_, _)| PredefinedType::CharacterVarying,
        ),
        map_res(take_while1(|c: char| c.is_alphabetic()), |s: &str| {
            PredefinedType::from_str(s)
        }),
    ))(input)
}

pub fn parse_data_type(input: &str) -> IResult<&str, DataType> {
    map(
        pair(
            ws(parse_predefined_type),
            opt(map(
                delimited(
                    parse_left_parenthesis,
                    separated_list1(parse_comma, parse_value),
                    parse_right_parenthesis,
                ),
                List,
            )),
        ),
        |(t, o)| DataType(t, o),
    )(input)
}

impl Format for PredefinedType {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::DateTime(d) => f.append_format(d),
            _ => {
                let t: &'static str = self.into();
                f.append(&t.to_uppercase())
            }
        }
    }
}

impl Format for DateTimeType {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Date => f.append(&Keyword::Date),
            Self::Timestamp(o) => f.append(&Keyword::Timestamp).append_format(o),
            Self::Time(o) => f.append(&Keyword::Time).append_format(o),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct DateTimeOption(Option<u8>, Option<bool>);

impl Format for DateTimeOption {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match &self.0 {
            Some(p) => f.append_str("(").append(p).append_str(")"),
            None => f,
        };
        match &self.1 {
            Some(true) => f
                .ws()
                .append(&Keyword::With)
                .ws()
                .append(&Keyword::Time)
                .ws()
                .append(&Keyword::Zone),
            Some(false) => f
                .ws()
                .append(&Keyword::Without)
                .ws()
                .append(&Keyword::Time)
                .ws()
                .append(&Keyword::Zone),
            None => f,
        }
    }
}

impl Format for DataType {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(&self.0);
        match &self.1 {
            Some(s) => f.append_str("(").append_format(s).append_str(")"),
            None => f,
        }
    }
}

#[cfg(test)]
mod tests;
