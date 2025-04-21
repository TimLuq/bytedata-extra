use bytedata::StringData;

use crate::ParseError;

pub trait LineSource {
    type Error;

    fn read_line(&mut self) -> Result<Option<StringData<'_>>, Self::Error>;
}

impl LineSource for bytedata::StringQueue<'_> {
    type Error = crate::ParseError;

    fn read_line(&mut self) -> Result<Option<StringData<'_>>, Self::Error> {
        let line = self.take_line();
        if line.is_empty() {
            Ok(None)
        } else {
            let line = bytedata::StringData::from(line);
            Ok(Some(line))
        }
    }
}

pub(crate) struct LineSourceBuf<S> {
    pub(crate) line_num: u64,
    source: S,
}


impl<S: LineSource> LineSourceBuf<S> {
    pub fn new(source: S) -> Self {
        Self {
            line_num: 0,
            source,
        }
    }

    pub fn read_line<E: From<S::Error>>(&mut self) -> Result<Option<StringData<'_>>, E> {
        match self.source.read_line() {
            Ok(Some(line)) => {
                self.line_num += 1;
                Ok(Some(line))
            }
            Ok(None) => Ok(None),
            Err(e) => Err(E::from(e)),
        }
    }
}

pub(crate) trait LineParser {
    fn parse_line(&mut self, line: &StringData<'_>) -> Result<(), ParseError>;
}
