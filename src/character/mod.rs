use crate::ws::ws;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

pub fn parse_left_parenthesis(input: &str) -> IResult<&str, ()> {
    map(ws(tag("(")), |_| ())(input)
}

pub fn parse_right_parenthesis(input: &str) -> IResult<&str, ()> {
    map(ws(tag(")")), |_| ())(input)
}

pub fn parse_comma(input: &str) -> IResult<&str, ()> {
    map(ws(tag(",")), |_| ())(input)
}
