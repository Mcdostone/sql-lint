#[cfg(test)]
mod tests;

use crate::character::parse_comma;
use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::formatter::Formatter;
use crate::list::List;
use crate::term::parse_term;
use crate::term::Term;
use crate::ws::ws;
use crate::Format;
use nom::branch::alt;
use nom::character::complete::alphanumeric1;
use nom::character::complete::one_of;
use nom::combinator::map;
use nom::combinator::recognize;
use nom::multi::many1;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct Function(pub String, pub List<Term>);

impl Format for Function {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(&self.0)
            .append_str("(")
            .append_format(&self.1)
            .append_str(")")
    }
}

fn function_name(input: &str) -> IResult<&str, String> {
    map(
        recognize(many1(alt((alphanumeric1, recognize(one_of("_")))))),
        |s: &str| s.to_string(),
    )(input)
}

pub fn parse_function(input: &str) -> IResult<&str, Function> {
    map(
        tuple((
            function_name,
            delimited(
                parse_left_parenthesis,
                map(separated_list0(parse_comma, ws(parse_term)), List),
                parse_right_parenthesis,
            ),
        )),
        |(n, terms)| Function(n, terms),
    )(input)
}
