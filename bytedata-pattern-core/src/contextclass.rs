use crate::{
    charclass::is_full_ws,
    test_exec::{ExecFlag, ExecResult, TestExec},
    CharacterClass,
};

/// Zero-width context classes.
#[derive(Clone, Copy)]
#[non_exhaustive]
pub enum ContextClass {
    /// Matches the start of the input.
    Start,
    /// Matches the end of the input.
    End,
    /// Matches the location where a word starts or ends.
    WordBoundary,
    /// Matches a location at the start of a word.
    WordStart,
    /// Matches a location at the end of a word.
    WordEnd,
    /// Matches the end of the input while disregarding trailing newlines. `\n*$`
    EndTrimmed,
}

impl core::fmt::Debug for ContextClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Start => f.write_str("ContextClass::Start"),
            Self::End => f.write_str("ContextClass::End"),
            Self::WordBoundary => f.write_str("ContextClass::WordBoundary"),
            Self::WordStart => f.write_str("ContextClass::WordStart"),
            Self::WordEnd => f.write_str("ContextClass::WordEnd"),
            Self::EndTrimmed => f.write_str("ContextClass::EndTrimmed"),
        }
    }
}

impl core::fmt::Display for ContextClass {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Start => f.write_str("^"),
            Self::End => f.write_str("$"),
            Self::WordBoundary => f.write_str(r"\b"),
            Self::WordStart => f.write_str("\\<"),
            Self::WordEnd => f.write_str("\\>"),
            Self::EndTrimmed => f.write_str(r"\Z"),
        }
    }
}

impl ContextClass {
    pub(crate) const fn run_test(self, mut exec: TestExec<'_>) -> TestExec<'_> {
        match self {
            ContextClass::Start => {
                exec.result = if exec.offset == 0 {
                    None
                } else {
                    Some(ExecResult::Mismatch)
                };
                exec
            }
            ContextClass::End => {
                exec.result = if !exec.chunk.is_empty() {
                    Some(ExecResult::Mismatch)
                } else if exec.flags.is_set(ExecFlag::Incomplete) {
                    Some(ExecResult::Incomplete)
                } else {
                    None
                };
                exec
            }
            ContextClass::WordBoundary => {
                if exec.offset == 0 {
                    exec.result = None;
                    return exec;
                }
                if exec.chunk.is_empty() {
                    exec.result = if exec.flags.is_set(ExecFlag::Incomplete) {
                        Some(ExecResult::Incomplete)
                    } else {
                        None
                    };
                    return exec;
                }
                let (chk_chr, chk_len) = bytedata::const_utf8_char_next(exec.chunk);
                if chk_len == 0 {
                    exec.result = Some(ExecResult::Incomplete);
                    return exec;
                }
                let Some(chk_chr) = char::from_u32(chk_chr) else {
                    exec.result = Some(ExecResult::InvalidEncoding);
                    return exec;
                };

                let prev_word = exec.flags.is_set(ExecFlag::PrevWord);
                let next_word = CharacterClass::Word.check_char(chk_chr);
                exec.result = if prev_word != next_word {
                    None
                } else {
                    Some(ExecResult::Mismatch)
                };
                exec
            }
            ContextClass::WordStart => {
                if exec.chunk.is_empty() {
                    exec.result = if exec.flags.is_set(ExecFlag::Incomplete) {
                        Some(ExecResult::Incomplete)
                    } else {
                        Some(ExecResult::Mismatch)
                    };
                    return exec;
                }
                if exec.offset != 0 && exec.flags.is_set(ExecFlag::PrevWord) {
                    exec.result = Some(ExecResult::Mismatch);
                    return exec;
                }
                let (chk_chr, chk_len) = bytedata::const_utf8_char_next(exec.chunk);
                if chk_len == 0 {
                    exec.result = Some(ExecResult::Incomplete);
                    return exec;
                }
                let Some(chk_chr) = char::from_u32(chk_chr) else {
                    exec.result = Some(ExecResult::InvalidEncoding);
                    return exec;
                };
                exec.result = if CharacterClass::Word.check_char(chk_chr) {
                    None
                } else {
                    Some(ExecResult::Mismatch)
                };
                exec
            }
            ContextClass::WordEnd => {
                if exec.offset != 0 && !exec.flags.is_set(ExecFlag::PrevWord) {
                    exec.result = Some(ExecResult::Mismatch);
                    return exec;
                }
                if exec.chunk.is_empty() {
                    if exec.flags.is_set(ExecFlag::Incomplete) {
                        exec.result = Some(ExecResult::Incomplete);
                    } else {
                        exec.result = None;
                    }
                    return exec;
                }
                let (chk_chr, chk_len) = bytedata::const_utf8_char_next(exec.chunk);
                if chk_len == 0 {
                    exec.result = Some(ExecResult::Incomplete);
                    return exec;
                }
                let Some(chk_chr) = char::from_u32(chk_chr) else {
                    exec.result = Some(ExecResult::InvalidEncoding);
                    return exec;
                };
                exec.result = if CharacterClass::Word.check_char(chk_chr) {
                    Some(ExecResult::Mismatch)
                } else {
                    None
                };
                exec
            }
            ContextClass::EndTrimmed => {
                if exec.chunk.is_empty() {
                    exec.result = if exec.flags.is_set(ExecFlag::Incomplete) {
                        Some(ExecResult::Incomplete)
                    } else {
                        None
                    };
                    return exec;
                }
                let mut loop_exec = exec;
                loop {
                    let (chr, len) = bytedata::const_utf8_char_next(loop_exec.chunk);
                    if len == 0 {
                        exec.result = Some(ExecResult::Incomplete);
                        return exec;
                    }
                    let Some(chr) = char::from_u32(chr) else {
                        exec.result = Some(ExecResult::Mismatch);
                        return exec;
                    };
                    if !is_full_ws(chr) {
                        exec.result = Some(ExecResult::Mismatch);
                        return exec;
                    }
                    let len = len as usize;
                    loop_exec.chunk = bytedata::const_or_bytes(
                        bytedata::const_slice(loop_exec.chunk, len..loop_exec.chunk.len()),
                        b"",
                    );
                    loop_exec.offset += len;
                    if loop_exec.chunk.is_empty() {
                        exec.result = if exec.flags.is_set(ExecFlag::Incomplete) {
                            Some(ExecResult::Incomplete)
                        } else {
                            None
                        };
                        return exec;
                    }
                }
            }
        }
    }
}
