/// # iana-time-zone-haiku
///
/// [![Crates.io](https://img.shields.io/crates/v/iana-time-zone-haiku.svg)](https://crates.io/crates/iana-time-zone-haiku)
/// [![Documentation](https://docs.rs/iana-time-zone/badge.svg)](https://docs.rs/iana-time-zone/)
/// [![Crate License](https://img.shields.io/crates/l/iana-time-zone-haiku-haiku.svg)](https://crates.io/crates/iana-time-zone-haiku)
/// [![build](https://github.com/strawlab/iana-time-zone/workflows/build/badge.svg?branch=master)](https://github.com/strawlab/iana-time-zone/actions?query=branch%3Amaster)
///
/// [iana-time-zone](https://github.com/strawlab/iana-time-zone) support crate for Haiku OS.

#[cxx::bridge(namespace = "iana_time_zone_haiku")]
mod ffi {
    // SAFETY: in here "unsafe" is simply part of the syntax
    unsafe extern "C++" {
        include!("iana-time-zone-haiku/src/interface.h");

        fn get_tz(buf: &mut [u8]) -> usize;
    }
}

pub fn get_timezone() -> Option<String> {
    // The longest name in the IANA time zone database is 25 ASCII characters long.
    let mut buf = [0u8; 32];
    let len = ffi::get_tz(&mut buf);
    // The name should not be empty, or excessively long.
    match buf.get(..len)? {
        b"" => None,
        s => Some(std::str::from_utf8(s).ok()?.to_owned()),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[cfg(not(target_os = "haiku"))]
    fn test_fallback_on_non_haiku_platforms() {
        assert!(super::get_timezone().is_none());
    }
}
