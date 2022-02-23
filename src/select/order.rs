use crate::character::parse_comma;
use crate::clause::Clause;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::term::column::parse_column_ref;
use crate::term::column::ColumnRef;
use crate::ws::ws;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct OrderByClause(pub List<Order>);

#[derive(Debug, PartialEq, Clone)]
pub struct Order(pub SortKey, pub Option<OrderSort>, pub Option<NullsSort>);

#[derive(Debug, PartialEq, Clone)]
pub enum OrderSort {
    Asc,
    Desc,
}

#[derive(Debug, PartialEq, Clone)]
pub enum SortKey {
    ColumnRef(ColumnRef),
}

#[derive(Debug, PartialEq, Clone)]
pub enum NullsSort {
    First,
    Last,
}

fn null_sort(input: &str) -> IResult<&str, NullsSort> {
    let (input, _) = parse_keyword(Keyword::Nulls).parse(input)?;
    alt((
        value(NullsSort::First, parse_keyword(Keyword::First)),
        value(NullsSort::Last, parse_keyword(Keyword::Last)),
    ))(input)
}

fn order_sort(input: &str) -> IResult<&str, OrderSort> {
    alt((
        value(OrderSort::Asc, parse_keyword(Keyword::Asc)),
        value(OrderSort::Desc, parse_keyword(Keyword::Desc)),
    ))(input)
}

fn order(input: &str) -> IResult<&str, Order> {
    map(
        tuple((ws(parse_sort_key), ws(opt(order_sort)), ws(opt(null_sort)))),
        |(e, sort, nulls)| Order(e, sort, nulls),
    )(input)
}

pub fn parse_order_by_clause(input: &str) -> IResult<&str, OrderByClause> {
    map(
        tuple((
            OrderByClause::parse_keyword,
            parse_keyword(Keyword::By),
            ws(separated_list1(parse_comma, ws(order))),
        )),
        |(_, _, o)| OrderByClause(List(o)),
    )(input)
}

pub fn parse_sort_key(input: &str) -> IResult<&str, SortKey> {
    map(parse_column_ref, SortKey::ColumnRef)(input)
}

impl fmt::Display for OrderSort {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Asc => write!(f, "{}", Keyword::Asc),
            Self::Desc => write!(f, "{}", Keyword::Desc),
        }
    }
}

/*impl fmt::Display for Order {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.1 {
            None => write!(f, "{}", self.0),
            Some(sort) => write!(f, "{} {}", self.0, sort),
        }
    }
}*/

impl Format for SortKey {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::ColumnRef(c) => f.append_format(c),
        }
    }
}

impl Format for Order {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(&self.0);
        match &self.1 {
            None => f,
            Some(sort) => f.ws().append_format(sort),
        }
    }
}

impl Clause for OrderByClause {
    const KEYWORD: &'static Keyword = &Keyword::Order;
}

impl Format for OrderByClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.left_side(self.keyword())
            .ws()
            .append(&Keyword::By)
            .ws()
            .append_format(&self.0)
    }
}

impl Format for List<Order> {
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
