pub fn get_timezone_inner() -> std::result::Result<String, crate::GetTimezoneError> {
    Err(crate::GetTimezoneError::FailedParsingString)
}

compile_error!(
    "iana-time-zone is currently implemented for Linux, Window, MacOS, FreeBSD, NetBSD, \
    OpenBSD, Dragonfly, WebAssembly (browser), and iOS.",
);
