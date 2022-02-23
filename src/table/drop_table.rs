use crate::clause::Clause;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::table::create::parse_table_ref;
use crate::table::create::TableRef;
use crate::ws::ws;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::value;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct DropTable(pub TableRef, pub Option<DropBehavior>);

pub fn parse_drop_table(input: &str) -> IResult<&str, DropTable> {
    map(
        tuple((
            parse_keyword(Keyword::Drop),
            parse_keyword(Keyword::Table),
            ws(parse_table_ref),
            opt(parse_drop_behavior),
        )),
        |(_, _, t, d)| DropTable(t, d),
    )(input)
}

pub fn parse_drop_behavior(input: &str) -> IResult<&str, DropBehavior> {
    alt((
        value(DropBehavior::Cascade, parse_keyword(Keyword::Cascade)),
        value(DropBehavior::Restrict, parse_keyword(Keyword::Restrict)),
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub enum DropBehavior {
    Cascade,
    Restrict,
}

impl fmt::Display for DropBehavior {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Cascade => write!(f, "{}", Keyword::Cascade),
            Self::Restrict => write!(f, "{}", Keyword::Restrict),
        }
    }
}

impl Clause for DropTable {
    const KEYWORD: &'static Keyword = &Keyword::DropTable;
}

impl Format for DropTable {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&Keyword::Drop)
            .ws()
            .append(&Keyword::Table)
            .ws()
            .append(&self.0);
        match &self.1 {
            Some(o) => f.ws().append(o),
            None => f,
        }
    }
}
