use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::insert::parse_insert_into_statement;
use crate::insert::InsertStatement;
use crate::list::List;
use crate::r#type::parse_user_defined_type_definition;
use crate::r#type::UserDefinedTypeDefinition;
use crate::select::table_operator::combined_tables;
use crate::select::table_operator::CombinedTables;
use crate::select::{parse_select_statement, SelectStatement};
use crate::sequence::parse_sequence;
use crate::sequence::Sequence;
use crate::set::parse_set_statement;
use crate::set::SetStatement;
use crate::table::parse_table;
use crate::table::Table;
use crate::update::parser::parse_update_statement;
use crate::update::UpdateStatement;
use crate::ws::ws;
use nom::multi::many1;
use nom::sequence::terminated;
use std::ops::Deref;

use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::combinator::map;
use nom::IResult;

#[derive(Debug, PartialEq)]
pub struct Statement(Query);

impl Format for Statement {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(&self.0).append_str(";")
    }
}

impl Format for Query {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Query::CombinedSelect(c) => f.append_format(c),
            Query::Table(s) => f.append_format(s),
            Query::Select(select) => f.append_format(select.deref()),
            Query::Update(c) => f.append_format(c),
            Query::Insert(c) => f.append_format(c),
            Query::Sequence(c) => f.append_format(c),
            Query::UserDefinedType(c) => f.append_format(c),
            Query::Set(c) => f.append_format(c),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum Query {
    CombinedSelect(CombinedTables),
    Select(Box<SelectStatement>),
    Table(Table),
    Update(UpdateStatement),
    Insert(InsertStatement),
    Sequence(Sequence),
    UserDefinedType(UserDefinedTypeDefinition),
    Set(SetStatement),
}

pub fn parse_query(input: &str) -> IResult<&str, Query> {
    alt((
        map(combined_tables, Query::CombinedSelect),
        map(parse_select_statement, |s| Query::Select(Box::new(s))),
        map(parse_table, Query::Table),
        map(parse_update_statement, Query::Update),
        map(parse_insert_into_statement, Query::Insert),
        map(parse_sequence, Query::Sequence),
        map(parse_user_defined_type_definition, Query::UserDefinedType),
        map(parse_set_statement, Query::Set),
    ))(input)
}

pub fn parse_statements(input: &str) -> IResult<&str, List<Statement>> {
    map(
        many1(map(terminated(parse_query, ws(tag(";"))), Statement)),
        List,
    )(input)
}

impl Format for List<Statement> {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        for (pos, i) in self.0.iter().enumerate() {
            match pos {
                0 => f.append_format(i),
                _ => f.new_line().new_line().append_format(i),
            };
        }
        f
    }
}

#[cfg(test)]
mod tests;
