#include <cstddef>

#ifdef __HAIKU__

#include <cstring>

#include <Errors.h>
#include <LocaleRoster.h>
#include <String.h>
#include <TimeZone.h>

/**
 * @brief Retrieves the IANA time zone identifier on Haiku.
 *
 * This function obtains the default time zone from the Haiku Locale Roster and writes its IANA
 * time zone identifier into the provided buffer.
 *
 * @param buf Pointer to a character buffer where the time zone identifier will be stored.
 * @param buf_size The size of the buffer pointed to by @a buf.
 * @return The length of the time zone identifier written into @a buf (excluding the null terminator),
 *         or 0 if an error occurs (e.g., invalid parameters, failure to retrieve the time zone,
 *         or insufficient buffer size).
 *
 * @note If the buffer is large enough, the function guarantees that the output is null-terminated.
 *
 * @warning This function is marked as noexcept; it will not throw exceptions. In case of any
 *          internal error, the function will simply return 0.
 */
extern "C" [[nodiscard]] auto iana_time_zone_haiku_get_tz(char* buf, const std::size_t buf_size) noexcept
    -> std::size_t {
    if (buf == nullptr || buf_size == 0) {
        return 0;
    }

    try {
        // Retrieve the default locale roster (statically allocated object)
        BLocaleRoster* locale_roster = BLocaleRoster::Default();
        if (locale_roster == nullptr) {
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
        // Ensure there is room for the null terminator without doing an addition that might overflow.
        if (length >= buf_size) {
            return 0;
        }

        bname.CopyInto(buf, 0, raw_length);
        // Insert the null terminator.
        buf[length] = '\0';
        return length;
    } catch (const std::exception& /*e*/) {
        return 0;
    } catch (...) {
        return 0;
    }
}

#else

/**
 * @brief Dummy implementation for non-Haiku platforms.
 *
 * On non-Haiku platforms, this function does nothing and always returns 0.
 *
 * @param buf Pointer to a character buffer (unused).
 * @param buf_size The size of the buffer (unused).
 * @return Always returns 0.
 */
extern "C" [[nodiscard]] auto iana_time_zone_haiku_get_tz([[maybe_unused]] char* buf,
                                                          [[maybe_unused]] const std::size_t buf_size) noexcept
    -> std::size_t {
    return 0;
}

#endif
