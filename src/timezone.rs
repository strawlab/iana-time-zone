#[cfg(feature = "alloc")]
extern crate alloc;

use core::convert::{TryFrom, TryInto};
use core::num::NonZeroU8;
use core::str::{from_utf8, from_utf8_unchecked, from_utf8_unchecked_mut, FromStr};
use core::{borrow, cmp, fmt, hash, ops, ptr};

const TIMEZONE_MAX_LENGTH: usize = 31;

/// TODO
#[derive(Clone, Copy)]
pub struct Timezone {
    data: [u8; TIMEZONE_MAX_LENGTH],
    length: NonZeroU8,
}

impl Timezone {
    /// TODO
    #[inline]
    pub fn system() -> Result<Self, crate::GetTimezoneError> {
        crate::platform::get_timezone_inner()
    }

    /// TODO
    pub fn new(s: &str) -> Option<Self> {
        let length = match s.len().try_into().ok().and_then(NonZeroU8::new) {
            Some(len) if len.get() <= TIMEZONE_MAX_LENGTH as u8 => len,
            _ => return None,
        };

        let mut data = [0u8; TIMEZONE_MAX_LENGTH];
        unsafe { ptr::copy_nonoverlapping(s.as_ptr(), data.as_mut_ptr(), length.get() as usize) };
        Some(Self { length, data })
    }

    /// TODO
    pub fn try_from_utf16(buf: &[u16]) -> Option<Self> {
        let mut char_buf = [0u8; 4];
        let mut string_buf = [0u8; TIMEZONE_MAX_LENGTH];
        let mut pos = 0;
        for c in core::char::decode_utf16(buf.iter().copied()) {
            let src = c.ok()?.encode_utf8(&mut char_buf).as_bytes();
            let len = src.len();
            let dst = string_buf.get_mut(pos..)?.get_mut(..len)?;
            unsafe { ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), len) };
            pos += len;
        }
        Self::new(unsafe { from_utf8_unchecked(&string_buf[..pos]) })
    }

    /// TODO
    pub fn try_from_display(s: impl fmt::Display) -> Option<Self> {
        #[derive(Debug, Default, Clone, Copy)]
        struct Writer {
            buf: [u8; TIMEZONE_MAX_LENGTH],
            pos: usize,
        }

        impl fmt::Write for Writer {
            fn write_str(&mut self, src: &str) -> fmt::Result {
                let dst = self
                    .buf
                    .get_mut(self.pos..)
                    .and_then(|b| b.get_mut(..src.len()))
                    .ok_or(fmt::Error)?;
                unsafe { ptr::copy_nonoverlapping(src.as_ptr(), dst.as_mut_ptr(), src.len()) };
                self.pos += src.len();
                Ok(())
            }
        }

        let mut data = Writer::default();
        fmt::write(&mut data, format_args!("{}", s)).ok()?;
        Self::new(unsafe { from_utf8_unchecked(&data.buf[..data.pos]) })
    }

    /// TODO
    #[inline]
    pub fn as_str(&self) -> &str {
        let len = self.length.get() as usize;
        unsafe { from_utf8_unchecked(&self.data[..len]) }
    }

    /// TODO
    #[inline]
    pub fn as_mut_str(&mut self) -> &mut str {
        let len = self.length.get() as usize;
        unsafe { from_utf8_unchecked_mut(&mut self.data[..len]) }
    }
}

impl AsRef<str> for Timezone {
    #[inline]
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsMut<str> for Timezone {
    #[inline]
    fn as_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl ops::Deref for Timezone {
    type Target = str;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_str()
    }
}

impl ops::DerefMut for Timezone {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.as_mut_str()
    }
}

impl borrow::Borrow<str> for Timezone {
    #[inline]
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl borrow::BorrowMut<str> for Timezone {
    fn borrow_mut(&mut self) -> &mut str {
        self.as_mut_str()
    }
}

impl cmp::PartialEq for Timezone {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl cmp::PartialOrd for Timezone {
    #[inline]
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other.as_str())
    }
}

impl cmp::PartialEq<str> for Timezone {
    #[inline]
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl cmp::PartialOrd<str> for Timezone {
    #[inline]
    fn partial_cmp(&self, other: &str) -> Option<cmp::Ordering> {
        self.as_str().partial_cmp(other)
    }
}

impl cmp::Eq for Timezone {}

impl cmp::Ord for Timezone {
    #[inline]
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl hash::Hash for Timezone {
    #[inline]
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.as_str().hash(state)
    }
}

impl fmt::Display for Timezone {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}

impl fmt::Debug for Timezone {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Timezone").field(&self.as_str()).finish()
    }
}

/// Could not convert string into [`Timezone`]
#[derive(Debug, Clone, Copy)]
pub struct CovertError;

impl fmt::Display for CovertError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("could not convert string into Timezone")
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
impl std::error::Error for CovertError {}

impl FromStr for Timezone {
    type Err = CovertError;

    #[inline]
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value).ok_or(CovertError)
    }
}

impl TryFrom<&str> for Timezone {
    type Error = CovertError;

    #[inline]
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::new(value).ok_or(CovertError)
    }
}

impl TryFrom<&[u8]> for Timezone {
    type Error = CovertError;

    #[inline]
    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        from_utf8(value).ok().and_then(Self::new).ok_or(CovertError)
    }
}

impl TryFrom<&[u16]> for Timezone {
    type Error = CovertError;

    #[inline]
    fn try_from(value: &[u16]) -> Result<Self, Self::Error> {
        Self::try_from_utf16(value).ok_or(CovertError)
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl From<Timezone> for alloc::string::String {
    fn from(value: Timezone) -> Self {
        value.as_str().to_owned()
    }
}

#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl TryFrom<alloc::string::String> for Timezone {
    type Error = CovertError;

    #[inline]
    fn try_from(value: alloc::string::String) -> Result<Self, Self::Error> {
        Self::new(&value).ok_or(CovertError)
    }
}
