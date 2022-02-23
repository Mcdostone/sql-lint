use nom::branch::alt;
use nom::combinator::map;
use nom::IResult;

use crate::formatter::Format;
use crate::formatter::Formatter;

use self::alter::parse_alter_table;
use self::create::parse_create_table;
use self::{alter::AlterTable, create::CreateTableStatement};

pub mod alter;
pub mod constraint;
pub mod create;
pub mod drop_table;

#[derive(Debug, PartialEq, Clone)]

pub enum Table {
    Create(CreateTableStatement),
    Alter(AlterTable),
}

pub fn parse_table(input: &str) -> IResult<&str, Table> {
    alt((
        map(parse_create_table, Table::Create),
        map(parse_alter_table, Table::Alter),
    ))(input)
}

impl Format for Table {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Create(s) => f.append_format(s),
            Self::Alter(s) => f.append_format(s),
        }
    }
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod create_tests;

#[cfg(test)]
mod drop_table_tests;

#[cfg(test)]
mod alter_tests;
