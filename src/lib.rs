//! get the IANA time zone for the current system
//!
//! ```
//! extern crate iana_time_zone;
//! println!("current: {}", iana_time_zone::get_timezone().unwrap());
//! ```

#[deny(rust_2018_idioms)]

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::get_timezone as get_timezone;

#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "windows")]
pub use windows::get_timezone as get_timezone;

#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "macos")]
pub use macos::get_timezone as get_timezone;

/// Error types
#[derive(Debug)]
pub enum GetTimezoneError {
    /// Failed to parse
    FailedParsingString,
    /// Wrapped IO error
    IoError(std::io::Error),
    #[cfg(target_os = "windows")]
    /// Windows Runtime Error (`windows` target OS only)
    WinRtError(winrt::Error),
}

impl std::error::Error for GetTimezoneError {}

impl std::fmt::Display for GetTimezoneError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {

        use GetTimezoneError::*;
        let descr = match self {
            &FailedParsingString => "GetTimezoneError::FailedParsingString",
            &IoError(_) => "GetTimezoneError::IoError(_)",
            #[cfg(target_os = "windows")]
            &WinRtError(_) => "WinRtError(_)",
        };

        write!(f, "{}", descr)
    }
}

impl std::convert::From<std::io::Error> for GetTimezoneError {
    fn from(orig: std::io::Error) -> Self {
        GetTimezoneError::IoError(orig)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_current() {
        println!("current: {}", get_timezone().unwrap());
    }
}
