use crate::clause::Clause;
use crate::expression::parse_expression;
use crate::expression::parse_where_clause;
use crate::identifier::parse_name;
use crate::list::List;
use crate::table::create::parse_table_ref;
use crate::update::SetClause;
use crate::update::SetExpression;
use crate::update::UpdateClause;
use crate::update::UpdateStatement;
use crate::ws::ws;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_update_statement(input: &str) -> IResult<&str, UpdateStatement> {
    map(
        tuple((
            parse_update_clause,
            parse_set_clause,
            opt(parse_where_clause),
        )),
        |(u, s, w)| UpdateStatement(u, s, w),
    )(input)
}

pub fn parse_set_expression(input: &str) -> IResult<&str, SetExpression> {
    map(
        tuple((ws(parse_name), ws(tag("=")), ws(parse_expression))),
        |(n, _, e)| SetExpression(n, e),
    )(input)
}

pub fn parse_update_clause(input: &str) -> IResult<&str, UpdateClause> {
    map(
        pair(UpdateClause::parse_keyword, ws(parse_table_ref)),
        |(_, t)| UpdateClause(t),
    )(input)
}

pub fn parse_set_clause(input: &str) -> IResult<&str, SetClause> {
    map(
        pair(
            SetClause::parse_keyword,
            separated_list1(ws(tag(",")), parse_set_expression),
        ),
        |(_, l)| SetClause(List(l)),
    )(input)
}
