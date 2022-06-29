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

#[cfg(target_os = "linux")]
mod tz_linux;
#[cfg(target_os = "linux")]
use tz_linux as platform;

#[cfg(target_os = "windows")]
mod tz_windows;
#[cfg(target_os = "windows")]
use tz_windows as platform;

#[cfg(target_os = "macos")]
mod tz_macos;
#[cfg(target_os = "macos")]
use tz_macos as platform;

#[cfg(target_arch = "wasm32")]
mod tz_wasm32;
#[cfg(target_arch = "wasm32")]
use tz_wasm32 as platform;

#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
mod tz_freebsd;
#[cfg(any(target_os = "freebsd", target_os = "dragonfly"))]
use tz_freebsd as platform;

#[cfg(any(target_os = "netbsd", target_os = "openbsd"))]
mod tz_netbsd;
#[cfg(any(target_os = "netbsd", target_os = "openbsd"))]
use tz_netbsd as platform;

/// Error types
#[derive(Debug)]
pub enum GetTimezoneError {
    /// Failed to parse
    FailedParsingString,
    /// Wrapped IO error
    IoError(std::io::Error),
    /// Platform-specific error from the operating system
    OsError,
}

impl std::error::Error for GetTimezoneError {}

impl std::fmt::Display for GetTimezoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        use GetTimezoneError::*;
        let descr = match self {
            FailedParsingString => "GetTimezoneError::FailedParsingString",
            IoError(_) => "GetTimezoneError::IoError(_)",
            OsError => "OsError",
        };

        write!(f, "{}", descr)
    }
}

impl std::convert::From<std::io::Error> for GetTimezoneError {
    fn from(orig: std::io::Error) -> Self {
        GetTimezoneError::IoError(orig)
    }
}

/// Get the current IANA time zone as a string.
///
/// See the module-level documentatation for a usage example and more details
/// about this function.
pub fn get_timezone() -> std::result::Result<String, crate::GetTimezoneError> {
    platform::get_timezone_inner()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_current() {
        println!("current: {}", get_timezone().unwrap());
    }
}
