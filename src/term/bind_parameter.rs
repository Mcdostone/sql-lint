use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::digit1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::recognize;
use nom::combinator::value;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum BindParameter {
    Placeholder,
    Index(u8),
}

impl fmt::Display for BindParameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Placeholder => write!(f, "?"),
            Self::Index(i) => write!(f, ":{i}"),
        }
    }
}

pub fn parse_bind_parameter(input: &str) -> IResult<&str, BindParameter> {
    alt((
        value(BindParameter::Placeholder, tag("?")),
        map(
            tuple((tag(":"), map_res(recognize(digit1), str::parse))),
            |(_, i)| BindParameter::Index(i),
        ),
    ))(input)
}
