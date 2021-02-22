//! get the IANA time zone for the current system
//!
//! This small utility crate provides the
//! [`get_timezone()`](fn.get_timezone.html) function.
//!
//! ```
//! extern crate iana_time_zone;
//! println!("current: {}", iana_time_zone::get_timezone().unwrap());
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

/// Get the IANA time zone as a string.
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
