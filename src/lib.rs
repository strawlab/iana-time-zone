//! get the IANA time zone for the current system
//!
//! This small utility crate provides the
//! [`get_timezone()`](fn.get_timezone.html) function.
//!
//! ```rust
//! // Get the current time zone as a string.
//! let tz_str = iana_time_zone::get_timezone()?;
//! println!("The current time zone is: {}", tz_str);
//! # Ok::<(), iana_time_zone::GetTimezoneError>(())
//! ```
//!
//! The resulting string can be parsed to a
//! [`chrono-tz::Tz`](https://docs.rs/chrono-tz/latest/chrono_tz/enum.Tz.html)
//! variant like this:
//! ```ignore
//! let tz_str = iana_time_zone::get_timezone()?;
//! let tz: chrono_tz::Tz = tz_str.parse()?;
//! ```

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(missing_docs)]

#[cfg_attr(target_os = "linux", path = "tz_linux.rs")]
#[cfg_attr(target_os = "windows", path = "tz_windows.rs")]
#[cfg_attr(any(target_os = "macos", target_os = "ios"), path = "tz_macos.rs")]
#[cfg_attr(
    all(target_arch = "wasm32", not(target_os = "wasi")),
    path = "tz_wasm32.rs"
)]
#[cfg_attr(
    any(target_os = "freebsd", target_os = "dragonfly"),
    path = "tz_freebsd.rs"
)]
#[cfg_attr(
    any(target_os = "netbsd", target_os = "openbsd"),
    path = "tz_netbsd.rs"
)]
mod platform;
mod timezone;

use core::fmt;

pub use crate::timezone::{CovertError, Timezone};

/// Get the current IANA time zone as a string.
///
/// See the module-level documentatation for a usage example and more details
/// about this function.
#[inline]
pub fn get_timezone() -> Result<Timezone, GetTimezoneError> {
    platform::get_timezone_inner()
}

/// TODO
#[derive(Debug, Clone, Copy)]
pub enum GetTimezoneError {
    /// Platform-specific error from the operating system
    IoError(platform::Error),
    /// Failed to parse
    FailedParsingString,
}

impl fmt::Display for GetTimezoneError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GetTimezoneError::IoError(err) => write!(f, "platform-specific error: {:?}", err),
            GetTimezoneError::FailedParsingString => f.write_str("no valid timezone found"),
        }
    }
}

impl From<platform::Error> for GetTimezoneError {
    #[inline]
    fn from(value: platform::Error) -> Self {
        Self::IoError(value)
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for GetTimezoneError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_current() {
        println!("current: {}", get_timezone().unwrap());
    }
}
