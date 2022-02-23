use super::clause::SelectClause;
use super::parse_select_clause;
use crate::expression::parse_where_clause;
use crate::expression::WhereClause;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::keyword::Keyword;
use crate::select::group::parse_group_by_clause;
use crate::select::group::GroupByClause;
use crate::select::limit::parse_limit_clause;
use crate::select::limit::LimitClause;
use crate::select::order::parse_order_by_clause;
use crate::select::order::OrderByClause;
use crate::select::parse_from_clause;
use crate::select::FromClause;
use crate::statement::Statement;
use nom::combinator::opt;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct SelectStatement {
    pub select: SelectClause,
    pub from: Option<FromClause>,
    pub r#where: Option<WhereClause>,
    pub group_by: Option<GroupByClause>,
    pub order_by: Option<OrderByClause>,
    pub limit: Option<LimitClause>,
}

pub fn parse_select_statement(input: &str) -> IResult<&str, SelectStatement> {
    let (input, select) = parse_select_clause(input)?;
    let (input, from) = opt(parse_from_clause)(input)?;
    let (input, where_clause) = opt(parse_where_clause)(input)?;
    let (input, group_by) = opt(parse_group_by_clause)(input)?;
    let (input, order_by) = opt(parse_order_by_clause)(input)?;
    let (input, limit) = opt(parse_limit_clause)(input)?;
    Ok((
        input,
        SelectStatement {
            select,
            from,
            r#where: where_clause,
            group_by,
            order_by,
            limit,
        },
    ))
}

impl Statement for SelectStatement {
    fn ok(&self) -> usize {
        Keyword::Select.len()
    }
}

impl Format for SelectStatement {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.set_pad(self.ok());
        f.append_format(&self.select);
        match &self.from {
            Some(i) => f.append_clause(i),
            None => f,
        };
        match &self.r#where {
            Some(i) => f.append_clause(i),
            None => f,
        };
        match &self.group_by {
            Some(i) => f.append_clause(i),
            None => f,
        };
        match &self.order_by {
            Some(i) => f.append_clause(i),
            None => f,
        };
        match &self.limit {
            Some(l) => f.append_clause(l),
            None => f,
        }
    }
}
