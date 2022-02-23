use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::list::List;
use crate::numeric::{parse_numeric, Numeric};
use nom::branch::alt;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_until;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::value as nomValue;
use nom::sequence::delimited;
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    Null,
    Num(Numeric),
    Bool(bool),
    String(String),
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Value::Null => write!(f, "NULL"),
            Value::Bool(b) => write!(f, "{b}"),
            Value::String(s) => write!(f, "'{s}'"),
            Value::Num(numeric) => write!(f, "{numeric}"),
        }
    }
}

pub fn parse_value(input: &str) -> IResult<&str, Value> {
    use Value::*;
    alt((
        nomValue(Null, tag_no_case("null")),
        map(parse_numeric, Num),
        map(string, String),
        map(boolean, Bool),
    ))(input)
}

fn boolean(input: &str) -> IResult<&str, bool> {
    alt((
        nomValue(false, tag_no_case("false")),
        nomValue(true, tag_no_case("true")),
    ))(input)
}

fn string_delimiter(input: &str) -> IResult<&str, char> {
    alt((char('"'), char('\'')))(input)
}

fn string(input: &str) -> IResult<&str, String> {
    let build_string = alt((take_until("\""), take_until("\'")));
    map(
        delimited(string_delimiter, build_string, string_delimiter),
        String::from,
    )(input)
}

impl Format for List<Value> {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        for (pos, i) in self.0.iter().enumerate() {
            match pos {
                0 => f.append_format(i),
                _ => f.append_str(", ").append_format(i),
            };
        }
        f
    }
}
