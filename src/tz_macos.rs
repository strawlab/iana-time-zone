use core_foundation_sys::base::{Boolean, CFRange, CFRelease, CFTypeRef};
use core_foundation_sys::string::{
    kCFStringEncodingUTF8, CFStringGetBytes, CFStringGetCStringPtr, CFStringGetLength,
};
use core_foundation_sys::timezone::{CFTimeZoneCopySystem, CFTimeZoneGetName};

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    get_timezone().ok_or(crate::GetTimezoneError::OsError)
}

#[inline]
fn get_timezone() -> Option<String> {
    // The longest name in the IANA time zone database is 25 ASCII characters long.
    const MAX_LEN: usize = 32;

    // Get system time zone, and borrow its name.

    // SAFETY: `Dropping::new()` would have returned `None` if the call to `CFTimeZoneCopySystem()` had failed.
    let tz = Dropping::new(unsafe { CFTimeZoneCopySystem() })?;
    let name = unsafe { CFTimeZoneGetName(tz.0) };
    if name.is_null() {
        return None;
    }

    {
        // New block prevents any variables escaping this scope.

        // If the name is encoded in UTF-8, copy it directly.

        // SAFETY: we tested that `name` is not null, so we know that the result
        // of `CFTimeZoneGetName()` is valid
        let cstr_ptr = unsafe { CFStringGetCStringPtr(name, kCFStringEncodingUTF8) };
        if !cstr_ptr.is_null() {
            // SAFETY: 1) `cstr_ptr` must contain valid nul terminator. 2)
            // `cstr_ptr` must be valid for reads of bytes up to and including
            // the null terminator. 3) `cstr_ptr` must remain valid for lifetime
            // of `cstr`. We assume #1 and #2 are true based on the result from
            // CFStringGetCStringPtr. We ensure #3 by cloning the underlying &str
            // to a String.
            let cstr = unsafe { std::ffi::CStr::from_ptr(cstr_ptr) };
            if let Ok(name) = cstr.to_str() {
                return Some(name.to_owned());
            }
        }
    }

    // Otherwise convert the name to UTF-8.
    let mut buf = [0; MAX_LEN];
    let mut buf_bytes = 0;
    let range = CFRange {
        location: 0,
        // SAFETY: we tested that `name` is not null, so we know that the result of `CFTimeZoneGetName()` is valid
        length: unsafe { CFStringGetLength(name) },
    };
    if unsafe {
        CFStringGetBytes(
            name,
            range,
            kCFStringEncodingUTF8,
            b'\0',
            false as Boolean,
            buf.as_mut_ptr(),
            buf.len() as isize,
            &mut buf_bytes,
        )
    } != range.length
    {
        // Could not convert the name.
        None
    } else if buf_bytes < 1 || buf_bytes >= MAX_LEN as isize {
        // The name should not be empty, or excessively long.
        None
    } else {
        // Convert the name to a `String`.
        let name = core::str::from_utf8(&buf[..buf_bytes as usize]).ok()?;
        Some(name.to_owned())
    }
}

struct Dropping<T>(*const T);

impl<T> Drop for Dropping<T> {
    #[inline]
    fn drop(&mut self) {
        unsafe { CFRelease(self.0 as CFTypeRef) };
    }
}

impl<T> Dropping<T> {
    #[inline]
    fn new(v: *const T) -> Option<Self> {
        if v.is_null() {
            None
        } else {
            Some(Dropping(v))
        }
    }
}
