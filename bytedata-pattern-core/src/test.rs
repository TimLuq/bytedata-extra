use crate::{test_exec::{ExecFlag, ExecResult, TestExec}, CharacterClass, ContextClass};

#[derive(Clone, Copy)]
enum TestInner<'a> {
    Join(&'a [TestInner<'a>]),
    Verbatim(&'a str),
    Class(CharacterClass),
    Context(ContextClass),
    /// An inclusive match for all code points between the two characters.
    CharRange(char, char),
    /// A list of positive patterns. This succeeds if any of the tests match.
    OneOf(&'a [TestInner<'a>]),
    /// A list of positive patterns. This succeeds if any of the characters match.
    OneOfChars(&'a str),
    /// A list of negative patterns. This succeeds if none of the patterns match.
    NoneOf(&'a [TestInner<'a>]),
    /// The pattern is repeated `[min, max]` times.
    Repeat(&'a TestInner<'a>, (u32, u32)),
}

impl core::fmt::Debug for TestInner<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use TestInner::*;
        match self {
            Join(list) => {
                f.write_str("Test::join(&")?;
                f.debug_list().entries(list.iter()).finish()?;
                f.write_str(")")
            }
            Verbatim(text) => f.write_fmt(format_args!("Test::verbatim({:?})", *text)),
            Class(class) => f.write_fmt(format_args!("Test::char_class({:?})", class)),
            Context(cc) => f.write_fmt(format_args!("Test::context_class({:?})", cc)),
            CharRange(start, end) => f.write_fmt(format_args!("Test::char_range({:?}, {:?})", start, end)),
            OneOf(list) => {
                f.write_str("Test::one_of(&")?;
                f.debug_list().entries(list.iter()).finish()?;
                f.write_str(")")
            }
            OneOfChars(list) => f.write_fmt(format_args!("Test::one_of_chars({:?})", list)),
            NoneOf(list) => {
                f.write_str("Test::none_of(&")?;
                f.debug_list().entries(list.iter()).finish()?;
                f.write_str(")")
            }
            Repeat(inner, (min, max)) => f.write_fmt(format_args!("Test::repeat(&{:?}, ({:?}, {:?}))", inner, min, max)),
        }
    }
}

impl core::fmt::Display for TestInner<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        use TestInner::*;
        match self {
            Join(list) => {
                for test in list.iter() {
                    test.fmt(f)?;
                }
                Ok(())
            }
            Verbatim(text) => {
                let mut list = text.as_bytes();
                while !list.is_empty() {
                    let (chr, len) = bytedata::const_utf8_char_next(list);
                    if len == 0 {
                        break;
                    }
                    let chr = char::from_u32(chr).unwrap();
                    match chr {
                        '\\' | '[' | ']' | '^' | '-' | '(' | ')' | '{' | '}' | '|' | '*' | '+' | '?' | '.' | '$' => f.write_fmt(format_args!("\\{}", chr))?,
                        '\n' => f.write_str("\\n")?,
                        '\r' => f.write_str("\\r")?,
                        '\t' => f.write_str("\\t")?,
                        _ if (chr as u32) < 32 || (chr as u32 >= 0x7F && chr as u32 <= 0x9F) => f.write_fmt(format_args!("\\x{:02x}", chr as u32))?,
                        _ => {
                            let mut bytes = [0u8; 4];
                            f.write_str(chr.encode_utf8(&mut bytes))?;
                        }
                    }
                    list = bytedata::const_or_bytes(bytedata::const_slice(list, len as usize..list.len()), b"");
                }
                Ok(())
            }
            Class(class) => core::fmt::Display::fmt(class, f),
            Context(cc) => core::fmt::Display::fmt(cc, f),
            CharRange(start, end) => f.write_fmt(format_args!("[{:?}-{:?}]", start, end)),
            OneOf(list) => {
                f.write_str("(?:")?;
                let mut first = true;
                for test in list.iter() {
                    if first {
                        first = false;
                    } else {
                        f.write_str("|")?;
                    }
                    test.fmt(f)?;
                }
                f.write_str(")")
            }
            OneOfChars(list) => {
                f.write_str("[")?;
                let mut list = list.as_bytes();
                while !list.is_empty() {
                    let (chr, len) = bytedata::const_utf8_char_next(list);
                    if len == 0 {
                        break;
                    }
                    let chr = char::from_u32(chr).unwrap();
                    match chr {
                        '\\' | '[' | ']' | '^' | '-' | '$' => f.write_fmt(format_args!("\\{}", chr))?,
                        '\n' => f.write_str("\\n")?,
                        '\r' => f.write_str("\\r")?,
                        '\t' => f.write_str("\\t")?,
                        _ if (chr as u32) < 32 || (chr as u32 >= 0x7F && chr as u32 <= 0x9F) => f.write_fmt(format_args!("\\x{:02x}", chr as u32))?,
                        _ => {
                            let mut bytes = [0u8; 4];
                            f.write_str(chr.encode_utf8(&mut bytes))?;
                        }
                    }
                    list = bytedata::const_or_bytes(bytedata::const_slice(list, len as usize..list.len()), b"");
                }
                f.write_str("]")
            }
            NoneOf(list) => {
                f.write_str("(?!")?;
                let mut first = true;
                for test in list.iter() {
                    if first {
                        first = false;
                    } else {
                        f.write_str("|")?;
                    }
                    test.fmt(f)?;
                }
                f.write_str(")")
            }
            Repeat(inner, (min, max)) => {
                inner.fmt(f)?;
                if *min == 0 && *max == u32::MAX {
                    f.write_str("*+")
                } else if *min == 1 && *max == 1 {
                    Ok(())
                } else if *min == 0 && *max == 1 {
                    f.write_str("?+")
                } else if *min == 1 && *max == u32::MAX {
                    f.write_str("++")
                } else if *min == *max {
                    f.write_fmt(format_args!("{{{}}}+", min))
                } else {
                    f.write_fmt(format_args!("{{{},{}}}+", min, max))
                }
            }
        }
    }
}

