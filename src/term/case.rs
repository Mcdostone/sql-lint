use nom::branch::alt;
use std::ops::Deref;

use crate::clause::Clause;
use crate::expression::parse_expression;
use crate::expression::Expression;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::term::parse_term;
use crate::term::Term;
use crate::ws::ws;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::many1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub enum CaseExpression {
    Simple(Case),
    Searched(SearchedCase),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Case(pub Box<Term>, pub Vec<When>, pub Option<Box<Else>>);

#[derive(Debug, PartialEq, Clone)]
pub struct SearchedCase(pub Box<When>, pub Option<Box<Else>>);

#[derive(Debug, PartialEq, Clone)]
pub struct When(pub Expression, pub Term);

#[derive(Debug, PartialEq, Clone)]
pub struct Else(pub Term);

impl Clause for Case {
    const KEYWORD: &'static Keyword = &Keyword::Case;
}

impl Clause for SearchedCase {
    const KEYWORD: &'static Keyword = &Keyword::Case;
}

pub fn when(input: &str) -> IResult<&str, When> {
    map(
        tuple((
            parse_keyword(Keyword::When),
            ws(parse_expression),
            parse_keyword(Keyword::Then),
            ws(parse_term),
        )),
        |(_, e, _, t)| When(e, t),
    )(input)
}

fn parse_else(input: &str) -> IResult<&str, Box<Else>> {
    map(
        tuple((parse_keyword(Keyword::Else), parse_term)),
        |(_, t)| Box::new(Else(t)),
    )(input)
}

pub fn parse_case_expression(input: &str) -> IResult<&str, CaseExpression> {
    alt((
        map(parse_case, CaseExpression::Simple),
        map(parse_searched_case, CaseExpression::Searched),
    ))(input)
}

pub fn parse_searched_case(input: &str) -> IResult<&str, SearchedCase> {
    map(
        tuple((
            parse_keyword(Keyword::Case),
            ws(when),
            opt(parse_else),
            parse_keyword(Keyword::End),
        )),
        |(_, w, e, __)| SearchedCase(Box::new(w), e),
    )(input)
}

pub fn parse_case(input: &str) -> IResult<&str, Case> {
    map(
        tuple((
            parse_keyword(Keyword::Case),
            parse_term,
            many1(when),
            opt(parse_else),
            parse_keyword(Keyword::End),
        )),
        |(_, t, w, e, __)| Case(Box::new(t), w, e),
    )(input)
}

impl Format for CaseExpression {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Simple(i) => i.format(f),
            Self::Searched(i) => i.format(f),
        }
    }
}

impl Format for Case {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(self.keyword());
        f.space().append_format(self.0.deref());
        for e in self.1.iter() {
            f.new_line().right_side(e);
        }
        match &self.2 {
            Some(i) => f.new_line().right_side(i.deref()),
            None => f,
        };
        f.new_line().right_side(&Keyword::End)
    }
}

impl Format for SearchedCase {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(self.keyword()).space();
        f.new_line().right_side(self.0.deref());
        match &self.1 {
            Some(i) => f.new_line().right_side(i.deref()),
            None => f,
        };
        f.new_line().right_side(&Keyword::End)
    }
}

impl Format for When {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&Keyword::When)
            .ws()
            .append_format(&self.0)
            .ws()
            .append_format(&Keyword::Then)
            .ws()
            .append_format(&self.1)
    }
}

impl Format for Else {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&Keyword::Else).ws().append_format(&self.0)
    }
}
