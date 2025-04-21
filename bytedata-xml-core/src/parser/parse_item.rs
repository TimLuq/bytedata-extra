use super::Location;

pub enum ParseError {
    Incomplete,
    NoMatch,
}

pub type ParseResult<T> = Result<(T, usize), ParseError>;

pub trait ParseItem<'a>: Sized {
    fn parse_item(input: &bytedata::StringQueue<'a>, location: &Location<'a>) -> ParseResult<Self>;
    fn can_parse(input: &bytedata::StringQueue<'a>) -> Result<usize, ParseError> {
        Self::parse_item(input, &Location::unknown()).map(|(_, len)| len)
    }
}
