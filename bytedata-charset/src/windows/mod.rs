//! Windows code pages.

#[cfg(feature = "windows-874")]
mod win_874;
#[cfg(feature = "windows-874")]
pub use win_874::*;

#[cfg(feature = "windows-1250")]
mod win_1250;
#[cfg(feature = "windows-1250")]
pub use win_1250::*;

#[cfg(feature = "windows-1251")]
mod win_1251;
#[cfg(feature = "windows-1251")]
pub use win_1251::*;

#[cfg(feature = "windows-1252")]
mod win_1252;
#[cfg(feature = "windows-1252")]
pub use win_1252::*;

#[cfg(feature = "windows-1253")]
mod win_1253;
#[cfg(feature = "windows-1253")]
pub use win_1253::*;

#[cfg(feature = "windows-1254")]
mod win_1254;
#[cfg(feature = "windows-1254")]
pub use win_1254::*;

#[cfg(feature = "windows-1255")]
mod win_1255;
#[cfg(feature = "windows-1255")]
pub use win_1255::*;

#[cfg(feature = "windows-1256")]
mod win_1256;
#[cfg(feature = "windows-1256")]
pub use win_1256::*;

#[cfg(feature = "windows-1257")]
mod win_1257;
#[cfg(feature = "windows-1257")]
pub use win_1257::*;

#[cfg(feature = "windows-1258")]
mod win_1258;
#[cfg(feature = "windows-1258")]
pub use win_1258::*;
