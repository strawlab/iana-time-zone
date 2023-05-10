pub fn get_timezone_inner() -> std::result::Result<String, crate::GetTimezoneError> {
    Err(crate::GetTimezoneError::OsError)
}

#[cfg(not(feature = "fallback"))]
compile_error!(
    "iana-time-zone is currently implemented for Linux, Window, MacOS, FreeBSD, NetBSD, \
    OpenBSD, Dragonfly, WebAssembly (browser), iOS, Illumos, Android, Solaris and Haiku. \
    If you are seeing this error on a supported platform, ensure the platform support \
    Cargo feature is enabled, e.g. platform-android, platform-apple, platform-haiku, \
    platform-wasm, platform-windows, or platform-all.",
);
