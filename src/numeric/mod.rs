use nom::branch::alt;
use nom::character::complete::i32;
use nom::combinator::map;
use nom::number::complete::double;
use nom::number::complete::float;

use nom::IResult;
use std::fmt;
use std::str;

#[derive(Debug, PartialEq, Clone)]
pub enum Numeric {
    Int(i32),
    Float(f32),
    Decimal(f64),
}

pub fn parse_numeric(input: &str) -> IResult<&str, Numeric> {
    alt((
        map(i32, Numeric::Int),
        map(float, Numeric::Float),
        map(double, Numeric::Decimal),
    ))(input)
}

impl fmt::Display for Numeric {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Float(i) => write!(f, "{i}"),
            Self::Decimal(i) => write!(f, "{i}"),
            Self::Int(i) => write!(f, "{i}"),
        }
    }
}

//#[cfg(test)]
//mod tests;
