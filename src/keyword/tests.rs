use crate::keyword::is_keyword;
use crate::keyword::Keyword;

#[test]
fn test_is_keyword() {
    let input = "UNION";
    assert_eq!(is_keyword(input), Ok(("UNION", Keyword::Union)))
}
