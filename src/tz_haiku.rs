#[cxx::bridge(namespace = "tz_haiku")]
mod ffi {
    // SAFETY: in here "unsafe" is simply part of the syntax
    unsafe extern "C++" {
        include!("iana-time-zone/include/impl_haiku.h");

        fn get_tz(buf: &mut [u8]) -> usize;
    }
}

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    // The longest name in the IANA time zone database is 25 ASCII characters long.
    let mut buf = [0u8; 32];
    let len = ffi::get_tz(&mut buf);
    // The name should not be empty, or excessively long.
    match buf.get(..len) {
        None | Some(b"") => Err(crate::GetTimezoneError::OsError),
        Some(s) => {
            let s = std::str::from_utf8(s).map_err(|_| crate::GetTimezoneError::OsError)?;
            Ok(s.to_owned())
        }
    }
}