impl TestInner<'_> {
    const fn run_test(self, mut exec: TestExec<'_>) -> TestExec {
        use TestInner::*;
        match self {
            Join(list) => {
                let Some((last, mut list)) = list.split_last() else {
                    return exec;
                };
                while let Some((f, rest)) = list.split_first() {
                    exec = f.run_test(exec);
                    if exec.result.is_none() {
                        list = rest;
                        continue;
                    }
                    return exec;
                }
                last.run_test(exec)
            }
            Verbatim(expected) => {
                if expected.is_empty() {
                    return exec;
                }
                let mut expected = expected.as_bytes();
                loop {
                    let (exp_chr, exp_len) = bytedata::const_utf8_char_next(expected);
                    if exp_len == 0 {
                        exec.result = if expected.is_empty() {
                            None
                        } else {
                            Some(ExecResult::InvalidEncoding)
                        };
                        return exec;
                    }
                    let (chk_chr, chk_len) = bytedata::const_utf8_char_next(exec.chunk);
                    if chk_len == 0 {
                        exec.result = Some(ExecResult::Incomplete);
                        return exec;
                    }
                    if exp_chr != chk_chr {
                        exec.result = Some(ExecResult::Mismatch);
                        return exec;
                    }
                    let chk_len = chk_len as usize;
                    expected = bytedata::const_or_bytes(bytedata::const_slice(expected, (exp_len as usize)..expected.len()), b"");
                    exec.chunk = bytedata::const_or_bytes(bytedata::const_slice(exec.chunk, chk_len..exec.chunk.len()), b"");
                    exec.offset += chk_len;
                    if let Some(chk_chr) = char::from_u32(chk_chr) {
                        if CharacterClass::Word.check_char(chk_chr) {
                            exec.flags.set(ExecFlag::PrevWord);
                        } else {
                            exec.flags.unset(ExecFlag::PrevWord);
                        }
                    } else {
                        exec.result = Some(ExecResult::InvalidEncoding);
                        return exec;
                    }
                }
            }
            Class(class) => class.run_test(exec),
            CharRange(start, end) => {
                let (chk_chr, chk_len) = bytedata::const_utf8_char_next(exec.chunk);
                if chk_len == 0 {
                    exec.result = Some(ExecResult::Incomplete);
                    return exec;
                }
                if let Some(chk_chr) = char::from_u32(chk_chr) {
                    if chk_chr as u32 >= start as u32 && chk_chr as u32 <= end as u32 {
                        let chk_len = chk_len as usize;
                        exec.chunk = bytedata::const_or_bytes(bytedata::const_slice(exec.chunk, chk_len..exec.chunk.len()), b"");
                        exec.offset += chk_len;
                        if CharacterClass::Word.check_char(chk_chr) {
                            exec.flags.set(ExecFlag::PrevWord);
                            exec
                        } else {
                            exec.flags.unset(ExecFlag::PrevWord);
                            exec
                        }
                    } else {
                        exec.result = Some(ExecResult::Mismatch);
                        exec
                    }
                } else {
                    exec.result = Some(ExecResult::InvalidEncoding);
                    exec
                }
            }
            Context(cc) => cc.run_test(exec),
            OneOf(list) => {
                let mut i = 0;
                while i < list.len() {
                    let f = &list[i];
                    i += 1;
                    let new_exec = f.run_test(exec);
                    if new_exec.result.is_none() || matches!(new_exec.result, Some(ExecResult::Incomplete)) {
                        return new_exec;
                    }
                }
                exec.result = Some(ExecResult::Mismatch);
                exec
            }
            OneOfChars(list) => {
                if exec.chunk.is_empty() {
                    exec.result = if exec.flags.is_set(ExecFlag::Incomplete) {
                        Some(ExecResult::Incomplete)
                    } else {
                        Some(ExecResult::Mismatch)
                    };
                    return exec;
                }
                let (chk_chr, chk_len) = bytedata::const_utf8_char_next(exec.chunk);
                if chk_len == 0 {
                    exec.result = Some(ExecResult::InvalidEncoding);
                    return exec;
                }
                let Some(chk_chr) = char::from_u32(chk_chr) else {
                    exec.result = Some(ExecResult::InvalidEncoding);
                    return exec;
                };
                let mut list = list.as_bytes();
                while !list.is_empty() {
                    let (exp_chr, exp_len) = bytedata::const_utf8_char_next(list);
                    if exp_len == 0 {
                        exec.result = Some(ExecResult::InvalidEncoding);
                        return exec;
                    }
                    if exp_chr == chk_chr as u32 {
                        let chk_len = chk_len as usize;
                        exec.chunk = bytedata::const_or_bytes(bytedata::const_slice(exec.chunk, chk_len..exec.chunk.len()), b"");
                        exec.offset += chk_len;
                        if CharacterClass::Word.check_char(chk_chr) {
                            exec.flags.set(ExecFlag::PrevWord);
                            exec.result = None;
                            return exec;
                        } else {
                            exec.flags.unset(ExecFlag::PrevWord);
                            exec.result = None;
                            return exec;
                        }
                    }
                    list = bytedata::const_or_bytes(bytedata::const_slice(list, exp_len as usize..list.len()), b"");
                }
                exec.result = Some(ExecResult::Mismatch);
                exec
            }
            NoneOf(list) => {
                let mut i = 0;
                while i < list.len() {
                    let f = &list[i];
                    i += 1;
                    let new_exec = f.run_test(exec);
                    if new_exec.result.is_none() {
                        exec.result = Some(ExecResult::Mismatch);
                        return exec;
                    }
                    if !matches!(new_exec.result, Some(ExecResult::Mismatch)) {
                        return new_exec;
                    }
                }
                exec.result = None;
                exec
            }
            Repeat(inner, (min, max)) => {
                let mut loop_exec = exec;
                let mut max = max;
                let mut min = min;
                while max != 0 {
                    let new_exec = inner.run_test(loop_exec);
                    if new_exec.result.is_none() {
                        loop_exec = new_exec;
                        min = min.saturating_sub(1);
                        max = max.saturating_sub(1);
                    } else {
                        break;
                    }
                }
                if min == 0 {
                    loop_exec.result = None;
                    loop_exec
                } else {
                    exec.result = Some(ExecResult::Mismatch);
                    exec
                }
            }
        }
    }
    
    pub const fn min_len(self) -> usize {
        match self {
            TestInner::Join(list) => {
                let mut min_len = 0;
                let mut i = 0;
                while i < list.len() {
                    let test = &list[i];
                    i += 1;
                    min_len += test.min_len();
                }
                min_len
            }
            TestInner::Verbatim(text) => text.len(),
            TestInner::Class(_) => 1,
            TestInner::CharRange(_, _) => 1,
            TestInner::Context(_) => 0,
            TestInner::OneOf(list) => {
                let mut min_len = usize::MAX;
                let mut i = 0;
                while i < list.len() {
                    let test = &list[i];
                    i += 1;
                    let len = test.min_len();
                    if len < min_len {
                        min_len = len;
                    }
                }
                min_len
            }
            TestInner::OneOfChars(list) => {
                let mut list = list.as_bytes();
                let mut min_len = 4usize;
                while !list.is_empty() {
                    let (_, len) = bytedata::const_utf8_char_next(list);
                    if len == 0 {
                        break;
                    }
                    if len == 1 {
                        return 1;
                    }
                    let len = len as usize;
                    if len < min_len {
                        min_len = len;
                    }
                    list = bytedata::const_or_bytes(bytedata::const_slice(list, len..list.len()), b"");
                }
                min_len
            }
            TestInner::NoneOf(_) => 0,
            TestInner::Repeat(_, (0, _)) => 0,
            TestInner::Repeat(inner, (min, _)) => inner.min_len() * (min as usize),
        }
    }
}

