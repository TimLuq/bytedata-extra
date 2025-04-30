#[expect(clippy::module_inception)]
mod single_byte;
pub use single_byte::*;

#[cfg(feature = "ibm866")]
mod ibm866;
#[cfg(feature = "ibm866")]
pub use ibm866::*;

#[cfg(feature = "koi8-r")]
mod koi8_r;
#[cfg(feature = "koi8-r")]
pub use koi8_r::*;

#[cfg(feature = "koi8-u")]
mod koi8_u;
#[cfg(feature = "koi8-u")]
pub use koi8_u::*;

#[cfg(feature = "macintosh")]
mod macintosh;
#[cfg(feature = "macintosh")]
pub use macintosh::*;

#[cfg(feature = "x-mac-cyrillic")]
mod x_mac_cyrillic;
#[cfg(feature = "x-mac-cyrillic")]
pub use x_mac_cyrillic::*;

#[cfg(feature = "x-user-defined")]
mod x_user_defined;
#[cfg(feature = "x-user-defined")]
pub use x_user_defined::*;
