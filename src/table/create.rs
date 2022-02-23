use crate::character::parse_comma;
use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::clause::Clause;
use crate::data_type::parse_data_type;
use crate::data_type::DataType;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::identifier::parse_delimited_identifier;
use crate::identifier::parse_name;
use crate::identifier::Delimitedidentifier;
use crate::identifier::Name;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::table::constraint::parse_column_constraint_definition;
use crate::table::constraint::parse_table_constraint_definition;
use crate::table::constraint::ColumnConstraintDefinition;
use crate::table::constraint::TableConstraintDefinition;
use crate::term::value::parse_value;
use crate::term::value::Value;
use crate::ws::ws;
use nom::branch::alt;
use nom::character::complete::char;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;
use nom::Parser;
use std::fmt;

#[derive(Debug, PartialEq, Clone)]
pub struct CreateTableStatement(pub TableRef, pub List<TableElement>);

pub fn parse_create_table(input: &str) -> IResult<&str, CreateTableStatement> {
    let (input, _) = pair(
        parse_keyword(Keyword::Create),
        parse_keyword(Keyword::Table),
    )(input)?;
    let (input, table_name) = ws(parse_table_ref).parse(input)?;
    let (input, column_defs) = map(
        delimited(
            parse_left_parenthesis,
            separated_list1(parse_comma, ws(parse_table_element)),
            parse_right_parenthesis,
        ),
        List,
    )(input)?;
    Ok((input, CreateTableStatement(table_name, column_defs)))
}

pub fn parse_table_element(input: &str) -> IResult<&str, TableElement> {
    alt((
        map(parse_table_constraint_definition, |c| {
            TableElement::TableConstraintDefinition(c)
        }),
        map(parse_column_def, TableElement::ColumnDef),
    ))(input)
}

impl Format for CreateTableStatement {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&Keyword::Create)
            .ws()
            .append(&Keyword::Table)
            .ws()
            .append(&self.0)
            .ws()
            .append_str("(")
            .new_line();
        f.set_pad(4);

        for (pos, line) in self.1 .0.iter().enumerate() {
            match pos {
                0 => f.indent(line),
                _ => f.append_str(",").new_line().indent(line),
            };
        }
        f.new_line().append_str(")")
    }
}

impl Format for TableElement {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::ColumnDef(c) => f.indent(c),
            Self::TableConstraintDefinition(c) => f.indent(c),
        }
    }
}

impl Format for ColumnDef {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(&self.0).ws().append_format(&self.1);
        match &self.2 {
            Some(c) => f.ws().append_format(c),
            None => f,
        };
        match &self.3 {
            Some(c) => f.ws().append_format(c),
            None => f,
        }
    }
}

impl fmt::Display for TableRef {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.0 {
            Some(s) => write!(f, "{s}.{}", self.1),
            None => write!(f, "{}", self.1),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableRef(pub Option<Name>, pub Name);

pub fn parse_table_ref(input: &str) -> IResult<&str, TableRef> {
    alt((
        map(
            tuple((opt(parse_name), char('.'), parse_name)),
            |(s, _, n)| TableRef(s, n),
        ),
        map(parse_name, |n| TableRef(None, n)),
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnDef(
    pub Delimitedidentifier,
    pub DataType,
    pub Option<DefaultClause>,
    pub Option<ColumnConstraintDefinition>,
);

#[derive(Debug, PartialEq, Clone)]
pub enum DefaultOption {
    Value(Value),
}

#[derive(Debug, PartialEq, Clone)]
pub struct DefaultClause(DefaultOption);

impl Clause for DefaultClause {
    const KEYWORD: &'static Keyword = &Keyword::Default;
}

impl Format for DefaultClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(self.keyword()).ws().append_format(&self.0)
    }
}

impl Format for DefaultOption {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Value(v) => f.append(v),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum TableElement {
    ColumnDef(ColumnDef),
    TableConstraintDefinition(TableConstraintDefinition),
}

pub fn parse_column_def(input: &str) -> IResult<&str, ColumnDef> {
    map(
        tuple((
            ws(parse_delimited_identifier),
            ws(parse_data_type),
            opt(ws(parse_default_clause)),
            opt(ws(parse_column_constraint_definition)),
        )),
        |(name, d, def, c)| ColumnDef(name, d, def, c),
    )(input)
}

pub fn parse_default_clause(input: &str) -> IResult<&str, DefaultClause> {
    map(
        pair(DefaultClause::parse_keyword, parse_default_option),
        |(_, o)| DefaultClause(o),
    )(input)
}

pub fn parse_default_option(input: &str) -> IResult<&str, DefaultOption> {
    map(parse_value, DefaultOption::Value)(input)
}
