use self::bind_parameter::BindParameter;
use self::case::Case;
use self::column::ColumnRef;
use self::value::Value;
use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::formatter::Format;
use crate::formatter::Formatter;
use std::ops::Deref;

use crate::function::parse_function;
use crate::function::Function;
use crate::identifier::parse_name;
use crate::identifier::Name;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::select::parse_select_statement;
use crate::select::SelectStatement;
use crate::term::bind_parameter::parse_bind_parameter;
use crate::term::case::parse_case;
use crate::term::column::parse_column_ref;
use crate::term::value::parse_value;
use crate::ws::ws;
use nom::branch::alt;
use nom::combinator::map;
use nom::sequence::delimited;
use nom::sequence::tuple;
use nom::IResult;

pub mod bind_parameter;
pub mod case;
pub mod column;
pub mod value;

#[derive(Debug, PartialEq, Clone)]
pub enum Term {
    Value(Value),
    Case(Case),
    BindParameter(BindParameter),
    ColumnRef(ColumnRef),
    Function(Function),
    AliasedTerm(Box<Term>, Name),
    Subquery(Box<SelectStatement>),
}

/*
impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Value(value) => write!(f, "{value}"),
            Term::Case(case) => write!(f, "{}", case.output()),
            Term::BindParameter(parameter) => write!(f, "{parameter}"),
            Term::ColumnRef(column) => write!(f, "{column}"),
            Term::Function(fu) => write!(f, "{}", fu.output()),
            Term::AliasedTerm(n, a) => write!(f, "{n} AS {a}"),
            Self::Subquery(s) => write!(f, "({})", s.output()),
        }
    }
}*/

impl Format for Term {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::ColumnRef(column) => f.append_format(column),
            Self::Value(value) => f.append_format(value),
            Self::Case(case) => f.append_format(case),
            Self::BindParameter(parameter) => f.append_format(parameter),
            Self::Function(fu) => f.append_format(fu),
            Self::AliasedTerm(n, a) => f
                .append_format(n.deref())
                .space()
                .append(&Keyword::As)
                .space()
                .append(a),
            Self::Subquery(s) => f
                .new_line()
                .new_context()
                .right_side(&"(".to_string())
                .set_offset(2)
                .append_format(s.deref())
                .append_str(")")
                .set_offset(0)
                .pop_context(),
        }
    }
}

pub fn parse_term(input: &str) -> IResult<&str, Term> {
    alt((
        map(
            tuple((ws(term), parse_keyword(Keyword::As), ws(parse_name))),
            |(t, _, a)| Term::AliasedTerm(Box::new(t), a),
        ),
        term,
    ))(input)
}

fn term(input: &str) -> IResult<&str, Term> {
    alt((
        map(parse_value, Term::Value),
        map(parse_case, Term::Case),
        map(parse_function, Term::Function),
        map(parse_column_ref, Term::ColumnRef),
        map(parse_bind_parameter, Term::BindParameter),
        map(
            delimited(
                parse_left_parenthesis,
                parse_select_statement,
                parse_right_parenthesis,
            ),
            |s| Term::Subquery(Box::new(s)),
        ),
    ))(input)
}

impl Format for List<Term> {
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
impl fmt::Display for List<Term> {
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

#[cfg(test)]
mod bind_parameter_tests;
#[cfg(test)]
mod case_tests;
#[cfg(test)]
mod column_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod value_tests;
