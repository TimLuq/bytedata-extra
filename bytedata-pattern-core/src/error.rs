
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ExecError {
    /// The pattern may or may not match the input, but the input is too short to tell.
    /// This error is only returned if the input was marked as incomplete.
    Incomplete,

    /// The input contained an invalid utf-8 character.
    /// This should only happen if the input is bytes and not a validated string.
    InvalidEncoding(usize),
}
