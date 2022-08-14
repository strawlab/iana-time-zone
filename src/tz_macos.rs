use core_foundation_sys::base::{CFRelease, CFTypeRef};
use core_foundation_sys::string::{kCFStringEncodingUTF8, CFStringGetCStringPtr};
use core_foundation_sys::timezone::{CFTimeZoneCopySystem, CFTimeZoneGetName};

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    unsafe {
        if let Some(tz) = Dropping::new(CFTimeZoneCopySystem()) {
            if let Some(name) = Dropping::new(CFTimeZoneGetName(tz.0)) {
                let name = CFStringGetCStringPtr(name.0, kCFStringEncodingUTF8);
                if !name.is_null() {
                    let name = std::ffi::CStr::from_ptr(name);
                    if let Ok(name) = name.to_str() {
                        return Ok(name.to_owned());
                    }
                }
            }
        }
    }
    Err(crate::GetTimezoneError::OsError)
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
    unsafe fn new(v: *const T) -> Option<Self> {
        if v.is_null() {
            None
        } else {
            Some(Self(v))
        }
    }
}
