
#[non_exhaustive]
pub enum Error {
    EndOfData,
    InvalidData,
    Io { io_error: std::io::Error },
}
