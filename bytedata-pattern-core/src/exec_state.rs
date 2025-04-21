use crate::flags::U8Flags;


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum ExecResult {
    Incomplete = 0b0000_0001,
    Mismatch = 0b0000_0010,
    InvalidEncoding = 0b0000_0100,
}

impl U8Flags<ExecResult> {
    #[inline]
    #[must_use]
    pub(crate) const fn is_ok(self) -> bool {
        self.flags == 0
    }
    #[inline]
    #[must_use]
    pub(crate) const fn is_err(self) -> bool {
        self.flags != 0
    }
    #[inline]
    #[must_use]
    pub(crate) const fn is_set(self, flag: ExecResult) -> bool {
        self.flags & (flag as u8) != 0
    }
}

impl ExecResult {
    pub(crate) const fn as_flags(self) -> U8Flags<ExecResult> {
        U8Flags::new().set_u8(self as u8)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub(crate) enum ExecFlag {
    Incomplete = 0b0000_0001,
    PrevWord = 0b0001_0000,
}

impl U8Flags<ExecFlag> {
    #[inline]
    pub(crate) const fn is_ok(self) -> bool {
        self.flags == 0
    }
    #[inline]
    pub(crate) const fn is_err(self) -> bool {
        self.flags != 0
    }
    #[inline]
    pub(crate) const fn set(&mut self, flag: ExecFlag) {
        self.flags |= flag as u8;
    }
    #[inline]
    pub(crate) const fn unset(&mut self, flag: ExecFlag) {
        self.flags &= !(flag as u8);
    }
    #[inline]
    #[must_use]
    pub(crate) const fn is_set(self, flag: ExecFlag) -> bool {
        self.flags & (flag as u8) != 0
    }
}

impl ExecFlag {
    pub(crate) const fn as_flags(self) -> U8Flags<ExecFlag> {
        U8Flags::new().set_u8(self as u8)
    }
}

#[derive(Clone, Copy)]
pub(crate) struct TestExec<'e> {
    pub(crate) chunk: &'e [u8],
    pub(crate) offset: usize,
    pub(crate) flags: U8Flags<ExecFlag>,
    pub(crate) result: Option<ExecResult>,
}

impl TestExec<'_> {
    #[inline]
    #[must_use]
    pub(crate) const fn is_ok(&self) -> bool {
        self.result.is_none()
    }
    #[inline]
    #[must_use]
    pub(crate) const fn is_err(&self) -> bool {
        self.result.is_some()
    }
}

impl<'a> TestExec<'a> {
    pub(crate) const fn new(chunk: &'a [u8], offset: usize) -> Self {
        Self {
            chunk,
            offset,
            flags: U8Flags::new(),
            result: None,
        }
    }
}
