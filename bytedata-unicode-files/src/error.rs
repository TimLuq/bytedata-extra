
pub enum ParseError {
    /// Encountered a value that was expected to be a codepoint which turned out to be invalid.
    /// `(line, col)`.
    InvalidCodepoint(u64, usize),

    /// Encountered a line which contained an unexpected line prefix.
    UnexpectedLineType(u64),

    /// Encountered a line which contained an unexpected line prefix.
    UnexpectedEndOfFile(u64),
}