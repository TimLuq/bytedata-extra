#[cfg(feature = "gb18030")]
mod gb18030;
#[cfg(feature = "gb18030")]
pub use gb18030::*;

#[cfg(feature = "gbk")]
mod gbk;
#[cfg(feature = "gbk")]
pub use gbk::*;


#[cfg(feature = "big5")]
mod big5;
#[cfg(feature = "big5")]
pub use big5::*;
