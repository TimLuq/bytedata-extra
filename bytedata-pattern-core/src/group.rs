#[derive(Debug, Clone)]
pub struct Group<'a> {
    pattern: &'a crate::Pattern<'a>,
    /// The name of the capturing group, if any.
    capturing: bytedata::StringData<'a>,
    /// The range of repetitions for this group. `(0, u32::MAX)` means zero or more repetitions.
    repeat: (u32, u32),
}

impl<'a> Group<'a> {
    #[inline]
    #[must_use]
    pub const fn new<'b: 'a>(
        capturing: bytedata::StringData<'a>,
        pattern: &'a crate::Pattern<'b>,
    ) -> Self {
        Self {
            pattern,
            capturing,
            repeat: (1, 1),
        }
    }

    #[inline]
    #[must_use]
    pub const fn name(&self) -> Option<&str> {
        if self.capturing.is_empty() {
            None
        } else {
            Some(self.capturing.as_str())
        }
    }

    #[inline]
    #[must_use]
    pub const fn repeat(&self) -> Option<core::ops::RangeInclusive<u32>> {
        if self.repeat.0 == 1 && self.repeat.1 == 1 {
            None
        } else {
            Some(self.repeat.0..=self.repeat.1)
        }
    }

    #[inline]
    #[must_use]
    pub const fn with_repeat(mut self, range: core::ops::RangeInclusive<u32>) -> Self {
        self.repeat = (*range.start(), *range.end());
        self
    }
}

impl Group<'_> {
    pub(crate) const fn test_inner<'e>(
        &self,
        state: crate::test_exec::TestExec<'e>,
    ) -> crate::test_exec::TestExec<'e> {
        let mut state = state;
        let mut offset = state.offset;
        let mut count = 0;
        while count < self.repeat.1 {
            let state = self.pattern.test_inner(state);
            if let Some(crate::test_exec::ExecResult::Mismatch) = state.result {
                break;
            }
            if let Some(crate::test_exec::ExecResult::InvalidEncoding) = state.result {
                return state;
            }
            if state.result.is_none() {
                break;
            }
            count += 1;
            offset = state.offset;
        }
        state.result = if count < self.repeat.0 {
            Some(crate::test_exec::ExecResult::Mismatch)
        } else {
            None
        };
        state.offset = offset;
        state
    }

    pub const fn min_len(&self) -> usize {
        if self.repeat.0 == 0 {
            0
        } else {
            self.pattern.min_len() * self.repeat.0 as usize
        }
    }
}

impl std::fmt::Display for Group<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.capturing.is_empty() {
            f.write_fmt(format_args!("(?:{})", self.pattern))?;
        } else {
            f.write_fmt(format_args!("(?P<{}>{})", self.capturing, self.pattern))?;
        }
        if self.repeat.0 == 0 && self.repeat.1 == u32::MAX {
            f.write_str("*+")
        } else if self.repeat.0 == 1 && self.repeat.1 == 1 {
            Ok(())
        } else if self.repeat.0 == 0 && self.repeat.1 == 1 {
            f.write_str("?+")
        } else if self.repeat.0 == 1 && self.repeat.1 == u32::MAX {
            f.write_str("++")
        } else if self.repeat.0 == self.repeat.1 {
            f.write_fmt(format_args!("{{{}}}+", self.repeat.0))
        } else {
            f.write_fmt(format_args!("{{{},{}}}+", self.repeat.0, self.repeat.1))
        }
    }
}
