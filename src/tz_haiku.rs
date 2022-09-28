// Notice: Only Haiku's formats "MESSAGE_FORMAT_HAIKU{,_SWAPPED}" are implemented.
//         BeOS's "MESSAGE_FORMAT_R5" and "MESSAGE_FORMAT_DANO" cannot be read.

use std::convert::{TryFrom, TryInto};
use std::ffi::CStr;
use std::fs::OpenOptions;
use std::io::{BufRead, Cursor, Read};
use std::mem::{size_of, size_of_val};

use crate::GetTimezoneError;

use self::opaque::Endian;

// Put `Endian` into a module, so `Endian::0` is inaccessible.
mod opaque {
    use super::Endianness;

    #[derive(Debug, Clone, Copy, Default)]
    #[repr(transparent)]
    pub(crate) struct Endian<T: 'static + Copy + Ord + Default>(T);

    impl Endian<u16> {
        pub(super) fn get(self, endian: Endianness) -> u16 {
            match endian {
                Endianness::Little => u16::from_le(self.0),
                Endianness::Big => u16::from_be(self.0),
            }
        }
    }

    impl Endian<u32> {
        pub(super) fn get(self, endian: Endianness) -> u32 {
            match endian {
                Endianness::Little => u32::from_le(self.0),
                Endianness::Big => u32::from_be(self.0),
            }
        }
    }
}

pub type AreaId = i32;
pub type PortId = i32;
pub type TeamId = i32;
pub type TypeCode = [u8; 4];
pub type Format = [u8; 4];

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
struct MessageHeader {
    format: Format,
    _what: Endian<u32>,
    _flags: Endian<u32>,

    _target: Endian<i32>,
    _current_specifier: Endian<i32>,
    _area_id: Endian<AreaId>,

    _reply_port: Endian<PortId>,
    _reply_target: Endian<i32>,
    _reply_team: Endian<TeamId>,

    _data_size: Endian<u32>,
    field_count: Endian<u32>,
    hash_table_size: Endian<u32>,
    _hash_table: [Endian<i32>; 0],
}

#[derive(Debug, Clone, Copy, Default)]
#[repr(C, packed)]
struct FieldHeader {
    _flags: Endian<u16>,
    name_length: Endian<u16>,
    typ: TypeCode,
    _count: Endian<u32>,
    data_size: Endian<u32>,
    _offset: Endian<u32>,
    _next_field: Endian<i32>,
}

/// SAFETY: `Self` must be packed, and not contain any references or niches
unsafe trait ReadPod: 'static + Copy + Default + Sized {
    fn read_from(mut cursor: impl Read) -> std::io::Result<Self> {
        // It would be possible to use MaybeUninit, but reading the config will be orders of
        // magnitude slower than the overhead of nulling `this` first.
        let mut this = Self::default();
        cursor.read(this.as_bytes_mut())?;
        Ok(this)
    }

    fn as_bytes_mut(&mut self) -> &mut [u8] {
        let start = std::ptr::addr_of_mut!(*self) as *mut u8;
        // SAFETY: the trait's contract ensures that `self` is POD
        unsafe { core::slice::from_raw_parts_mut(start, size_of_val(self)) }
    }
}

unsafe impl ReadPod for MessageHeader {}
unsafe impl ReadPod for FieldHeader {}
unsafe impl ReadPod for Endian<u32> {}

const LITTLE: Format = *b"HMF1";
const BIG: Format = *b"1FMH";
const RTSC: TypeCode = *b"RTSC"; // I have no idea what "RTSC" stands for.
const TIMEZONE: &[u8] = b"timezone\0";

#[derive(Debug, Clone, Copy)]
enum Endianness {
    Little,
    Big,
}

impl TryFrom<Format> for Endianness {
    type Error = crate::GetTimezoneError;

    fn try_from(value: Format) -> Result<Self, Self::Error> {
        match value {
            LITTLE => Ok(Endianness::Little),
            BIG => Ok(Endianness::Big),
            _ => Err(GetTimezoneError::FailedParsingString),
        }
    }
}

pub(crate) fn get_timezone_inner() -> Result<String, crate::GetTimezoneError> {
    const CONFIG_PATH: &str = "/boot/home/config/settings/Time settings";

    let mut buf = Vec::new();
    OpenOptions::new()
        .read(true)
        .open(CONFIG_PATH)?
        .read_to_end(&mut buf)?;
    decode_config(&buf)
}

