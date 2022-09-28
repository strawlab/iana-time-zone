use core_foundation_sys::base::{Boolean, CFRange, CFRelease, CFTypeRef};
use core_foundation_sys::string::{
    kCFStringEncodingUTF8, CFStringGetBytes, CFStringGetCStringPtr, CFStringGetLength, CFStringRef,
};
use core_foundation_sys::timezone::{CFTimeZoneCopySystem, CFTimeZoneGetName, CFTimeZoneRef};

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    get_timezone().ok_or(crate::GetTimezoneError::OsError)
}

#[inline]
fn get_timezone() -> Option<String> {
    // The longest name in the IANA time zone database is 25 ASCII characters long.
    const MAX_LEN: usize = 32;
    let mut buf = [0; MAX_LEN];

    // Get system time zone, and borrow its name.
    let tz = SystemTimeZone::new()?;
    let name = tz.name()?;

    // If the name is encoded in UTF-8, copy it directly.
    let name = if let Some(name) = name.as_utf8() {
        name
    } else {
        // Otherwise convert the name to UTF-8.
        name.to_utf8(&mut buf)?
    };

    if name.len() < 1 || name.len() >= MAX_LEN {
        // The name should not be empty, or excessively long.
        None
    } else {
        Some(name.to_owned())
    }
}

struct SystemTimeZone(CFTimeZoneRef);

struct StringRef<'a> {
    string: CFStringRef,
    phantom: core::marker::PhantomData<&'a SystemTimeZone>,
}

impl Drop for SystemTimeZone {
    fn drop(&mut self) {
        // SAFETY: `SystemTimeZone` is only ever created with a valid `CFTimeZoneRef`.
        unsafe { CFRelease(self.0 as CFTypeRef) };
    }
}

impl SystemTimeZone {
    fn new() -> Option<Self> {
        // SAFETY: No invariants to uphold. We'll release the pointer when we don't need it anymore.
        let v = unsafe { CFTimeZoneCopySystem() };
        if v.is_null() {
            None
        } else {
            Some(SystemTimeZone(v))
        }
    }

    fn name(&self) -> Option<StringRef> {
        // SAFETY: `SystemTimeZone` is only ever created with a valid `CFTimeZoneRef`.
        let string = unsafe { CFTimeZoneGetName(self.0) };
        if string.is_null() {
            None
        } else {
            Some(StringRef {
                string,
                phantom: core::marker::PhantomData,
            })
        }
    }
}

impl<'a> StringRef<'a> {
    fn as_utf8(&self) -> Option<&'a str> {
        // SAFETY: `StringRef` is only ever created with a valid `CFStringRef`.
        let v = unsafe { CFStringGetCStringPtr(self.string, kCFStringEncodingUTF8) };
        if !v.is_null() {
            // SAFETY: `CFStringGetCStringPtr()` returns NUL-terminated strings.
            let v = unsafe { std::ffi::CStr::from_ptr(v) };
            if let Ok(v) = v.to_str() {
                return Some(v);
            }
        }
        None
    }

    fn to_utf8<'b>(&self, buf: &'b mut [u8]) -> Option<&'b str> {
        // SAFETY: `StringRef` is only ever created with a valid `CFStringRef`.
        let length = unsafe { CFStringGetLength(self.string) };

        let mut buf_bytes = 0;
        let range = CFRange {
            location: 0,
            length,
        };

        let converted_bytes = unsafe {
            // SAFETY: `StringRef` is only ever created with a valid `CFStringRef`.
            CFStringGetBytes(
                self.string,
                range,
                kCFStringEncodingUTF8,
                b'\0',
                false as Boolean,
                buf.as_mut_ptr(),
                buf.len() as isize,
                &mut buf_bytes,
            )
        };
        if converted_bytes != length || buf_bytes < 0 || buf_bytes as usize > buf.len() {
            None
        } else {
            std::str::from_utf8(&buf[..buf_bytes as usize]).ok()
        }
    }
}
