use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::select::parse_select_statement;
use crate::select::SelectStatement;
use crate::ws::ws;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::value;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub enum TableOperator {
    Union(bool),
    Intersect,
    Minus,
    Except,
}

#[derive(Debug, PartialEq, Clone)]
pub enum QueryTerm {
    Select(Box<SelectStatement>),
    Parenthesis(Box<QueryTerm>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct CombinedTables(pub QueryTerm, pub TableOperator, pub QueryTerm);

impl Format for TableOperator {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Union(true) => f.left_side(&Keyword::Union).ws().append(&Keyword::All),
            Self::Union(false) => f.left_side(&Keyword::Union),
            Self::Intersect => f.left_side(&Keyword::Intersect),
            Self::Minus => f.left_side(&Keyword::Minus),
            Self::Except => f.left_side(&Keyword::Except),
        }
    }

    fn side(&self) -> usize {
        match self {
            Self::Intersect => 9,
            Self::Except => 6,
            _ => 5,
        }
    }
}

impl Format for CombinedTables {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(&self.0)
            .new_line()
            .new_line()
            .append_format(&self.1)
            .new_line()
            .new_line()
            .append_format(&self.2)
    }
}

impl Format for QueryTerm {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Select(s) => f.append_format(s.deref()),
            Self::Parenthesis(s) => {
                f.set_pad(f.pad + 1);
                f.append_str("(").append_format(s.deref()).append_str(")");
                f.set_pad(f.pad - 1);
                f
            }
        }
    }
}

pub fn parse_query_term(input: &str) -> IResult<&str, QueryTerm> {
    alt((
        map(ws(parse_select_statement), |s| {
            QueryTerm::Select(Box::new(s))
        }),
        map(
            delimited(
                parse_left_parenthesis,
                parse_query_term,
                parse_right_parenthesis,
            ),
            |s| QueryTerm::Parenthesis(Box::new(s)),
        ),
    ))(input)
}

pub fn combined_tables(input: &str) -> IResult<&str, CombinedTables> {
    map(
        tuple((
            ws(parse_query_term),
            ws(table_operator),
            ws(parse_query_term),
        )),
        |(l, o, r)| CombinedTables(l, o, r),
    )(input)
}

fn table_operator_all(input: &str) -> IResult<&str, bool> {
    alt((
        map(parse_keyword(Keyword::All), |_| true),
        map(not(parse_keyword(Keyword::All)), |_| false),
    ))(input)
}

fn table_operator(input: &str) -> IResult<&str, TableOperator> {
    alt((
        map(
            tuple((parse_keyword(Keyword::Union), table_operator_all)),
            |(_, all)| TableOperator::Union(all),
        ),
        value(TableOperator::Intersect, parse_keyword(Keyword::Intersect)),
        value(TableOperator::Minus, parse_keyword(Keyword::Minus)),
        value(TableOperator::Except, parse_keyword(Keyword::Except)),
    ))(input)
}