#[derive(Clone, Copy)]
#[repr(transparent)]
pub struct Test<'a> {
    inner: TestInner<'a>,
}

impl core::fmt::Debug for Test<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.inner, f)
    }
}

impl core::fmt::Display for Test<'_> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.inner, f)
    }
}

impl<'a> Test<'a> {
    #[inline]
    const fn slice_as_inner(slice: &'_ [Self]) -> &'_ [TestInner<'a>] {
        unsafe { core::mem::transmute(slice) }
    }

    /// If the `first` test succeeds the rest of the test string gets passed on to `second`.
    #[inline]
    pub const fn join(list: &'a [Test<'a>]) -> Self {
        debug_assert!(!list.is_empty(), "the list of tests must not be empty");
        Self {
            inner: TestInner::Join(Test::slice_as_inner(list)),
        }
    }

    /// An exact match of characters.
    #[inline]
    pub const fn verbatim(text: &'a str) -> Self {
        Self {
            inner: TestInner::Verbatim(text),
        }
    }

    /// A match against a specific character class.
    #[inline]
    pub const fn char_class(char_class: CharacterClass) -> Self {
        Self {
            inner: TestInner::Class(char_class),
        }
    }

    /// An inclusive match for all code points between the two characters.
    #[inline]
    pub const fn char_range(range_start: char, range_end: char) -> Self {
        debug_assert!(range_start <= range_end, "start of the char range must be before the end, this will match nothing");
        Self {
            inner: TestInner::CharRange(range_start, range_end),
        }
    }

