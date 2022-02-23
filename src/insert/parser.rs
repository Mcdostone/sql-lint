use crate::character::parse_comma;
use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::clause::Clause;
use crate::expression::parse_expression;
use crate::identifier::parse_name;
use crate::insert::InsertIntoClause;
use crate::insert::InsertStatement;
use crate::insert::InsertValue;
use crate::insert::ValuesClause;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::table::create::parse_table_ref;
use crate::ws::ws;
use nom::branch::alt;
use nom::combinator::map;
use nom::combinator::opt;
use nom::combinator::value;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;

pub fn parse_insert_into_clause(input: &str) -> IResult<&str, InsertIntoClause> {
    map(
        tuple((
            InsertIntoClause::parse_keyword,
            parse_keyword(Keyword::Into),
            ws(parse_table_ref),
            opt(map(
                delimited(
                    parse_left_parenthesis,
                    separated_list1(parse_comma, ws(parse_name)),
                    parse_right_parenthesis,
                ),
                List,
            )),
        )),
        |(_, _, t, cols)| InsertIntoClause(t, cols),
    )(input)
}

pub fn parse_values_clause(input: &str) -> IResult<&str, ValuesClause> {
    map(
        pair(
            ValuesClause::parse_keyword,
            map(separated_list1(parse_comma, ws(insert_value)), List),
        ),
        |(_, vals)| ValuesClause(vals),
    )(input)
}

pub fn parse_insert_into_statement(input: &str) -> IResult<&str, InsertStatement> {
    map(
        pair(parse_insert_into_clause, parse_values_clause),
        |(i, v)| InsertStatement(i, v),
    )(input)
}

fn insert_value(input: &str) -> IResult<&str, InsertValue> {
    alt((
        map(
            delimited(
                parse_left_parenthesis,
                separated_list1(parse_comma, ws(insert_value)),
                parse_right_parenthesis,
            ),
            |v| InsertValue::ParenthesisExpression(List(v)),
        ),
        value(InsertValue::Default, parse_keyword(Keyword::Default)),
        map(parse_expression, |e| InsertValue::Expression(Box::new(e))),
    ))(input)
}
