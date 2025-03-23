#include <cstddef>

#ifdef __HAIKU__

#include <cstring>

#include <Errors.h>
#include <LocaleRoster.h>
#include <String.h>
#include <TimeZone.h>

extern "C" {

auto iana_time_zone_haiku_get_tz(char *buf, const std::size_t buf_size) -> std::size_t {
    try {
        static_assert(sizeof(char) == sizeof(uint8_t), "Illegal char size");

        if (buf == nullptr || buf_size == 0) {
            return 0;
        }

        // `BLocaleRoster::Default()` returns a reference to a statically allocated object.
        // https://github.com/haiku/haiku/blob/8f16317/src/kits/locale/LocaleRoster.cpp#L143-L147
        BLocaleRoster *locale_roster(BLocaleRoster::Default());
        if (!locale_roster) {
            return 0;
        }

        BTimeZone timezone(nullptr, nullptr);
        if (locale_roster->GetDefaultTimeZone(&timezone) != B_OK) {
            return 0;
        }

        const BString bname = timezone.ID();
        const auto raw_length = bname.Length();
        if (raw_length <= 0) {
            return 0;
        }

        const auto length = static_cast<std::size_t>(raw_length);
        if (length > buf_size) {
            return 0;
        }

        bname.CopyInto(buf, 0, raw_length);

        // Optionally, NUL-terminate the buffer if there's room:
        if (length < buf_size) {
            buf[length] = '\0';
        }

        return length;
    } catch (...) {
        return 0;
    }
}

}  // extern "C"

#else

extern "C" {

// NOLINTBEGIN(misc-unused-parameters)

auto iana_time_zone_haiku_get_tz(char *buf, const std::size_t buf_size) -> std::size_t { return 0; }

// NOLINTEND(misc-unused-parameters)

}  // extern "C"

#endif
