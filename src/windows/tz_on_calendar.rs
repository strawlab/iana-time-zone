use std::ptr::null_mut;

use super::inspectable::Inspectable;
use super::interfaces::ITimeZoneOnCalendar;
use super::{E_FAIL, GUID, HRESULT};

const TIMEZONE_ON_CALENDAR_GUID: GUID = GUID {
    data1: 0xbb3c25e5,
    data2: 0x46cf,
    data3: 0x4317,
    data4: [0xa3, 0xf5, 0x02, 0x62, 0x1a, 0xd5, 0x44, 0x78],
};

pub struct TzOnCalendar(ITimeZoneOnCalendar);

impl TzOnCalendar {
    pub fn query(source: &Inspectable) -> Result<Self, HRESULT> {
        let mut result = null_mut();
        // SAFETY: An `Instance` is only ever created with a valid `IUnknown`.
        unsafe {
            let source = source.as_ptr();
            ((**source).QueryInterface)(source.cast(), &TIMEZONE_ON_CALENDAR_GUID, &mut result)
                .into_result()?;
        };
        // Per contract, `result` cannot be null, but better safe than sorry.
        if result.is_null() {
            return Err(E_FAIL);
        }

        Ok(Self(result.cast()))
    }

    /// SAFETY: You are not allowed to release the returned pointer.
    pub unsafe fn as_ptr(&self) -> ITimeZoneOnCalendar {
        self.0
    }
}

impl Drop for TzOnCalendar {
    fn drop(&mut self) {
        // SAFETY: `TzOnCalendar` is only ever created with a valid `ITimeZoneOnCalendar`.
        unsafe { ((**self.0).Release)(self.0.cast()) };
    }
}
