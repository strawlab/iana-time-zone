#[cxx::bridge(namespace = "tz_haiku")]
mod ffi {
    unsafe extern "C++" {
        include!("iana-time-zone/include/impl_haiku.h");

        /// SAFETY: `buf` must be valid and be at least `buf_len` bytes long
        unsafe fn get_tz(buf: *mut u8, buf_len: usize) -> usize;
    }
}

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    // The longest name in the IANA time zone database is 25 ASCII characters long.
    let mut buf = [0u8; 32];
    // SAFETY: the buffer is valid and called function ensures that sizeof(u8) == sizeof(c_char)
    let len = unsafe { ffi::get_tz(buf.as_mut_ptr(), buf.len()) };
    // The name should not be empty, or excessively long.
    if len > 0 && len < buf.len() {
        let s = std::str::from_utf8(&buf[..len]).map_err(|_| crate::GetTimezoneError::OsError)?;
        Ok(s.to_owned())
    } else {
        Err(crate::GetTimezoneError::OsError)
    }
}
