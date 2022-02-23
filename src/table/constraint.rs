use crate::character::parse_comma;
use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::identifier::parse_name;
use crate::identifier::Name;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::table::alter::parse_referential_triggered_action;
use crate::table::alter::ReferentialTriggeredAction;
use crate::table::create::parse_table_ref;
use crate::table::create::TableRef;
use crate::term::column::parse_column_ref;
use crate::term::column::ColumnRef;
use crate::ws::ws;
use nom::branch::alt;
use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;
use std::fmt;

pub fn parse_table_constraint_definition(input: &str) -> IResult<&str, TableConstraintDefinition> {
    map(
        pair(
            opt(parse_constraint_name_definition),
            parse_table_constraint,
        ),
        |(name, c)| TableConstraintDefinition(name, c),
    )(input)
}

pub fn parse_unique_specification(input: &str) -> IResult<&str, UniqueSpecification> {
    alt((
        map(parse_keyword(Keyword::Unique), |_| {
            UniqueSpecification::Unique
        }),
        map(
            pair(parse_keyword(Keyword::Primary), parse_keyword(Keyword::Key)),
            |_| UniqueSpecification::PrimaryKey,
        ),
    ))(input)
}

pub fn parse_unique_constraint_definition(
    input: &str,
) -> IResult<&str, UniqueConstraintDefinition> {
    map(
        pair(
            ws(parse_unique_specification),
            delimited(
                parse_left_parenthesis,
                separated_list1(parse_comma, parse_column_ref),
                parse_right_parenthesis,
            ),
        ),
        |(u, l)| UniqueConstraintDefinition(u, List(l)),
    )(input)
}

impl Format for UniqueConstraintDefinition {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(&self.0)
            .append_str("(")
            .append_format(&self.1)
            .append_str(")")
    }
}

impl Format for List<ColumnRef> {
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

impl fmt::Display for UniqueSpecification {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PrimaryKey => write!(f, "PRIMARY KEY"),
            Self::Unique => write!(f, "UNIQUE"),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum UniqueSpecification {
    Unique,
    PrimaryKey,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TableConstraintDefinition(Option<ConstraintNameDefinition>, TableConstraint);
#[derive(Debug, PartialEq, Clone)]
pub struct UniqueConstraintDefinition(UniqueSpecification, List<ColumnRef>);
#[derive(Debug, PartialEq, Clone)]
pub struct ReferentialConstraintDefinition(List<ColumnRef>, ReferencesSpecification);

#[derive(Debug, PartialEq, Clone)]
pub struct ReferencesSpecification(
    pub TableRef,
    pub List<ColumnRef>,
    Option<ReferentialTriggeredAction>,
);

#[derive(Debug, PartialEq, Clone)]
pub enum TableConstraint {
    Unique(UniqueConstraintDefinition),
    Reference(ReferentialConstraintDefinition),
    Check(String),
}

pub fn parse_references_specification(input: &str) -> IResult<&str, ReferencesSpecification> {
    map(
        tuple((
            parse_keyword(Keyword::References),
            ws(parse_table_ref),
            delimited(
                parse_left_parenthesis,
                separated_list1(parse_comma, parse_column_ref),
                parse_right_parenthesis,
            ),
            opt(parse_referential_triggered_action),
        )),
        |(_, t, l, r)| ReferencesSpecification(t, List(l), r),
    )(input)
}

pub fn parse_referential_constraint_definition(
    input: &str,
) -> IResult<&str, ReferentialConstraintDefinition> {
    map(
        tuple((
            parse_keyword(Keyword::Foreign),
            parse_keyword(Keyword::Key),
            delimited(
                parse_left_parenthesis,
                separated_list1(parse_comma, parse_column_ref),
                parse_right_parenthesis,
            ),
            parse_references_specification,
        )),
        |(_, _, l, s)| ReferentialConstraintDefinition(List(l), s),
    )(input)
}

pub fn parse_table_constraint(input: &str) -> IResult<&str, TableConstraint> {
    alt((
        map(parse_referential_constraint_definition, |u| {
            TableConstraint::Reference(u)
        }),
        map(parse_unique_constraint_definition, |u| {
            TableConstraint::Unique(u)
        }),
        map(
            pair(
                parse_keyword(Keyword::Check),
                delimited(
                    parse_left_parenthesis,
                    take_while1(|c| c != ')'),
                    parse_right_parenthesis,
                ),
            ),
            |(_, p)| TableConstraint::Check(p.to_string()),
        ),
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstraintNameDefinition(pub Name);

pub fn parse_constraint_name_definition(input: &str) -> IResult<&str, ConstraintNameDefinition> {
    map(
        pair(parse_keyword(Keyword::Constraint), ws(parse_name)),
        |(_, n)| ConstraintNameDefinition(n),
    )(input)
}

impl fmt::Display for ConstraintNameDefinition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", Keyword::Constraint, self.0)
    }
}

impl Format for TableConstraintDefinition {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match &self.0 {
            Some(n) => f.append_format(n).ws().append_format(&self.1),
            None => f.append_format(&self.1),
        }
    }
}

impl Format for ReferencesSpecification {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(&Keyword::References)
            .ws()
            .append_format(&self.0);
        f.append_str("(");
        for (pos, i) in self.1 .0.iter().enumerate() {
            match pos {
                0 => f.append_format(i),
                _ => f.append_str(", ").append_format(i),
            };
        }
        f.append_str(")");
        match &self.2 {
            Some(s) => f.ws().append_format(s),
            None => f,
        }
    }
}

impl Format for ReferentialConstraintDefinition {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&Keyword::Foreign)
            .ws()
            .append(&Keyword::Key)
            .ws()
            .append_str("(");
        for (pos, i) in self.0 .0.iter().enumerate() {
            match pos {
                0 => f.append_format(i),
                _ => f.append_str(", ").append_format(i),
            };
        }
        f.append_str(")").ws().append_format(&self.1)
    }
}

impl Format for TableConstraint {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Reference(r) => f.append_format(r),
            Self::Unique(u) => f.append_format(u),
            Self::Check(c) => f
                .append(&Keyword::Check)
                .append_str("(")
                .append(c)
                .append_str(")"),
        }
    }
}

impl Format for ColumnConstraintDefinition {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match &self.0 {
            Some(n) => f.append_format(n).ws().append_format(&self.1),
            None => f.append_format(&self.1),
        }
    }
}

impl Format for ColumnConstraint {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::NotNull => f.append(&Keyword::Not).ws().append(&Keyword::Null),
            Self::Unique(u) => f.append(u),
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum ColumnConstraint {
    NotNull,
    Unique(UniqueSpecification),
}

#[derive(Debug, PartialEq, Clone)]
pub struct ColumnConstraintDefinition(pub Option<ConstraintNameDefinition>, pub ColumnConstraint);

pub fn parse_column_constraint_definition(
    input: &str,
) -> IResult<&str, ColumnConstraintDefinition> {
    map(
        pair(
            opt(parse_constraint_name_definition),
            parse_column_constraint,
        ),
        |(name, c)| ColumnConstraintDefinition(name, c),
    )(input)
}

pub fn parse_column_constraint(input: &str) -> IResult<&str, ColumnConstraint> {
    alt((
        map(
            pair(parse_keyword(Keyword::Not), parse_keyword(Keyword::Null)),
            |_| ColumnConstraint::NotNull,
        ),
        map(parse_unique_specification, ColumnConstraint::Unique),
    ))(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Definition(Name);
