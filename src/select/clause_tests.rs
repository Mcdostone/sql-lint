use crate::identifier::Name;
use crate::list::List;
use crate::select::clause::SelectClause;
use crate::select::clause::SelectedExpression;
use crate::select::clause::SetQuantifier;
use crate::select::parse_select_clause;
use crate::term::column::ColumnRef;
use crate::term::Term;

#[test]
fn test_parse_clause() {
    let input = "SELECT distinct price";
    assert_eq!(
        parse_select_clause(input),
        Ok((
            "",
            SelectClause(
                Some(SetQuantifier::Distinct),
                List(vec!(SelectedExpression::Term(Term::ColumnRef(
                    ColumnRef::Name(Name::Name("price".to_string()))
                )))),
            )
        ))
    )
}
