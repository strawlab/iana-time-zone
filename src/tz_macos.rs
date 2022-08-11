use core_foundation_sys::base::{CFRelease, CFTypeRef};
use core_foundation_sys::string::{kCFStringEncodingUTF8, CFStringGetCStringPtr};
use core_foundation_sys::timezone::{CFTimeZoneCopySystem, CFTimeZoneGetName};

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    unsafe {
        Dropping::new(CFTimeZoneCopySystem())
            .and_then(|tz| Dropping::new(CFTimeZoneGetName(tz.0)))
            .and_then(|name| {
                let name = CFStringGetCStringPtr(name.0, kCFStringEncodingUTF8);
                if name.is_null() {
                    None
                } else {
                    Some(name)
                }
            })
            .and_then(|name| std::ffi::CStr::from_ptr(name).to_str().ok())
            .map(|name| name.to_owned())
            .ok_or(crate::GetTimezoneError::OsError)
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
    unsafe fn new(v: *const T) -> Option<Self> {
        if v.is_null() {
            None
        } else {
            Some(Self(v))
        }
    }
}
