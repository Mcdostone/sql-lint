use crate::character::parse_comma;
use crate::table::constraint::parse_table_constraint_definition;
use crate::table::constraint::TableConstraintDefinition;
use crate::table::Formatter;
use crate::term::column::parse_column_ref;
use crate::term::column::ColumnRef;
use crate::term::value::parse_value;
use crate::term::value::Value;
use nom::combinator::value;
use nom::sequence::pair;

use nom::{
    branch::alt,
    combinator::{map, opt},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

use crate::{
    formatter::Format,
    identifier::{parse_name, Name},
    keyword::{parse_keyword, Keyword},
    ws::ws,
};

use super::{
    create::{parse_column_def, parse_table_ref, ColumnDef, TableRef},
    drop_table::{parse_drop_behavior, DropBehavior},
};

#[derive(Debug, PartialEq, Clone)]
pub struct AlterTable(pub TableRef, pub Vec<AlterTableAction>);

pub fn parse_alter_table(input: &str) -> IResult<&str, AlterTable> {
    map(
        tuple((
            parse_keyword(Keyword::Alter),
            parse_keyword(Keyword::Table),
            ws(parse_table_ref),
            separated_list1(parse_comma, parse_alter_table_action),
        )),
        |(_, _, t, v)| AlterTable(t, v),
    )(input)
}

pub fn parse_alter_table_action(input: &str) -> IResult<&str, AlterTableAction> {
    alt((
        map(
            tuple((
                parse_keyword(Keyword::Add),
                opt(parse_keyword(Keyword::Column)),
                parse_column_def,
            )),
            |(_, _, c)| AlterTableAction::AddColumnDefinition(c),
        ),
        map(
            tuple((
                parse_keyword(Keyword::Drop),
                parse_keyword(Keyword::Column),
                parse_name,
                opt(parse_drop_behavior),
            )),
            |(_, _, c, b)| AlterTableAction::DropColumnDefinition(c, b),
        ),
        map(
            tuple((
                parse_keyword(Keyword::Add),
                parse_table_constraint_definition,
            )),
            |(_, t)| AlterTableAction::AddTableConstraintDefinition(t),
        ),
        map(
            tuple((
                parse_keyword(Keyword::Alter),
                opt(parse_keyword(Keyword::Column)),
                parse_column_ref,
                parse_alter_column_action,
            )),
            |(_, _, t, a)| AlterTableAction::AlterColumnDefinition(t, a),
        ),
    ))(input)
}

impl Format for AlterTable {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&Keyword::Alter)
            .ws()
            .append(&Keyword::Table)
            .ws()
            .append_format(&self.0);
        for i in self.1.iter() {
            f.new_line().indent(i);
        }
        f
    }
}

impl Format for AlterTableAction {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::AddColumnDefinition(c) => f
                .indent(&Keyword::Add)
                .ws()
                .append_format(&Keyword::Column)
                .ws()
                .append_format(c),
            Self::DropColumnDefinition(n, o) => {
                f.indent(&Keyword::Drop)
                    .ws()
                    .append_format(&Keyword::Column)
                    .ws()
                    .append_format(n);
                match o {
                    Some(e) => f.ws().append(e),
                    None => f,
                }
            }
            Self::AddTableConstraintDefinition(c) => f.indent(&Keyword::Add).ws().append_format(c),
            Self::AlterColumnDefinition(c, a) => f
                .indent(&Keyword::Alter)
                .ws()
                .append(&Keyword::Column)
                .ws()
                .append_format(c)
                .ws()
                .append_format(a),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum AlterTableAction {
    AddColumnDefinition(ColumnDef),
    DropColumnDefinition(Name, Option<DropBehavior>),
    AddTableConstraintDefinition(TableConstraintDefinition),
    AlterColumnDefinition(ColumnRef, AlterColumnAction),
}

#[derive(Debug, PartialEq, Clone)]
pub enum AlterColumnAction {
    SetDefaultColumnClause(DefaultOption),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DefaultOption {
    Value(Value),
}

pub fn parse_default_option(input: &str) -> IResult<&str, DefaultOption> {
    map(parse_value, DefaultOption::Value)(input)
}

pub fn parse_alter_column_action(input: &str) -> IResult<&str, AlterColumnAction> {
    map(
        tuple((
            parse_keyword(Keyword::Set),
            parse_keyword(Keyword::Default),
            parse_default_option,
        )),
        |(_, _, d)| AlterColumnAction::SetDefaultColumnClause(d),
    )(input)
}

impl Format for DefaultOption {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Value(v) => f.append_format(v),
        }
    }
}

impl Format for AlterColumnAction {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::SetDefaultColumnClause(c) => f
                .append(&Keyword::Set)
                .ws()
                .append(&Keyword::Default)
                .ws()
                .append_format(c),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReferentialTriggeredAction {
    Update(ReferentialAction),
    Delete(ReferentialAction),
}

impl Format for ReferentialTriggeredAction {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Delete(d) => f
                .append(&Keyword::On)
                .ws()
                .append(&Keyword::Delete)
                .ws()
                .append_format(d),
            Self::Update(u) => f
                .append(&Keyword::On)
                .ws()
                .append(&Keyword::Update)
                .ws()
                .append_format(u),
        }
    }
}

impl Format for ReferentialAction {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::NoAction => f.append(&Keyword::No).ws().append(&Keyword::Action),
            Self::Cascade => f.append(&Keyword::Cascade),
            Self::Restrict => f.append(&Keyword::Restrict),
            Self::SetDefault => f.append(&Keyword::Set).ws().append(&Keyword::Default),
            Self::SetNull => f.append(&Keyword::Set).ws().append(&Keyword::Null),
        }
    }
}

pub fn parse_referential_triggered_action(
    input: &str,
) -> IResult<&str, ReferentialTriggeredAction> {
    alt((
        map(
            tuple((
                parse_keyword(Keyword::On),
                parse_keyword(Keyword::Update),
                parse_referential_action,
            )),
            |(_, _, a)| ReferentialTriggeredAction::Update(a),
        ),
        map(
            tuple((
                parse_keyword(Keyword::On),
                parse_keyword(Keyword::Delete),
                parse_referential_action,
            )),
            |(_, _, d)| ReferentialTriggeredAction::Delete(d),
        ),
    ))(input)
}

pub fn parse_referential_action(input: &str) -> IResult<&str, ReferentialAction> {
    alt((
        value(ReferentialAction::Cascade, parse_keyword(Keyword::Cascade)),
        value(
            ReferentialAction::SetNull,
            pair(parse_keyword(Keyword::Set), parse_keyword(Keyword::Null)),
        ),
        value(
            ReferentialAction::SetDefault,
            pair(parse_keyword(Keyword::Set), parse_keyword(Keyword::Default)),
        ),
        value(
            ReferentialAction::Restrict,
            parse_keyword(Keyword::Restrict),
        ),
        value(
            ReferentialAction::NoAction,
            pair(parse_keyword(Keyword::No), parse_keyword(Keyword::Action)),
        ),
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub enum ReferentialAction {
    Cascade,
    SetNull,
    SetDefault,
    Restrict,
    NoAction,
}
