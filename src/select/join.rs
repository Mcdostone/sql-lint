use crate::character::parse_comma;
use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::clause::Clause;
use crate::expression::parse_expression;
use crate::expression::parse_expressions;
use crate::expression::Expression;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::select::from::table_expression;
use crate::select::from::TableExpression;
use crate::ws::ws;
use nom::combinator::opt;
use nom::multi::many1;

use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::peek;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;
use std::ops::Deref;

#[derive(Debug, PartialEq, Clone)]
pub struct JoinClause(
    pub JoinType,
    pub Box<TableExpression>,
    pub Option<JoinSpecification>,
);

#[derive(Debug, PartialEq, Clone)]
pub enum OuterJoinType {
    Left,
    Right,
    Full,
}

#[derive(Debug, PartialEq, Clone)]
pub struct OuterJoin(pub OuterJoinType, pub bool);

#[derive(Debug, PartialEq, Clone)]
pub enum JoinType {
    Default,
    Inner,
    QualifedJoin(OuterJoin),
    Cross,
    Natural,
}

impl fmt::Display for JoinType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Default => write!(f, ""),
            Self::Inner => write!(f, "{}", Keyword::Inner),
            Self::Cross => write!(f, "{}", Keyword::Cross),
            Self::Natural => write!(f, "{}", Keyword::Natural),
            Self::QualifedJoin(o) => write!(f, "{o}"),
        }
    }
}

impl fmt::Display for OuterJoinType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Left => write!(f, "{}", Keyword::Left),
            Self::Right => write!(f, "{}", Keyword::Right),
            Self::Full => write!(f, "{}", Keyword::Full),
        }
    }
}

impl fmt::Display for OuterJoin {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.1 {
            true => write!(f, "{} {}", self.0, Keyword::Outer),
            false => write!(f, "{}", self.0),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum JoinSpecification {
    On(List<Expression>),
    Using(List<Expression>),
}

impl Format for JoinClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self.0 {
            JoinType::Default => f.left_side(&Keyword::Join),
            _ => f.right_side(&self.0).space().append(&Keyword::Join),
        };
        f.space().append_format(self.1.deref());
        match &self.2 {
            Some(s) => f.append_clause(s),
            None => f,
        }
    }
}

impl Clause for JoinClause {
    const KEYWORD: &'static Keyword = &Keyword::Join;
}

impl Format for JoinSpecification {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match &self {
            Self::On(e) => {
                f.right_side(&Keyword::On).space();
                f.set_pad(f.pad + 7);
                f.append_format(e);
                f.set_pad(f.pad - 7)
            }
            Self::Using(e) => f.right_side(&Keyword::Using).space().append_format(e),
        }
    }
}

pub fn join_type_outer(input: &str) -> IResult<&str, bool> {
    alt((
        map(parse_keyword(Keyword::Outer), |_| true),
        map(not(parse_keyword(Keyword::Outer)), |_| false),
    ))(input)
}

pub fn outer_join(input: &str) -> IResult<&str, OuterJoin> {
    map(tuple((outer_join_type, ws(join_type_outer))), |(t, o)| {
        OuterJoin(t, o)
    })(input)
}

pub fn outer_join_type(input: &str) -> IResult<&str, OuterJoinType> {
    alt((
        value(OuterJoinType::Left, parse_keyword(Keyword::Left)),
        value(OuterJoinType::Right, parse_keyword(Keyword::Right)),
        value(OuterJoinType::Full, parse_keyword(Keyword::Full)),
    ))(input)
}

pub fn join_type(input: &str) -> IResult<&str, JoinType> {
    alt((
        value(JoinType::Natural, parse_keyword(Keyword::Natural)),
        value(JoinType::Cross, parse_keyword(Keyword::Cross)),
        value(JoinType::Inner, parse_keyword(Keyword::Inner)),
        map(outer_join, JoinType::QualifedJoin),
        map(peek(parse_keyword(Keyword::Join)), |_| JoinType::Default),
    ))(input)
}

pub fn join_specification(input: &str) -> IResult<&str, JoinSpecification> {
    alt((
        map(
            tuple((parse_keyword(Keyword::On), ws(parse_expressions))),
            |(_, expr)| JoinSpecification::On(expr),
        ),
        map(
            tuple((
                parse_keyword(Keyword::Using),
                delimited(
                    parse_left_parenthesis,
                    map(separated_list1(parse_comma, ws(parse_expression)), List),
                    parse_right_parenthesis,
                ),
            )),
            |(_, expr)| JoinSpecification::Using(expr),
        ),
    ))(input)
}

pub fn parse_join_clause(input: &str) -> IResult<&str, JoinClause> {
    map(
        tuple((
            ws(join_type),
            parse_keyword(Keyword::Join),
            ws(table_expression),
            opt(ws(join_specification)),
        )),
        |(t, _, table, spec)| JoinClause(t, Box::new(table), spec),
    )(input)
}

pub fn parse_joins_clause(input: &str) -> IResult<&str, List<JoinClause>> {
    map(many1(parse_join_clause), List)(input)
}
