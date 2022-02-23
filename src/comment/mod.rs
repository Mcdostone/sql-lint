use nom::bytes::complete::tag;
use nom::bytes::complete::take_until;
use nom::combinator::value;

use nom::sequence::tuple;
use nom::IResult;

pub fn parse_multiline_comment(input: &str) -> IResult<&str, ()> {
    value((), tuple((tag("(*"), take_until("*/"), tag("*/"))))(input)
}

/*pub fn parse_inline_comment(input: &str) -> IResult<&str, ()> {
    value((), tuple((tag("--"), take_while(is_newline))))(input)
}*/

//#[cfg(test)]
//mod tests;
