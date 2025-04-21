mod charclass;
mod contextclass;
mod error;
mod flags;

mod exec_state;
mod test_exec;

mod group;
mod test;

pub use charclass::CharacterClass;
pub use contextclass::ContextClass;
pub use error::ExecError;
pub use group::Group;
pub use test::Test;

/// A pattern that can be tested against input.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum Pattern<'a> {
    Test(Test<'a>),
    Group(Group<'a>),
    Either(&'a [Pattern<'a>]),
    Join(&'a [Pattern<'a>]),
}

impl core::fmt::Display for Pattern<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Pattern::Test(test) => core::fmt::Display::fmt(test, f),
            Pattern::Group(group) => core::fmt::Display::fmt(group, f),
            Pattern::Either(patterns) => {
                f.write_str("(?:")?;
                let mut fst = true;
                for pattern in patterns.iter() {
                    if fst {
                        fst = false;
                    } else {
                        f.write_str("|")?;
                    }
                    pattern.fmt(f)?;
                }
                f.write_str(")")?;
                Ok(())
            }
            Pattern::Join(patterns) => {
                for pattern in patterns.iter() {
                    pattern.fmt(f)?;
                }
                Ok(())
            }
        }
    }
}

impl Pattern<'_> {
    /// Test if the pattern matches the input.
    /// This is a convenience method for [`Self::test_bytes_complete`].
    #[inline]
    pub const fn test_str(&self, input: &str) -> Result<bool, ExecError> {
        self.test_bytes_complete(input.as_bytes())
    }
    /// Test if the pattern matches the input.
    /// This is a convenience method for [`Self::test_bytes_complete`].
    #[inline]
    pub const fn test_bytes(&self, input: &[u8]) -> Result<bool, ExecError> {
        self.test_bytes_complete(input)
    }

    /// Get the minimum length of an input that could possibly match the pattern.
    pub const fn min_len(&self) -> usize {
        match self {
            Pattern::Test(test) => test.min_len(),
            Pattern::Group(group) => group.min_len(),
            Pattern::Either(patterns) => {
                let mut min_len = usize::MAX;
                let mut i = 0;
                while i < patterns.len() {
                    let pattern = &patterns[i];
                    i += 1;
                    let len = pattern.min_len();
                    if len < min_len {
                        min_len = len;
                    }
                }
                min_len
            }
            Pattern::Join(patterns) => {
                let mut min_len = 0;
                let mut i = 0;
                while i < patterns.len() {
                    let pattern = &patterns[i];
                    i += 1;
                    min_len += pattern.min_len();
                }
                min_len
            }
        }
    }

    /// Test if the pattern matches the input.
    #[inline]
    pub const fn test_bytes_complete(&self, input: &[u8]) -> Result<bool, ExecError> {
        let state = test_exec::TestExec::new(input, 0);
        let state = self.test_inner(state);
        match state.result {
            None => Ok(true),
            Some(test_exec::ExecResult::Mismatch) => Ok(false),
            Some(test_exec::ExecResult::Incomplete) => Ok(false),
            Some(test_exec::ExecResult::InvalidEncoding) => Err(ExecError::InvalidEncoding(state.offset)),
        }
    }

    #[inline]
    pub(crate) const fn test_inner<'e>(&self, state: test_exec::TestExec<'e>) -> test_exec::TestExec<'e> {
        match self {
            Pattern::Test(test) => test.test_inner(state),
            Pattern::Group(group) => group.test_inner(state),
            Pattern::Either(patterns) => {
                let mut i = 0;
                while i < patterns.len() {
                    let pattern = &patterns[i];
                    let state = pattern.test_inner(state);
                    match state.result {
                        Some(test_exec::ExecResult::Mismatch) => {
                            i += 1;
                            continue
                        }
                        _ => return state,
                    }
                }
                let mut state = state;
                state.result = Some(test_exec::ExecResult::Mismatch);
                state
            }
            Pattern::Join(patterns) => {
                let mut loop_state = state;
                let mut i = 0;
                while i < patterns.len() {
                    let pattern = &patterns[i];
                    let state = pattern.test_inner(loop_state);
                    match state.result {
                        None => {
                            loop_state = state;
                            i += 1;
                            continue
                        }
                        _ => return state,
                    }
                }
                loop_state
            }
        }
    }
}