fn decode_config(config: &[u8]) -> Result<String, crate::GetTimezoneError> {
    // We cannot simply cast slices, because the data is most likely misaligned.
    // Use a `Cursor` instead.
    let mut cursor = Cursor::new(config);
    let hdr = MessageHeader::read_from(&mut cursor)?;
    let endian = Endianness::try_from(hdr.format)?;

    // Skip hash table (no need to search in an 1-element array)
    let hash_table_size = hdr
        .hash_table_size
        .get(endian)
        .checked_mul(size_of::<i32>() as _)
        .ok_or(crate::GetTimezoneError::FailedParsingString)?
        .try_into()
        .map_err(|_| crate::GetTimezoneError::FailedParsingString)?;
    cursor.consume(hash_table_size);

    // Iterate over the entries.
    for _ in 0..hdr.field_count.get(endian) {
        let field = FieldHeader::read_from(&mut cursor)?;
        let name_length = field.name_length.get(endian) as u32;
        let data_size = field.data_size.get(endian);

        // take a view of the current field and advance the cursor
        let field_size = name_length
            .checked_add(data_size)
            .ok_or(crate::GetTimezoneError::FailedParsingString)?;
        let mut field_cursor = cursor.clone().take(field_size as u64);
        cursor.consume(field_size as usize);

        // The field does not contain the expected data, continue.
        if field.typ != RTSC || name_length != TIMEZONE.len() as u32 {
            continue;
        }
        let mut tz = [0u8; TIMEZONE.len()];
        field_cursor.read_exact(&mut tz)?;
        if tz != TIMEZONE {
            continue;
        }

        // The name should not be empty, or excessively long.
        let data_len = Endian::<u32>::read_from(&mut field_cursor)?.get(endian);
        if data_len < 1 || data_len > 32 {
            continue;
        }

        // Try to read as UTF-8 (should be ASCII)
        let mut buf = &mut [0u8; 32][..data_len as usize];
        field_cursor.read_exact(&mut buf)?;
        if let Ok(name) = CStr::from_bytes_with_nul(buf) {
            if let Ok(name) = name.to_str() {
                return Ok(name.to_owned());
            }
        }
    }
    Err(crate::GetTimezoneError::FailedParsingString)
}

#[test]
fn test() {
    #[rustfmt::skip]
    let data = [
        // format
        0x48, 0x4d, 0x46, 0x31,
        // what
        0x00, 0x00, 0x00, 0x00,
        // flags
        0x01, 0x00, 0x00, 0x00,
        // target
        0xff, 0xff, 0xff, 0xff,
        // current_specifier
        0xff, 0xff, 0xff, 0xff,
        // area_id
        0xff, 0xff, 0xff, 0xff,
        // reply_port
        0xff, 0xff, 0xff, 0xff,
        // reply_target
        0xff, 0xff, 0xff, 0xff,
        // reply_team
        0xff, 0xff, 0xff, 0xff,
        // data_size
        0x1b, 0x00, 0x00, 0x00,
        // field_count
        0x01, 0x00, 0x00, 0x00,
        // hash_table_size
        0x05, 0x00, 0x00, 0x00,

        // hash_table
        0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff,
        0xff, 0xff, 0xff, 0xff,
        0x00, 0x00, 0x00, 0x00,
        0xff, 0xff, 0xff, 0xff,

        // flags
        0x01, 0x00,
        // name_length
        0x09, 0x00,
        // type "RTSC"
        0x52, 0x54, 0x53, 0x43,
        // count
        0x01, 0x00, 0x00, 0x00,
        // data_size
        0x12, 0x00, 0x00, 0x00,
        // offset
        0x00, 0x00, 0x00, 0x00,
        // next_field
        0xff, 0xff, 0xff, 0xff,

        // "timezone" + NUL
        0x74, 0x69, 0x6d, 0x65, 0x7a, 0x6f, 0x6e, 0x65, 0x00,

        // length + "Europe/Berlin" + NUL
        0x0e, 0x00, 0x00, 0x00,
        0x45, 0x75, 0x72, 0x6f, 0x70, 0x65, 0x2f, 0x42, 0x65, 0x72, 0x6c, 0x69, 0x6e, 0x00,

        // padding
        0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20,
    ];

    assert_eq!(decode_config(&data).unwrap(), "Europe/Berlin");
}
