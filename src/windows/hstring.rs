use std::ptr::null_mut;

use super::tz_on_calendar::TzOnCalendar;
use super::winapi::windows_delete_string;
use super::HRESULT;

#[repr(C)]
pub struct HstringHeader {
    pub flags: u32,
    pub len: u32,
    pub padding1: u32,
    pub padding2: u32,
    pub ptr: *const u16,
}

#[repr(transparent)]
pub struct Hstring(*mut HstringHeader);

impl<'a> Hstring {
    pub fn from_tz_on_calendar(instance: &'a TzOnCalendar) -> Result<Self, HRESULT> {
        let mut string = null_mut();
        // SAFETY: A `TzOnCalendar` is only ever created with a valid instance.
        let result = unsafe {
            let instance = instance.as_ptr();
            ((**instance).GetTimeZone)(instance, &mut string)
        };
        if result < 0 || string.is_null() {
            return Err(result);
        }

        Ok(Self(string))
    }
}

impl Drop for Hstring {
    fn drop(&mut self) {
        // SAFETY: An `HString` is only ever created with a valid `HSTRING`.
        let _ = unsafe { windows_delete_string(self) };
    }
}

// The `w!(…)` and `h!(…)` macros were copied and adapted from
// <https://github.com/microsoft/windows-rs/blob/cc2032e7462633e53e560ce1ee00a3ea9f87402e/crates/libs/windows/src/core/strings/literals.rs>.
// © 2022 Microsoft, licensed under MIT OR Apache-2.0.

/// Convert an ASCII string to a null-terminated `*const u16` wide string.
/// The string must be ASCII and not contain NUL characters.
macro_rules! w {
    ($s:literal) => {{
        const INPUT: &[u8] = $s.as_bytes();
        const OUTPUT_LEN: usize = $s.len() + 1;
        const OUTPUT: [u16; OUTPUT_LEN] = {
            let mut buffer = [0; OUTPUT_LEN];
            let mut pos = 0;
            while pos < INPUT.len() {
                // ensure that the input string is ASCII and does not contain any NUL characters
                let _ascii_only = [0][if matches!(INPUT[pos], 1..=0x7f) { 0 } else { 1 }];

                buffer[pos] = INPUT[pos] as u16;
                pos += 1;
            }
            buffer
        };
        OUTPUT.as_ptr()
    }};
}

/// A literal HSTRING, length-prefixed wide string with a trailing null terminator for use with
/// WinRT APIs. The string must be ASCII and not contain NUL characters.
macro_rules! h {
    ($s:literal) => {{
        use std::mem::transmute;

        use $crate::windows::hstring::{w, Hstring, HstringHeader};

        /// Don't use reference counting.
        const HSTRING_REFERENCE_FLAG: u32 = 0x1;
        /// Unknown flag, copied from `windows::h!(…)`.
        const UNKNOWN_FLAG_10: u32 = 0x10;

        const INPUT: &[u8] = $s.as_bytes();
        const OUTPUT_LEN: usize = INPUT.len() + 1;
        if OUTPUT_LEN == 1 {
            // SAFETY: A null pointer is a valid `HSTRING`, denoting an empty string.
            //         `WindowsGetStringRawBuffer(null())` returns a pointer to an empty string.
            unsafe { transmute(std::ptr::null::<u16>()) }
        } else {
            const OUTPUT: *const u16 = w!($s);
            const HEADER: HstringHeader = HstringHeader {
                flags: HSTRING_REFERENCE_FLAG | UNKNOWN_FLAG_10,
                len: (OUTPUT_LEN - 1) as u32,
                padding1: 0,
                padding2: 0,
                ptr: OUTPUT,
            };
            // SAFETY: an `HSTRING` is exactly equivalent to a pointer to an `HSTRING_HEADER`
            unsafe { transmute::<&HstringHeader, Hstring>(&HEADER) }
        }
    }};
}

pub(crate) use {h, w};
