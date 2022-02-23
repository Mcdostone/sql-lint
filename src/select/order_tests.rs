use crate::{
    formatter::Format,
    identifier::Name,
    list::List,
    select::order::{parse_order_by_clause, NullsSort, Order, OrderByClause, OrderSort, SortKey},
    term::column::ColumnRef,
};

#[test]
fn test_order_by() {
    let input = "ORDER BY username";
    assert_eq!(
        parse_order_by_clause(input),
        Ok((
            "",
            OrderByClause(List(vec!(Order(
                SortKey::ColumnRef(ColumnRef::Name(Name::Name("username".to_string()))),
                None,
                None
            ))))
        ))
    )
}

#[test]
fn test_order_by_desc() {
    let input = "ORDER BY username DESC";
    assert_eq!(
        parse_order_by_clause(input),
        Ok((
            "",
            OrderByClause(List(vec!(Order(
                SortKey::ColumnRef(ColumnRef::Name(Name::Name("username".to_string()))),
                Some(OrderSort::Desc),
                None
            ))))
        ))
    )
}

#[test]
fn test_order_by_nulls_first() {
    let input = "ORDER BY username NULLS FIRST";
    assert_eq!(
        parse_order_by_clause(input),
        Ok((
            "",
            OrderByClause(List(vec!(Order(
                SortKey::ColumnRef(ColumnRef::Name(Name::Name("username".to_string()))),
                None,
                Some(NullsSort::First)
            ))))
        ))
    )
}

#[test]
fn test_order_by_nulls_last() {
    let input = "ORDER BY username NULLS LAST";
    assert_eq!(
        parse_order_by_clause(input),
        Ok((
            "",
            OrderByClause(List(vec!(Order(
                SortKey::ColumnRef(ColumnRef::Name(Name::Name("username".to_string()))),
                None,
                Some(NullsSort::Last)
            ))))
        ))
    )
}

#[test]
fn test_order_by_desc_nulls_last() {
    let input = "ORDER BY username DESC NULLS LAST";
    assert_eq!(
        parse_order_by_clause(input),
        Ok((
            "",
            OrderByClause(List(vec!(Order(
                SortKey::ColumnRef(ColumnRef::Name(Name::Name("username".to_string()))),
                Some(OrderSort::Desc),
                Some(NullsSort::Last)
            ))))
        ))
    )
}

#[test]
fn test_format_order_by() {
    assert_eq!(
        parse_order_by_clause("ORDEr by username desc")
            .unwrap()
            .1
            .lol(),
        "ORDER BY username DESC"
    );
    assert_eq!(
        parse_order_by_clause("ORDER BY email asc").unwrap().1.lol(),
        "ORDER BY email ASC"
    );
    assert_eq!(
        parse_order_by_clause("ORDER BY age").unwrap().1.lol(),
        "ORDER BY age"
    );
    assert_eq!(
        parse_order_by_clause("ORDER BY age  ,  year")
            .unwrap()
            .1
            .lol(),
        "ORDER BY age, year"
    )
}
