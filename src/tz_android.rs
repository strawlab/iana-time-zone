use std::ffi::CStr;
use std::sync::OnceLock;

use android_system_properties::AndroidSystemProperties;

// The system property named 'persist.sys.timezone' contains the name of the
// current timezone.
//
// From https://android.googlesource.com/platform/bionic/+/gingerbread-release/libc/docs/OVERVIEW.TXT#79:
//
// > The name of the current timezone is taken from the TZ environment variable,
// > if defined. Otherwise, the system property named 'persist.sys.timezone' is
// > checked instead.
const TIMEZONE_PROPERTY_NAME: &CStr = c"persist.sys.timezone";

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    get_properties()
        .and_then(|properties| properties.get_from_cstr(TIMEZONE_PROPERTY_NAME))
        .ok_or(crate::GetTimezoneError::OsError)
}

fn get_properties() -> Option<&'static AndroidSystemProperties> {
    static PROPERTIES: OnceLock<AndroidSystemProperties> = OnceLock::new();

    Some(PROPERTIES.get_or_init(AndroidSystemProperties::new))
}
