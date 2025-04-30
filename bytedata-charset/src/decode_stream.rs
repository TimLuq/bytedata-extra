use super::CharsetDecoding;

/// The operation to perform when an invalid character is encountered.
#[derive(Debug, Clone, Copy)]
#[expect(clippy::exhaustive_enums)]
pub enum DecodeInvalid {
    /// Replace the invalid character with the replacement character.
    Replace,
    /// Ignore the invalid character.
    Ignore,
    /// Return an error when an invalid character is encountered.
    Error,
}

pub struct DecodeStream<'a, C, S> {
    charset: C,
    source: S,
    buffer: bytedata::ByteQueue<'a>,
    invalid: DecodeInvalid,
}

impl<C: core::fmt::Debug, S> core::fmt::Debug for DecodeStream<'_, C, S> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.debug_struct("DecodeStream")
            .field("charset", &self.charset)
            .field("buffer", &self.buffer)
            .field("invalid", &self.invalid)
            .finish()
    }
}

impl<C: CharsetDecoding, S> DecodeStream<'_, C, S> {
    /// Create a new decode stream.
    #[inline]
    #[must_use]
    pub const fn new(charset: C, source: S) -> Self {
        Self {
            charset,
            source,
            buffer: bytedata::ByteQueue::new(),
            invalid: DecodeInvalid::Replace,
        }
    }

    /// Set the invalid character handling.
    #[inline]
    pub fn set_invalid(&mut self, invalid: DecodeInvalid) {
        self.invalid = invalid;
    }

    /// Set the invalid character handling.
    #[inline]
    #[must_use]
    pub const fn with_invalid(mut self, invalid: DecodeInvalid) -> Self {
        self.invalid = invalid;
        self
    }

    /// Returns the charset used for decoding.
    #[inline]
    pub const fn charset(&self) -> &C {
        &self.charset
    }
}

#[inline]
fn read_buffer<'a>(
    chunk_buffer_input_n: &mut usize,
    chunk_buffer_input: &mut [u8],
    charset_max_bytes: u16,
    buffer: &bytedata::ByteQueue<'a>,
) -> bytedata::ByteData<'a> {
    let mut input_n = *chunk_buffer_input_n;
    if input_n != 0 {
        let end = (charset_max_bytes as usize).min(chunk_buffer_input.len());
        let req = end - input_n;
        if req == 0 {
            return bytedata::ByteData::empty();
        }
        let mut moved = false;
        for byte in buffer.bytes().skip(input_n).take(req) {
            chunk_buffer_input[input_n] = byte;
            input_n += 1;
            *chunk_buffer_input_n = input_n;
            moved = true;
        }
        if !moved {
            return bytedata::ByteData::empty();
        }
        bytedata::ByteData::from_chunk_slice(&chunk_buffer_input[..input_n])
    } else if let Some(chunk) = buffer.front() {
        chunk.clone()
    } else {
        bytedata::ByteData::empty()
    }
}