    /// A combination of multiple tests. The first test that matches will be used.
    #[inline]
    pub const fn one_of(list: &'a [Test<'a>]) -> Self {
        debug_assert!(!list.is_empty(), "the list of tests must not be empty");
        Self {
            inner: TestInner::OneOf(Test::slice_as_inner(list)),
        }
    }

    /// A combination of multiple tests. The first test that matches will be used.
    #[inline]
    pub const fn one_of_chars(list: &'a str) -> Self {
        debug_assert!(!list.is_empty(), "the list of tests must not be empty");
        Self {
            inner: TestInner::OneOfChars(list),
        }
    }

    /// A combination of multiple negative tests. If one of the tests matches the pattern fails.
    pub const fn none_of(list: &'a [Test<'a>]) -> Self {
        debug_assert!(!list.is_empty(), "the list of tests must not be empty");
        Self {
            inner: TestInner::NoneOf(Test::slice_as_inner(list)),
        }
    }

    /// A test that possibly matches a repeated pattern.
    pub const fn repeat_any(test: &'a Test<'a>, range: core::ops::RangeToInclusive<u32>) -> Self {
        Self {
            inner: TestInner::Repeat(&test.inner, (0, range.end)),
        }
    }

    /// A test that possibly matches a repeated pattern at least a specific number of times.
    pub const fn repeat_many(test: &'a Test<'a>, range: core::ops::RangeInclusive<u32>) -> Self {
        Self {
            inner: TestInner::Repeat(&test.inner, (*range.start(), *range.end())),
        }
    }

    /// A test that possibly matches a repeated pattern a specific number of times.
    pub const fn repeat_exact(test: &'a Test<'a>, count: u32) -> Self {
        Self {
            inner: TestInner::Repeat(&test.inner, (count, count)),
        }
    }

    /// A test that possibly matches a repeated pattern a specific number of times.
    pub const fn context_class(context_class: ContextClass) -> Self {
        Self {
            inner: TestInner::Context(context_class),
        }
    }

    #[inline]
    pub const fn test(&self, data: &'a [u8]) -> Result<bool, crate::ExecError> {
        let min_len = self.min_len();
        let end = (data.len() + 1).saturating_sub(min_len);
        let mut offset = 0;
        let mut data = data;
        while offset < end {
            let exec = TestExec::new(data, offset);
            let exec = self.test_inner(exec);
            if exec.result.is_none() {
                return Ok(true);
            }
            let (_, skip_len) = bytedata::const_utf8_char_next(data);
            if skip_len == 0 {
                if data.is_empty() {
                    return Ok(false);
                }
                return Err(crate::ExecError::InvalidEncoding(offset));
            }
            let skip_len = skip_len as usize;
            offset += skip_len;
            data = bytedata::const_or_bytes(bytedata::const_slice(data, skip_len..data.len()), b"");
        }
        Ok(false)
    }

    #[inline]
    pub(crate) const fn test_inner<'e>(&self, state: TestExec<'e>) -> TestExec<'e> {
        self.inner.run_test(state)
    }

    #[inline]
    pub fn assert(&self, data: &'a [u8], message: &'a str) {
        let result = self.test(data);
        match result {
            Err(err) => panic!("assertion failed: {message} (/{self}/ {err:?})"),
            Ok(false) => panic!("assertion failed: {message} (/{self}/ NoMatch)"),
            Ok(true) => (),
        }
    }

    #[inline]
    pub fn assert_fail(&self, data: &'a [u8], message: &'a str) {
        if matches!(self.test(data), Ok(true)) {
            panic!("assertion failed: {} (!/{self}/)", message);
        }
    }

    #[inline]
    pub const fn min_len(&self) -> usize {
        self.inner.min_len()
    }
}
