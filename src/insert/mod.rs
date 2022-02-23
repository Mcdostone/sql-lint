mod parser;
pub use self::parser::*;
use std::ops::Deref;

#[cfg(test)]
mod tests;

use crate::clause::Clause;
use crate::expression::Expression;
use crate::formatter::Format;
use crate::formatter::Formatter;
use crate::identifier::Name;
use crate::keyword::Keyword;
use crate::list::List;

use crate::table::create::TableRef;

#[derive(Debug, PartialEq, Clone)]
pub struct InsertStatement(pub InsertIntoClause, pub ValuesClause);

// INSERT INTO users (first_name, last_name)
#[derive(Debug, PartialEq, Clone)]
pub struct InsertIntoClause(pub TableRef, pub Option<List<Name>>);

// VALUES ('John', 'Doe'), ('Carpenter', 'Brut')
#[derive(Debug, PartialEq, Clone)]
pub struct ValuesClause(pub List<InsertValue>);

// ('John', 'Doe')
#[derive(Debug, PartialEq, Clone)]
pub enum InsertValue {
    Default,
    Expression(Box<Expression>),
    ParenthesisExpression(List<InsertValue>),
}

impl Clause for InsertIntoClause {
    const KEYWORD: &'static Keyword = &Keyword::Insert;
}

impl Clause for ValuesClause {
    const KEYWORD: &'static Keyword = &Keyword::Values;
}

impl Format for InsertStatement {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.set_pad(self.0.keyword().len());
        f.append_format(&self.0).append_clause(&self.1)
    }
}

impl Format for InsertIntoClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_with_padding(self.keyword())
            .space()
            .append(&Keyword::Into)
            .space()
            .append(&self.0);
        match &self.1 {
            Some(o) => f.space().append(&format!("({o})")),
            None => f,
        }
    }
}

impl Format for ValuesClause {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        f.append_with_padding(self.keyword()).space();
        for (pos, e) in self.0 .0.iter().enumerate() {
            match pos {
                0 => f.append_format(e),
                _ => f.new_line().right_side(e),
            };
        }
        f
    }
}

impl Format for InsertValue {
    fn format<'a>(&self, f: &'a mut Formatter) -> &'a mut Formatter {
        match self {
            Self::Default => f.append(&Keyword::Default),
            Self::Expression(e) => f.append_format(e.deref()),
            Self::ParenthesisExpression(e) => f.append_str("(").append_format(e).append_str(")"),
        }
    }
}

impl Format for List<InsertValue> {
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
