pub mod parser;
#[cfg(test)]
mod tests;

pub use self::parser::parse_update_statement;
use crate::clause::Clause;
use crate::expression::Expression;
use crate::expression::WhereClause;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::identifier::Name;
use crate::keyword::Keyword;
use crate::list::List;
use crate::table::create::TableRef;

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateStatement(pub UpdateClause, pub SetClause, pub Option<WhereClause>);

#[derive(Debug, PartialEq, Clone)]
pub struct UpdateClause(pub TableRef);

#[derive(Debug, PartialEq, Clone)]
pub struct SetClause(pub List<SetExpression>);

#[derive(Debug, PartialEq, Clone)]
pub struct SetExpression(pub Name, pub Expression);

impl Clause for UpdateClause {
    const KEYWORD: &'static Keyword = &Keyword::Update;
}

impl Clause for SetClause {
    const KEYWORD: &'static Keyword = &Keyword::Set;
}

impl Format for UpdateStatement {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.set_pad(self.0.keyword().len());
        f.append_format(&self.0).append_clause(&self.1);
        match &self.2 {
            Some(i) => f.append_clause(i),
            None => f,
        }
    }
}

impl Format for UpdateClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append(self.keyword()).space().append(&self.0)
    }
}

impl Format for SetClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_with_padding(self.keyword()).space();
        for (pos, e) in self.0 .0.iter().enumerate() {
            match pos {
                0 => f.append_format(e),
                _ => f.append_str(",").new_line().right_side(e),
            };
        }
        f
    }
}

impl Format for SetExpression {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_format(&self.0)
            .ws()
            .append_str("=")
            .ws()
            .append_format(&self.1)
    }
}
