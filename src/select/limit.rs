use crate::clause::Clause;

use crate::keyword::Keyword;
use crate::term::bind_parameter::parse_bind_parameter;
use crate::term::bind_parameter::BindParameter;
use nom::branch::alt;
use nom::character::complete::u16;
use nom::combinator::map;
use nom::sequence::pair;
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum LimitClause {
    Number(u16),
    BindParameter(BindParameter),
}

pub fn parse_limit_clause(input: &str) -> IResult<&str, LimitClause> {
    map(
        pair(
            LimitClause::parse_keyword,
            alt((
                map(parse_bind_parameter, LimitClause::BindParameter),
                map(u16, LimitClause::Number),
            )),
        ),
        |(_, s)| s,
    )(input)
}

impl fmt::Display for LimitClause {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Number(n) => write!(f, "{} {n}", Self::KEYWORD),
            Self::BindParameter(b) => write!(f, "{} {b}", Self::KEYWORD),
        }
    }
}

impl Clause for LimitClause {
    const KEYWORD: &'static Keyword = &Keyword::Limit;
}
