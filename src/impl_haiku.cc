#include "iana-time-zone/include/impl_haiku.h"
#include "iana-time-zone/src/tz_haiku.rs.h"

#include <cstring>
#include <Errors.h>
#include <LocaleRoster.h>
#include <String.h>
#include <TimeZone.h>

size_t ::tz_haiku::get_tz(rust::Slice<uint8_t> buf) {
    try {
        static_assert(sizeof(char) == sizeof(uint8_t), "Illegal char size");

        if (buf.empty()) {
            return 0;
        }

        BLocaleRoster *locale_roster(BLocaleRoster::Default());
        if (!locale_roster) {
            return 0;
        }

        BTimeZone tz(NULL, NULL);
        if (locale_roster->GetDefaultTimeZone(&tz) != B_OK) {
            return 0;
        }

        BString bname(tz.ID());
        int32_t ilength(bname.Length());
        if (ilength <= 0) {
            return 0;
        }

        size_t length(ilength);
        if (length > buf.size()) {
            return 0;
        }

        // BString::String() returns a borrowed string.
        // https://www.haiku-os.org/docs/api/classBString.html#ae4fe78b06c8e3310093b80305e14ba87
        const char *sname(bname.String());
        if (!sname) {
            return 0;
        }

        std::memcpy(buf.data(), sname, length);
        return length;
    } catch (...) {
        return 0;
    }
}
