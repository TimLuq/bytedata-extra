use bytedata::StringData;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location<'a> {
    pub(crate) path: StringData<'a>,
    pub(crate) line: u64,
    pub(crate) column: u64,
    pub(crate) byte: u64,
}
impl<'a> Location<'a> {
    pub const fn new(path: StringData<'a>, line: u64, column: u64, byte: u64) -> Self {
        Self { path, line, column, byte }
    }
    pub const fn unknown() -> Self {
        Self { path: StringData::empty(), line: 0, column: 0, byte: 0 }
    }
    pub const fn path(&self) -> &StringData<'a> {
        &self.path
    }
    pub const fn line(&self) -> u64 {
        self.line
    }
    pub const fn column(&self) -> u64 {
        self.column
    }
    pub const fn byte(&self) -> u64 {
        self.byte
    }
}