impl<'a, C: CharsetDecoding, S: Iterator> DecodeStream<'a, C, S>
where
    S::Item: Into<bytedata::ByteData<'a>>,
{
    /// Pulls the next chunk of data from the source iterator and decodes it.
    #[expect(clippy::missing_inline_in_public_items)]
    pub fn iter_next(&mut self) -> Option<Result<bytedata::StringData<'a>, &'static str>> {
        let mut chunk_buffer_output = [0_u8; 14];
        let mut chunk_buffer_input = [0_u8; 14];
        let mut chunk_buffer_output_n = 0;
        let mut chunk_buffer_input_n = 0;
        loop {
            loop {
                let inner_buf = read_buffer(
                    &mut chunk_buffer_input_n,
                    &mut chunk_buffer_input,
                    self.charset.size_hint().1,
                    &self.buffer,
                );
                match self.charset.decode(inner_buf.as_slice()) {
                    crate::DecodeResult::Char(ch, len) => {
                        core::mem::drop(self.buffer.drain(..len as usize));
                        let u8_len = ch
                            .encode_utf8(&mut chunk_buffer_output[chunk_buffer_output_n..])
                            .len();
                        chunk_buffer_output_n += u8_len;
                        if chunk_buffer_output_n <= 10 {
                            continue;
                        }
                        let chunk = bytedata::ByteData::from_chunk_slice(
                            &chunk_buffer_output[..chunk_buffer_output_n],
                        );
                        // SAFETY: the buffer is filled with valid utf-8 data
                        let chunk = unsafe { bytedata::StringData::from_bytedata_unchecked(chunk) };
                        return Some(Ok(chunk));
                    }
                    crate::DecodeResult::InvalidChar(_char_data, len) => {
                        match self.invalid {
                            DecodeInvalid::Replace => {
                                core::mem::drop(self.buffer.drain(..len as usize));
                                chunk_buffer_input_n = 0;
                                let u8_len = '\u{FFFD}'
                                    .encode_utf8(&mut chunk_buffer_output[chunk_buffer_output_n..])
                                    .len();
                                chunk_buffer_output_n += u8_len;
                                if chunk_buffer_output_n <= 10 {
                                    continue;
                                }
                                let chunk = bytedata::ByteData::from_chunk_slice(
                                    &chunk_buffer_output[..chunk_buffer_output_n],
                                );
                                // SAFETY: the buffer is filled with valid utf-8 data
                                let chunk =
                                    unsafe { bytedata::StringData::from_bytedata_unchecked(chunk) };
                                return Some(Ok(chunk));
                            }
                            DecodeInvalid::Ignore => {
                                core::mem::drop(self.buffer.drain(..len as usize));
                                chunk_buffer_input_n = 0;
                                continue;
                            }
                            DecodeInvalid::Error => {
                                return Some(Err("invalid character"));
                            }
                        }
                    }
                    crate::DecodeResult::Utf8(len) => {
                        if chunk_buffer_output_n != 0 {
                            let chunk = bytedata::ByteData::from_chunk_slice(
                                &chunk_buffer_output[..chunk_buffer_output_n],
                            );
                            // SAFETY: the buffer is filled with valid utf-8 data
                            let chunk =
                                unsafe { bytedata::StringData::from_bytedata_unchecked(chunk) };
                            return Some(Ok(chunk));
                        }
                        #[expect(clippy::cast_possible_truncation)]
                        let len = len as usize;
                        core::mem::drop(self.buffer.drain(..len));
                        let inner_buf = if inner_buf.len() == len {
                            inner_buf
                        } else {
                            inner_buf.into_sliced(..len)
                        };
                        // SAFETY: the buffer is filled with valid utf-8 data
                        return Some(Ok(unsafe {
                            bytedata::StringData::from_bytedata_unchecked(inner_buf)
                        }));
                    }
                    crate::DecodeResult::Incomplete | crate::DecodeResult::Empty => {
                        break;
                    }
                }
            }

            if chunk_buffer_output_n != 0 {
                let chunk = bytedata::ByteData::from_chunk_slice(
                    &chunk_buffer_output[..chunk_buffer_output_n],
                );
                // SAFETY: the buffer is filled with valid utf-8 data
                let chunk = unsafe { bytedata::StringData::from_bytedata_unchecked(chunk) };
                return Some(Ok(chunk));
            }

            let Some(additional_data) = self.source.next() else {
                if !self.buffer.is_empty() {
                    return None;
                }
                return Some(Err("got to end of stream while decoding characters"));
            };

            let additional_data = additional_data.into();

            self.buffer.push_back(additional_data);
        }
    }
}

#[cfg(feature = "std")]
pub trait DecodeStreamRead {
    /// Read the next chunk of data from the stream and decode it into the buffer.
    ///
    /// # Errors
    ///
    /// - `std::io::ErrorKind::UnexpectedEof` → if the stream ends before the next character is fully read.
    /// - `*` → if the input stream returns an error.
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> std::io::Result<&'a str>;
}

