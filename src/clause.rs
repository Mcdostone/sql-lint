use crate::keyword::Keyword;
use crate::ws::ws;
use nom::bytes::complete::tag_no_case;
use nom::combinator::map;
use nom::IResult;

pub trait Clause {
    const KEYWORD: &'static Keyword;

    fn keyword(&self) -> &Keyword {
        Self::KEYWORD
    }

    fn side(&self) -> usize {
        self.keyword().len()
    }

    fn parse_keyword(input: &str) -> IResult<&str, ()> {
        let clause: &'static str = Self::KEYWORD.into();
        map(ws(tag_no_case(clause)), |_| ())(input)
    }
}
