#[cfg(test)]
mod tests;

use crate::character::parse_comma;
use crate::character::parse_left_parenthesis;
use crate::character::parse_right_parenthesis;
use crate::data_type::parse_predefined_type;
use crate::data_type::PredefinedType;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::identifier::parse_schema_qualified_name;
use crate::identifier::SchemaQualifiedName;
use crate::keyword::parse_keyword;
use crate::keyword::Keyword;
use crate::list::List;
use crate::term::parse_term;
use nom::combinator::map;
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::sequence::pair;
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct UserDefinedTypeDefinition(
    pub SchemaQualifiedName,
    pub Option<PredefinedType>,
    pub List<Member>,
);

pub fn parse_representation(input: &str) -> IResult<&str, PredefinedType> {
    map(
        pair(parse_keyword(Keyword::As), parse_predefined_type),
        |(_, s)| s,
    )(input)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Member(pub String);

pub fn parse_user_defined_type_definition(input: &str) -> IResult<&str, UserDefinedTypeDefinition> {
    map(
        tuple((
            parse_keyword(Keyword::Create),
            parse_keyword(Keyword::Type),
            parse_schema_qualified_name,
            opt(parse_representation),
            delimited(
                parse_left_parenthesis,
                separated_list1(parse_comma, parse_member),
                parse_right_parenthesis,
            ),
        )),
        |(_, _, n, t, l)| UserDefinedTypeDefinition(n, t, List(l)),
    )(input)
}

pub fn parse_member(input: &str) -> IResult<&str, Member> {
    map(parse_term, |t| Member(t.output()))(input)
}

impl Format for Member {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&self.0)
    }
}

impl Format for UserDefinedTypeDefinition {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(&Keyword::Create)
            .ws()
            .append(&Keyword::Type)
            .ws()
            .append_format(&self.0);
        match &self.1 {
            Some(s) => f.ws().append(&Keyword::As).ws().append_format(s),
            None => f,
        };
        f.ws().append_str("(");
        f.set_pad(4);
        for (pos, p) in self.2 .0.iter().enumerate() {
            match pos {
                0 => f.new_line().indent(p),
                _ => f.append_str(",").new_line().indent(p),
            };
        }
        f.new_line().append_str(")")
    }
}
