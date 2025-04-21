
//! ## ISO-8859 Character Encodings
//! 
//! The ISO-8859 character encodings are a series of 8-bit character encodings for writing in Latin scripts.
//! The encodings are designed to be compatible with ASCII-7, but also include additional characters for writing in various European languages.
//! 
//! Any new project should use UTF-8 instead of these encodings, but they are still in use in many legacy systems.
//! As such, this library provides support for these encodings.
//! When writing in a context which defines which encoding is used, such as XML or HTML, it is recommended to always use UTF-8 instead of ISO-8859 and only use these charsets to read external or existing data.

#[cfg(feature = "iso-8859-1")]
mod iso_1;
#[cfg(feature = "iso-8859-1")]
pub use iso_1::*;
#[cfg(feature = "iso-8859-2")]
mod iso_2;
#[cfg(feature = "iso-8859-2")]
pub use iso_2::*;
#[cfg(feature = "iso-8859-3")]
mod iso_3;
#[cfg(feature = "iso-8859-3")]
pub use iso_3::*;
#[cfg(feature = "iso-8859-4")]
mod iso_4;
#[cfg(feature = "iso-8859-4")]
pub use iso_4::*;
#[cfg(feature = "iso-8859-5")]
mod iso_5;
#[cfg(feature = "iso-8859-5")]
pub use iso_5::*;
#[cfg(feature = "iso-8859-6")]
mod iso_6;
#[cfg(feature = "iso-8859-6")]
pub use iso_6::*;
#[cfg(feature = "iso-8859-7")]
mod iso_7;
#[cfg(feature = "iso-8859-7")]
pub use iso_7::*;
#[cfg(feature = "iso-8859-8")]
mod iso_8;
#[cfg(feature = "iso-8859-8")]
pub use iso_8::*;
#[cfg(feature = "iso-8859-9")]
mod iso_9;
#[cfg(feature = "iso-8859-9")]
pub use iso_9::*;
#[cfg(feature = "iso-8859-10")]
mod iso_10;
#[cfg(feature = "iso-8859-10")]
pub use iso_10::*;
#[cfg(feature = "iso-8859-11")]
mod iso_11;
#[cfg(feature = "iso-8859-11")]
pub use iso_11::*;
#[cfg(feature = "iso-8859-13")]
mod iso_13;
#[cfg(feature = "iso-8859-13")]
pub use iso_13::*;
#[cfg(feature = "iso-8859-14")]
mod iso_14;
#[cfg(feature = "iso-8859-14")]
pub use iso_14::*;
#[cfg(feature = "iso-8859-15")]
mod iso_15;
#[cfg(feature = "iso-8859-15")]
pub use iso_15::*;
#[cfg(feature = "iso-8859-16")]
mod iso_16;
#[cfg(feature = "iso-8859-16")]
pub use iso_16::*;

/// The ISO-8859-1 charset.
pub(crate) const ISO_8859_1_CHARSET: [char; 128] = [
    '\u{0080}', '\u{0081}', '\u{0082}', '\u{0083}', '\u{0084}', '\u{0085}', '\u{0086}', '\u{0087}',
    '\u{0088}', '\u{0089}', '\u{008A}', '\u{008B}', '\u{008C}', '\u{008D}', '\u{008E}', '\u{008F}',
    '\u{0090}', '\u{0091}', '\u{0092}', '\u{0093}', '\u{0094}', '\u{0095}', '\u{0096}', '\u{0097}',
    '\u{0098}', '\u{0099}', '\u{009A}', '\u{009B}', '\u{009C}', '\u{009D}', '\u{009E}', '\u{009F}',
    '\u{00A0}', '\u{00A1}', '\u{00A2}', '\u{00A3}', '\u{00A4}', '\u{00A5}', '\u{00A6}', '\u{00A7}',
    '\u{00A8}', '\u{00A9}', '\u{00AA}', '\u{00AB}', '\u{00AC}', '\u{00AD}', '\u{00AE}', '\u{00AF}',
    '\u{00B0}', '\u{00B1}', '\u{00B2}', '\u{00B3}', '\u{00B4}', '\u{00B5}', '\u{00B6}', '\u{00B7}',
    '\u{00B8}', '\u{00B9}', '\u{00BA}', '\u{00BB}', '\u{00BC}', '\u{00BD}', '\u{00BE}', '\u{00BF}',
    '\u{00C0}', '\u{00C1}', '\u{00C2}', '\u{00C3}', '\u{00C4}', '\u{00C5}', '\u{00C6}', '\u{00C7}',
    '\u{00C8}', '\u{00C9}', '\u{00CA}', '\u{00CB}', '\u{00CC}', '\u{00CD}', '\u{00CE}', '\u{00CF}',
    '\u{00D0}', '\u{00D1}', '\u{00D2}', '\u{00D3}', '\u{00D4}', '\u{00D5}', '\u{00D6}', '\u{00D7}',
    '\u{00D8}', '\u{00D9}', '\u{00DA}', '\u{00DB}', '\u{00DC}', '\u{00DD}', '\u{00DE}', '\u{00DF}',
    '\u{00E0}', '\u{00E1}', '\u{00E2}', '\u{00E3}', '\u{00E4}', '\u{00E5}', '\u{00E6}', '\u{00E7}',
    '\u{00E8}', '\u{00E9}', '\u{00EA}', '\u{00EB}', '\u{00EC}', '\u{00ED}', '\u{00EE}', '\u{00EF}',
    '\u{00F0}', '\u{00F1}', '\u{00F2}', '\u{00F3}', '\u{00F4}', '\u{00F5}', '\u{00F6}', '\u{00F7}',
    '\u{00F8}', '\u{00F9}', '\u{00FA}', '\u{00FB}', '\u{00FC}', '\u{00FD}', '\u{00FE}', '\u{00FF}',
];
