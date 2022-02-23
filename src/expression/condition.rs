use crate::character::parse_comma;
use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::clause::Clause;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::keyword::is_keyword;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::term::parse_term;
use crate::term::Term;
use crate::ws::ws;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::combinator::not;
use nom::combinator::peek;
use nom::combinator::value;
use nom::multi::many1;
use nom::multi::separated_list0;
use nom::sequence::delimited;
use nom::sequence::preceded;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub enum Condition {
    Operand(Operand),
    BinaryExpression(Operand, RightOperand),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Operand {
    Term(Term),
}

#[derive(Debug, PartialEq, Clone)]
pub enum RightOperand {
    Compare(Compare, Operand),
    In(InPredicateValue),
    Like(Operand),
    Between(Operand, Operand),
    Null(bool),
}

#[derive(Debug, PartialEq, Clone)]
pub enum InPredicateValue {
    Subquery(Operand),
    InValueList(List<Operand>),
}

impl Format for InPredicateValue {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Subquery(v) => f.append_format(v),
            Self::InValueList(v) => f.append_str("(").append_format(v).append_str(")"),
        }
    }
}

pub fn parse_in_predicate_value(input: &str) -> IResult<&str, InPredicateValue> {
    alt((
        map(operand, InPredicateValue::Subquery),
        map(
            delimited(
                parse_left_parenthesis,
                separated_list0(parse_comma, ws(operand)),
                parse_right_parenthesis,
            ),
            |o| InPredicateValue::InValueList(List(o)),
        ),
    ))(input)
}

pub fn right_operand(input: &str) -> IResult<&str, RightOperand> {
    alt((
        map(
            tuple((parse_keyword(Keyword::Like), ws(operand))),
            |(_, o)| RightOperand::Like(o),
        ),
        map(
            tuple((parse_keyword(Keyword::Is), parse_keyword(Keyword::Null))),
            |(_, _)| RightOperand::Null(true),
        ),
        map(
            tuple((
                parse_keyword(Keyword::Is),
                parse_keyword(Keyword::Not),
                parse_keyword(Keyword::Null),
            )),
            |(_, _, _)| RightOperand::Null(false),
        ),
        map(
            tuple((
                parse_keyword(Keyword::Between),
                ws(operand),
                parse_keyword(Keyword::And),
                ws(operand),
            )),
            |(_, l, _, r)| RightOperand::Between(l, r),
        ),
        map(
            tuple((parse_keyword(Keyword::In), parse_in_predicate_value)),
            |(_, e)| RightOperand::In(e),
        ),
        map(tuple((ws(compare), ws(operand))), |(c, o)| {
            RightOperand::Compare(c, o)
        }),
    ))(input)
}

impl Format for RightOperand {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::In(v) => f.append(&Keyword::In).ws().append_format(v),
            Self::Between(l, r) => f
                .append(&Keyword::Between)
                .ws()
                .append_format(l)
                .ws()
                .append(&Keyword::And)
                .ws()
                .append_format(r),
            Self::Like(l) => f.append(&Keyword::Like).ws().append_format(l),
            Self::Compare(op, r) => f.append(op).ws().append_format(r),
            Self::Null(true) => f.append(&Keyword::Is).ws().append(&Keyword::Null),
            Self::Null(false) => f
                .append(&Keyword::Is)
                .ws()
                .append(&Keyword::Not)
                .ws()
                .append(&Keyword::Null),
        }
    }
}

impl Format for Operand {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Operand::Term(term) => f.append_format(term),
        }
    }
}

pub fn operand(input: &str) -> IResult<&str, Operand> {
    let (input, _) = peek(not(is_keyword))(input)?;
    map(parse_term, Operand::Term)(input)
}

pub fn compare(input: &str) -> IResult<&str, Compare> {
    alt((
        value(Compare::GreaterOrEqual, tag(">=")),
        value(Compare::LowerOrEqual, tag("<=")),
        value(Compare::GreaterThan, tag(">")),
        value(Compare::LowerThan, tag("<")),
        value(Compare::Equal, tag("=")),
        value(Compare::NotEqual, tag("!=")),
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Compare {
    GreaterThan,
    GreaterOrEqual,
    LowerThan,
    LowerOrEqual,
    Equal,
    NotEqual,
}

impl fmt::Display for Compare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::GreaterOrEqual => write!(f, ">="),
            Self::GreaterThan => write!(f, ">"),
            Self::LowerThan => write!(f, "<"),
            Self::LowerOrEqual => write!(f, "<="),
            Self::Equal => write!(f, "="),
            Self::NotEqual => write!(f, "!="),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Condition(Condition),
    AndExpression(Condition),
    OrExpression(Condition),
}

pub fn condition(input: &str) -> IResult<&str, Condition> {
    //Operand, Compare, Operand
    alt((
        map(tuple((ws(operand), right_operand)), |(left, right)| {
            Condition::BinaryExpression(left, right)
        }),
        map(ws(operand), Condition::Operand),
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhereClause(pub List<Expression>);

impl Clause for WhereClause {
    const KEYWORD: &'static Keyword = &Keyword::Where;
}

impl Format for WhereClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_left_right(self.keyword(), &self.0)
    }
}

impl Format for Condition {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Operand(o) => f.append_format(o),
            Self::BinaryExpression(l, r) => f.append_format(l).ws().append_format(r),
        }
    }
}

impl Format for Expression {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Condition(c) => f.append_format(c),
            Self::AndExpression(c) => f.new_line().append_left_right(&Keyword::And, c),
            Self::OrExpression(c) => f.new_line().append_left_right(&Keyword::Or, c),
        }
    }
}

pub fn parse_expression(input: &str) -> IResult<&str, Expression> {
    alt((
        map(
            preceded(parse_keyword(Keyword::And), condition),
            Expression::AndExpression,
        ),
        map(
            preceded(parse_keyword(Keyword::Or), condition),
            Expression::OrExpression,
        ),
        map(ws(condition), Expression::Condition),
    ))(input)
}

pub fn parse_expressions(input: &str) -> IResult<&str, List<Expression>> {
    map(many1(parse_expression), List)(input)
}

pub fn parse_where_clause(input: &str) -> IResult<&str, WhereClause> {
    let (input, _) = WhereClause::parse_keyword(input)?;
    let (input, expressions) = parse_expressions(input)?;
    Ok((input, WhereClause(List(expressions.0))))
}

impl Format for List<Operand> {
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

/*
impl fmt::Display for List<Expression> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0
                .iter()
                .map(|e| e.to_string())
                .collect::<Vec<String>>()
                .join(", ")
        )
    }
}*/

impl Format for List<Expression> {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        for i in self.0.iter() {
            f.append_format(i);
        }
        f
    }
}
