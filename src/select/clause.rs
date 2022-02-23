use crate::character::parse_comma;
use crate::clause::Clause;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::identifier::parse_name;
use crate::identifier::Name;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::term::{parse_term, Term};
use crate::ws::ws;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::value;
use nom::combinator::value as nomValue;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct SelectClause(pub Option<SetQuantifier>, pub List<SelectedExpression>);

#[derive(Debug, PartialEq, Clone)]
pub enum SelectedExpression {
    All,
    AllWithFamilyName(Name),
    Term(Term),
}

#[derive(Debug, PartialEq, Clone)]
pub enum SetQuantifier {
    Distinct,
    All,
}

impl Clause for SelectClause {
    const KEYWORD: &'static Keyword = &Keyword::Select;
}

impl Format for SelectedExpression {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::All => f.append(&"*"),
            Self::Term(term) => f.append_format(term),
            Self::AllWithFamilyName(n) => f.append(&format!("{n}.*")),
        }
    }
}

impl Format for SetQuantifier {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::All => f.append(&Keyword::All),
            Self::Distinct => f.append(&Keyword::Distinct),
        }
    }
}

impl Format for SelectClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(self.keyword()).ws();
        match &self.0 {
            Some(q) => f.append_format(q).ws(),
            None => f,
        };
        f.append_format(&self.1)
    }
}

pub fn parse_set_quantifier(input: &str) -> IResult<&str, SetQuantifier> {
    alt((
        value(SetQuantifier::All, parse_keyword(Keyword::All)),
        value(SetQuantifier::Distinct, parse_keyword(Keyword::Distinct)),
    ))(input)
}

pub fn parse_select_clause(input: &str) -> IResult<&str, SelectClause> {
    let (input, _) = SelectClause::parse_keyword(input)?;
    map(
        pair(
            opt(parse_set_quantifier),
            separated_list1(parse_comma, ws(parse_selected_expression)),
        ),
        |(q, v)| SelectClause(q, List(v)),
    )(input)
}

fn parse_selected_expression(input: &str) -> IResult<&str, SelectedExpression> {
    alt((
        nomValue(SelectedExpression::All, ws(tag("*"))),
        map(tuple((parse_name, char('.'), char('*'))), |(n, _, _)| {
            SelectedExpression::AllWithFamilyName(n)
        }),
        map(ws(parse_term), SelectedExpression::Term),
    ))(input)
}

impl Format for List<SelectedExpression> {
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