#[cfg(feature = "std")]
impl<C: CharsetDecoding, S: std::io::Read> DecodeStreamRead for DecodeStream<'_, C, S> {
    #[expect(clippy::too_many_lines, clippy::missing_inline_in_public_items)]
    fn read<'a>(&'a mut self, buf: &'a mut [u8]) -> std::io::Result<&'a str> {
        debug_assert!(buf.len() >= 4, "buf.len() must be at least 4 bytes");
        let mut t_offset = 0;
        let mut chunk_buffer = [0_u8; 12];
        loop {
            let fail: std::io::ErrorKind;
            let mut chunk_buffer_n = 0;
            loop {
                let inner_buf = read_buffer(
                    &mut chunk_buffer_n,
                    &mut chunk_buffer,
                    self.charset.size_hint().1,
                    &self.buffer,
                );
                match self.charset.decode(inner_buf.as_slice()) {
                    crate::DecodeResult::Char(ch, len) => {
                        let u8len = ch.len_utf8();
                        if u8len <= buf.len() - t_offset {
                            ch.encode_utf8(&mut buf[t_offset..]);
                            t_offset += u8len;
                            core::mem::drop(self.buffer.drain(..len as usize));
                            chunk_buffer_n = 0;
                            continue;
                        }
                        fail = std::io::ErrorKind::WriteZero;
                        break;
                    }
                    crate::DecodeResult::InvalidChar(_char_data, len) => {
                        match self.invalid {
                            DecodeInvalid::Replace => {
                                if 3 <= buf.len() - t_offset {
                                    buf[t_offset] = 0xEF;
                                    buf[t_offset + 1] = 0xBF;
                                    buf[t_offset + 2] = 0xBD;
                                    t_offset += 3;
                                    core::mem::drop(self.buffer.drain(..len as usize));
                                    chunk_buffer_n = 0;
                                    continue;
                                }
                                fail = std::io::ErrorKind::WriteZero;
                                break;
                            }
                            DecodeInvalid::Ignore => {
                                core::mem::drop(self.buffer.drain(..len as usize));
                                chunk_buffer_n = 0;
                                continue;
                            }
                            DecodeInvalid::Error => {
                                if t_offset != 0 {
                                    // SAFETY: the buffer is filled with valid utf-8 data up to `t_offset`
                                    return Ok(unsafe {
                                        core::str::from_utf8_unchecked(&buf[..t_offset])
                                    });
                                }
                                return Err(std::io::Error::new(
                                    std::io::ErrorKind::InvalidData,
                                    "invalid character",
                                ));
                            }
                        }
                    }
                    crate::DecodeResult::Utf8(len) => {
                        let maxlen = buf.len() as u64 - t_offset as u64;
                        let slic = inner_buf.as_slice();
                        #[expect(clippy::cast_possible_truncation)]
                        let len = if len > maxlen {
                            let maxlen = maxlen as usize;
                            let start = maxlen - 1;
                            let mut i = start;
                            while i > 0 && (slic[i] & 0xC0) == 0x80 {
                                i -= 1;
                            }
                            let add = match slic[i] {
                                0xC0..=0xDF => 2,
                                0xE0..=0xEF => 3,
                                0xF0..=0xF7 => 4,
                                _ => 1,
                            };
                            if add == maxlen - i {
                                i = maxlen;
                            }
                            i
                        } else {
                            len as usize
                        };
                        let end = t_offset + len;
                        buf[t_offset..end].copy_from_slice(&slic[..len]);
                        t_offset += len;
                        core::mem::drop(self.buffer.drain(..len));
                        chunk_buffer_n = 0;
                        continue;
                    }
                    crate::DecodeResult::Empty => {
                        fail = std::io::ErrorKind::BrokenPipe;
                        break;
                    }
                    crate::DecodeResult::Incomplete => {
                        if chunk_buffer_n == 0
                            && self.buffer.len() > inner_buf.len()
                            && inner_buf.len() < chunk_buffer.len()
                        {
                            chunk_buffer_n = inner_buf.len();
                            chunk_buffer[..chunk_buffer_n].copy_from_slice(inner_buf.as_slice());
                            continue;
                        }
                        fail = std::io::ErrorKind::UnexpectedEof;
                        break;
                    }
                }
            }

            // if the buffer was filled from the inner buffer, we should return the result
            if t_offset != 0 {
                // SAFETY: the buffer is filled with valid utf-8 data up to `t_offset`
                return Ok(unsafe { core::str::from_utf8_unchecked(&buf[..t_offset]) });
            }

            // if the user provided buffer is smaller than the chunk buffer, we should use it for the next read
            let (zero_cop, wbuf) = if buf.len() < chunk_buffer.len() {
                (false, chunk_buffer.as_mut_slice())
            } else {
                // there are some lifetime issues with the buffer since we mix two different buffers,
                // so we need to transmute the lifetime away
                // SAFETY: the usage lifetime is the shorter of the two lifetimes
                let buf = unsafe { core::mem::transmute_copy::<&mut [u8], &mut [u8]>(&buf) };
                (true, buf)
            };

            // read more bytes from the source
            let len = self.source.read(wbuf)?;
            if len == 0 {
                if self.buffer.is_empty() {
                    // SAFETY: the buffer is empty, so we can return an empty string
                    return Ok(unsafe { core::str::from_utf8_unchecked(&buf[..0]) });
                }
                if fail == std::io::ErrorKind::BrokenPipe {
                    // encoder returned "empty", but we have some bytes left
                    // for now, we just return the empty string

                    // SAFETY: the buffer is empty, so we can return an empty string
                    return Ok(unsafe { core::str::from_utf8_unchecked(&buf[..0]) });
                }
                return Err(std::io::Error::new(
                    fail,
                    "got to end of stream while decoding characters",
                ));
            }

            // if the charset is single-byte or var-byte, we can test if the read data starts with something utf-8 compatible
            if self.buffer.is_empty() && zero_cop && self.charset.size_hint().0 == 1 {
                if let crate::DecodeResult::Utf8(utf_len) = self.charset.decode(&buf[..len]) {
                    #[expect(clippy::cast_possible_truncation)]
                    let utf_len = utf_len as usize;
                    if utf_len == len {
                        // SAFETY: the whole chunk is utf-8 compatible, so return it without copying
                        return Ok(unsafe { core::str::from_utf8_unchecked(&buf[..len]) });
                    }
                    // mark the utf-8 compatible part as read
                    t_offset += utf_len;
                }
            }

            // fill buffer with the read data
            let data = &wbuf[t_offset..len];
            let data = if data.len() <= bytedata::ByteChunk::LEN {
                bytedata::ByteData::from_chunk_slice(data)
            } else {
                bytedata::ByteData::from_shared(data.into())
            };
            self.buffer.push_back(data);
        }
    }
}

#[cfg(feature = "std")]
impl<C: CharsetDecoding, S: std::io::Read> std::io::Read for DecodeStream<'_, C, S> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        DecodeStreamRead::read(self, buf).map(str::len)
    }
}
