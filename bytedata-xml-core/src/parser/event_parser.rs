
struct EventParser<'a, S> {
    pub(crate) parser: bytedata_charset::DecodeStream<'a, bytedata_charset::Utf8Encoding, S>,
}

impl<'a, S> EventParser<'a, S> {
    pub fn new(parser: S) -> Self {
        Self { parser: bytedata_charset::DecodeStream::new(bytedata_charset::UTF8, parser) }
    }

    pub fn try_next_event(&mut self) -> Result<Option<crate::Event>, bytedata_charset::Error> {
        let mut parser = &mut self.parser;
        let mut event = bytedata_xml_core::Event::new();
        loop {
            let byte = parser.next_byte()?;
            match byte {
                b'<' => {
                    let byte = parser.next_byte()?;
                    match byte {
                        b'/' => {
                            let name = parser.read_until(b'>')?;
                            event.end_element(name);
                        }
                        _ => {
                            let name = parser.read_until(b' ')?;
                            event.start_element(name);
                        }
                    }
                }
                b'>' => {
                    return Ok(Some(event));
                }
                _ => {}
            }
        }
    }
}