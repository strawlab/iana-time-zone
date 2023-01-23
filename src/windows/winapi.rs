use std::convert::TryInto;
use std::ffi::c_void;
use std::mem::transmute_copy;
use std::ptr::null_mut;

use super::hstring::Hstring;
use super::interfaces::IInspectable;
use super::HRESULT;

#[link(name = "windows")]
extern "system" {
    fn CoIncrementMTAUsage(pcookie: *mut isize) -> HRESULT;

    fn RoActivateInstance(activatableclassid: *mut c_void, instance: *mut IInspectable) -> HRESULT;

    fn WindowsDeleteString(string: *mut c_void) -> HRESULT;

    fn WindowsGetStringRawBuffer(string: *mut c_void, length: *mut u32) -> *const u16;
}

pub fn ro_activate_instance(activatableclassid: &Hstring) -> Result<IInspectable, HRESULT> {
    let mut result = null_mut();
    // SAFETY: Should be safe for all use cases. The supplied `HSTRING` cannot be invalid.
    let hr = unsafe { RoActivateInstance(transmute_copy(activatableclassid), &mut result) };
    // Per contract, `result` cannot be null, but better safe than sorry.
    if hr >= 0 && !result.is_null() {
        Ok(result)
    } else {
        Err(hr)
    }
}

pub fn windows_get_string_raw_buffer(string: &Hstring) -> Result<&[u16], ()> {
    let mut length = 0;
    // SAFETY: The supplied `HSTRING` is always valid.
    let data = unsafe { WindowsGetStringRawBuffer(transmute_copy(string), &mut length) };
    // Per contract, `result` cannot be null, but better safe than sorry.
    if data.is_null() {
        return Err(());
    }

    match length.try_into() {
        Ok(length) => {
            // SAFETY: `WindowsGetStringRawBuffer()` returns a valid pointer to a wide string.
            Ok(unsafe { core::slice::from_raw_parts(data, length) })
        }
        Err(_) => Err(()),
    }
}

/// SAFETY: Must be called exactly once per held `HSTRING`.
pub unsafe fn windows_delete_string(string: &Hstring) -> Result<(), HRESULT> {
    let hr = WindowsDeleteString(transmute_copy(string));
    if hr >= 0 {
        Ok(())
    } else {
        Err(hr)
    }
}

/// SAFETY: Don't call CoIncrementMTAUsage during process shutdown or inside dllmain.
pub unsafe fn co_increment_mta_usage(cookie: &mut isize) -> Result<(), HRESULT> {
    let hr = CoIncrementMTAUsage(cookie);
    if hr >= 0 {
        Ok(())
    } else {
        Err(hr)
    }
}
