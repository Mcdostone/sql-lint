use crate::character::parse_comma;
use crate::clause::Clause;
use crate::expression::parse_expression;
use crate::expression::Expression;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::term::column::parse_column_ref;
use crate::term::column::ColumnRef;
use crate::ws::ws;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct GroupByClause(pub List<GroupingElement>, pub Option<HavingClause>);

#[derive(Debug, PartialEq, Clone)]
pub enum GroupingElement {
    ColumnRef(ColumnRef),
}

#[derive(Debug, PartialEq, Clone)]
pub struct HavingClause(pub Expression);

impl Clause for GroupByClause {
    const KEYWORD: &'static Keyword = &Keyword::Group;
}

impl Format for HavingClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.left_side(self.keyword()).ws().append_format(&self.0)
    }
}

impl Format for GroupByClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.left_side(self.keyword())
            .ws()
            .append(&Keyword::By)
            .ws()
            .append_format(&self.0);
        match &self.1 {
            Some(i) => f.new_line().append_format(i),
            None => f,
        }
    }
}

impl Clause for HavingClause {
    const KEYWORD: &'static Keyword = &Keyword::Having;
}

pub fn parse_grouping_element(input: &str) -> IResult<&str, GroupingElement> {
    map(parse_column_ref, GroupingElement::ColumnRef)(input)
}

pub fn parse_group_by_clause(input: &str) -> IResult<&str, GroupByClause> {
    let (input, _) = GroupByClause::parse_keyword(input)?;
    map(
        tuple((
            parse_keyword(Keyword::By),
            separated_list1(parse_comma, ws(parse_grouping_element)),
            opt(parse_having_clause),
        )),
        |(_, e, h)| GroupByClause(List(e), h),
    )(input)
}

pub fn parse_having_clause(input: &str) -> IResult<&str, HavingClause> {
    map(
        tuple((parse_keyword(Keyword::Having), ws(parse_expression))),
        |(_, e)| HavingClause(e),
    )(input)
}

impl Format for GroupingElement {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::ColumnRef(c) => f.append_format(c),
        }
    }
}

impl Format for List<GroupingElement> {
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
