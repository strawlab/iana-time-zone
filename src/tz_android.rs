use std::ffi::CStr;
use std::sync::Once;

use android_system_properties::AndroidSystemProperties;

static INITALIZED: Once = Once::new();
static mut PROPERTIES: Option<AndroidSystemProperties> = None;

// From https://android.googlesource.com/platform/ndk/+/android-4.2.2_r1.2/docs/system/libc/OVERVIEW.html
// The system property named 'persist.sys.timezone' contains the name of the current timezone.
// SAFETY: the key is NUL-terminated and there are no other NULs
const KEY: &CStr = unsafe { CStr::from_bytes_with_nul_unchecked(b"persist.sys.timezone\0") };

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    INITALIZED.call_once(|| {
        let properties = AndroidSystemProperties::new();
        // SAFETY: `INITALIZED` is synchronizing. The variable is only assigned to once.
        unsafe { PROPERTIES = Some(properties) };
    });

    // SAFETY: `INITALIZED` is synchronizing. The variable is only assigned to once.
    let properties = unsafe { PROPERTIES.as_ref() };

    properties
        .and_then(|properties| properties.get_from_cstr(KEY))
        .ok_or(crate::GetTimezoneError::OsError)
}
