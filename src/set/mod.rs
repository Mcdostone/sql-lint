use crate::clause::Clause;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::identifier::parse_name;
use crate::identifier::Name;
use crate::keyword::Keyword;
use crate::term::value::parse_value;
use crate::term::value::Value;
use crate::ws::ws;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct SetStatement(pub Name, pub Value);

impl Clause for SetStatement {
    const KEYWORD: &'static Keyword = &Keyword::Set;
}

pub fn parse_set_statement(input: &str) -> IResult<&str, SetStatement> {
    map(
        tuple((
            SetStatement::parse_keyword,
            parse_name,
            ws(tag("=")),
            ws(parse_value),
        )),
        |(_, n, _, v)| SetStatement(n, v),
    )(input)
}

impl Format for SetStatement {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(self.keyword())
            .ws()
            .append_format(&self.0)
            .ws()
            .append_str("=")
            .ws()
            .append_format(&self.1)
    }
}

#[cfg(test)]
mod tests;
