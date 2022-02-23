use crate::ws::ws;
use nom::bytes::complete::tag_no_case;
use nom::bytes::complete::take_while1;
use nom::combinator::map;
use nom::combinator::map_res;
use nom::combinator::peek;
use nom::IResult;
use std::fmt;
use std::str::FromStr;

#[derive(Debug, EnumString, IntoStaticStr, Clone, Copy, PartialEq)]
#[strum(ascii_case_insensitive)]
pub enum Keyword {
    Character,
    Varying,
    Select,
    Date,
    With,
    Cascade,
    Restrict,
    Without,
    Time,
    Zone,
    As,
    From,
    Case,
    Join,
    Default,
    Into,
    Create,
    Sequence,
    Alter,
    Drop,
    Start,
    Timestamp,
    Increment,
    Cache,
    No,
    Minvalue,
    Maxvalue,
    Table,
    Group,
    Null,
    By,
    Nulls,
    First,
    Asc,
    All,
    Intersect,
    Minus,
    When,
    Then,
    Else,
    Inner,
    Outer,
    Cross,
    Left,
    Right,
    Full,
    Natural,
    End,
    Union,
    Except,
    Desc,
    Last,
    Between,
    In,
    Is,
    Like,
    Not,
    Primary,
    Key,
    Unique,
    Where,
    And,
    Or,
    Limit,
    On,
    Using,
    Avg,
    Order,
    DropTable,
    Having,
    Add,
    Column,
    References,
    Foreign,
    Action,
    Delete,
    Update,
    Insert,
    Type,
    Values,
    Constraint,
    Check,
    Set,
}

#[allow(clippy::len_without_is_empty)]
impl Keyword {
    pub fn len(&self) -> usize {
        self.to_string().len()
    }
}

pub fn is_keyword(input: &str) -> IResult<&str, Keyword> {
    map_res(peek(take_while1(|c| c != ' ')), |s: &str| {
        Keyword::from_str(&s.to_lowercase())
    })(input)
}

impl fmt::Display for Keyword {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let keyword: &'static str = self.into();
        write!(f, "{}", keyword.to_uppercase())
    }
}

pub fn parse_keyword(keyword: Keyword) -> impl Fn(&str) -> IResult<&str, ()> {
    let key: &str = keyword.into();
    move |i: &str| map(ws(tag_no_case(key)), |_| ())(i)
}

#[cfg(test)]
mod tests;
