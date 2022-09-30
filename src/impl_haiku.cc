#include "iana-time-zone/include/impl_haiku.h"
#include "iana-time-zone/src/tz_haiku.rs.h"

#include <cstring>
#include <Errors.h>
#include <LocaleRoster.h>
#include <String.h>
#include <TimeZone.h>

size_t ::tz_haiku::get_tz(uint8_t *buf, size_t buf_len) {
    try {
        static_assert(sizeof(char) == sizeof(uint8_t), "Illegal char size");

        if (!buf || !buf_len) {
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
        int32_t length(bname.Length());
        if (length <= 0 || size_t(length) > buf_len) {
            return 0;
        }

        const char *sname(bname.String());
        if (!sname) {
            return 0;
        }

        std::memcpy(buf, sname, length);
        return length;
    } catch (...) {
        return 0;
    }
}
