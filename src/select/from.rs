use crate::character::parse_comma;
use crate::clause::Clause;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::identifier::parse_name;
use crate::identifier::Name;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::select::join::parse_join_clause;
use crate::select::join::parse_joins_clause;
use crate::select::join::JoinClause;

use crate::ws::ws;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct FromClause(pub List<TableExpression>, pub Option<List<JoinClause>>);

#[derive(Debug, PartialEq, Clone)]
pub enum TableName {
    Name(Name),
    AliasedName(Name, Name),
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableExpression(pub TableName);

impl fmt::Display for TableName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Name(name) => write!(f, "{}", name),
            Self::AliasedName(name, alias) => write!(f, "{} {} {}", name, Keyword::As, alias),
        }
    }
}

impl Format for TableExpression {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&self.0)
    }
}

impl Format for FromClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.left_side(self.keyword()).space().append_format(&self.0);
        match &self.1 {
            Some(i) => f.append_format(i),
            None => f,
        }
    }
}

impl Clause for FromClause {
    const KEYWORD: &'static Keyword = &Keyword::From;
}

pub fn parse_from_clause(input: &str) -> IResult<&str, FromClause> {
    map(
        tuple((
            FromClause::parse_keyword,
            separated_list1(parse_comma, ws(table_expression)),
            opt(parse_joins_clause),
        )),
        |(_, t, j)| FromClause(List(t), j),
    )(input)
}

pub fn table_expression(input: &str) -> IResult<&str, TableExpression> {
    map(table_name, TableExpression)(input)
}

pub fn table_name(input: &str) -> IResult<&str, TableName> {
    alt((
        map(
            tuple((parse_name, opt(parse_keyword(Keyword::As)), ws(parse_name))),
            |(n, _, a)| TableName::AliasedName(n, a),
        ),
        map(tuple((parse_name, ws(opt(parse_join_clause)))), |(n, _)| {
            TableName::Name(n)
        }),
    ))(input)
}

impl Format for List<JoinClause> {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        for (pos, j) in self.0.iter().enumerate() {
            match pos {
                0 => f.append_clause(j),
                _ => f.new_line().append_clause(j),
            };
        }
        f
    }
}

impl Format for List<TableExpression> {
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
