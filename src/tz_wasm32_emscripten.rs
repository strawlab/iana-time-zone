use crate::GetTimezoneError;
use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr::NonNull;

unsafe extern "C" {
    fn emscripten_run_script_string(script: *const c_char) -> *mut c_char;
}

pub(crate) fn get_timezone_inner() -> Result<String, GetTimezoneError> {
    const SCRIPT: &CStr = c"Intl.DateTimeFormat().resolvedOptions().timeZone";

    // SAFETY: `SCRIPT` is a NUL-terminated JavaScript expression.
    let result = unsafe { emscripten_run_script_string(SCRIPT.as_ptr()) };
    let result = NonNull::new(result).ok_or(GetTimezoneError::OsError)?;
    // SAFETY: Emscripten returns a NUL-terminated string in a buffer it owns,
    // which stays valid until the next `emscripten_run_script_string` call. It
    // is copied before this function returns.
    let result = unsafe { CStr::from_ptr(result.as_ptr()) };
    result
        .to_owned()
        .into_string()
        .map_err(|_| GetTimezoneError::FailedParsingString)
}
