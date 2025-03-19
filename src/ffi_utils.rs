//! Cross platform FFI helpers.

#[cfg(any(test, target_os = "android"))]
use std::ffi::CStr;

/// A buffer to store the timezone name when calling the C API.
#[cfg(any(test, target_vendor = "apple", target_env = "ohos"))]
pub(crate) mod buffer {
    /// The longest name in the IANA time zone database is 32 ASCII characters long.
    pub const MAX_LEN: usize = 64;

    /// Return a buffer to store the timezone name.
    ///
    /// The buffer is used to store the timezone name when calling the C API.
    pub const fn tzname_buf() -> [u8; MAX_LEN] {
        [0; MAX_LEN]
    }
}

// The system property named 'persist.sys.timezone' contains the name of the
// current timezone.
//
// From https://android.googlesource.com/platform/bionic/+/gingerbread-release/libc/docs/OVERVIEW.TXT#79:
//
// > The name of the current timezone is taken from the TZ environment variable,
// > if defined. Otherwise, the system property named 'persist.sys.timezone' is
// > checked instead.
//
// TODO: Use a `c"..."` literal when MSRV is upgraded beyond 1.77.0.
// https://doc.rust-lang.org/edition-guide/rust-2021/c-string-literals.html
#[cfg(any(test, target_os = "android"))]
const ANDROID_TIMEZONE_PROPERTY_NAME: &[u8] = b"persist.sys.timezone\0";

/// Return a [`CStr`] to access the timezone from an Android system properties
/// environment.
#[cfg(any(test, target_os = "android"))]
pub(crate) fn android_timezone_property_name() -> &'static CStr {
    // In tests or debug mode, opt into extra runtime checks.
    if cfg!(any(test, debug_assertions)) {
        return CStr::from_bytes_with_nul(ANDROID_TIMEZONE_PROPERTY_NAME).unwrap();
    }

    // SAFETY: the key is NUL-terminated and there are no other NULs, this
    // invariant is checked in tests.
    unsafe { CStr::from_bytes_with_nul_unchecked(ANDROID_TIMEZONE_PROPERTY_NAME) }
}

#[cfg(test)]
mod tests {
    use core::mem::size_of_val;
    use std::ffi::CStr;

    use super::buffer::{tzname_buf, MAX_LEN};
    use super::{android_timezone_property_name, ANDROID_TIMEZONE_PROPERTY_NAME};

    #[test]
    fn test_android_timezone_property_name_is_valid_cstr() {
        CStr::from_bytes_with_nul(ANDROID_TIMEZONE_PROPERTY_NAME).unwrap();

        let mut invalid_property_name = ANDROID_TIMEZONE_PROPERTY_NAME.to_owned();
        invalid_property_name.push(b'\0');
        CStr::from_bytes_with_nul(&invalid_property_name).unwrap_err();
    }

    #[test]
    fn test_android_timezone_property_name_getter() {
        let key = android_timezone_property_name().to_bytes_with_nul();
        assert_eq!(key, ANDROID_TIMEZONE_PROPERTY_NAME);
        std::str::from_utf8(key).unwrap();
    }

    /// An exhaustive set of IANA timezone names for testing.
    ///
    /// Pulled from Wikipedia as of Sat March 22, 2025:
    ///
    /// - <https://en.wikipedia.org/wiki/List_of_tz_database_time_zones>
    /// - <https://en.wikipedia.org/w/index.php?title=List_of_tz_database_time_zones&oldid=1281103182>
    static KNOWN_TIMEZONE_NAMES: &[&str] = &[
        "Africa/Abidjan",
        "Africa/Accra",
        "Africa/Addis_Ababa",
        "Africa/Algiers",
        "Africa/Asmara",
        "Africa/Asmera",
        "Africa/Bamako",
        "Africa/Bangui",
        "Africa/Banjul",
        "Africa/Bissau",
        "Africa/Blantyre",
        "Africa/Brazzaville",
        "Africa/Bujumbura",
        "Africa/Cairo",
        "Africa/Casablanca",
        "Africa/Conakry",
        "Africa/Dakar",
        "Africa/Dar_es_Salaam",
        "Africa/Djibouti",
        "Africa/Douala",
        "Africa/El_Aaiun",
        "Africa/Freetown",
        "Africa/Gaborone",
        "Africa/Harare",
        "Africa/Johannesburg",
        "Africa/Juba",
        "Africa/Kampala",
        "Africa/Khartoum",
        "Africa/Kigali",
        "Africa/Libreville",
        "Africa/Lome",
        "Africa/Luanda",
        "Africa/Lusaka",
        "Africa/Malabo",
        "Africa/Maseru",
        "Africa/Mbabane",
        "Africa/Mogadishu",
        "Africa/Monrovia",
        "Africa/Nairobi",
        "Africa/Ndjamena",
        "Africa/Niamey",
        "Africa/Nouakchott",
        "Africa/Ouagadougou",
        "Africa/Porto-Novo",
        "Africa/Sao_Tome",
        "Africa/Timbuktu",
        "Africa/Tripoli",
        "Africa/Tunis",
        "Africa/Windhoek",
        "America/Anguilla",
        "America/Antigua",
        "America/Argentina/ComodRivadavia",
        "America/Aruba",
        "America/Asuncion",
        "America/Atka",
        "America/Barbados",
        "America/Belize",
        "America/Bogota",
        "America/Buenos_Aires",
        "America/Caracas",
        "America/Catamarca",
        "America/Cayenne",
        "America/Cayman",
        "America/Coral_Harbour",
        "America/Cordoba",
        "America/Costa_Rica",
        "America/Curacao",
        "America/Dominica",
        "America/El_Salvador",
        "America/Ensenada",
        "America/Fort_Wayne",
        "America/Godthab",
        "America/Grand_Turk",
        "America/Grenada",
        "America/Guadeloupe",
        "America/Guatemala",
        "America/Guyana",
        "America/Havana",
        "America/Indianapolis",
        "America/Jamaica",
        "America/Jujuy",
        "America/Knox_IN",
        "America/Kralendijk",
        "America/La_Paz",
        "America/Lima",
        "America/Louisville",
        "America/Lower_Princes",
        "America/Managua",
        "America/Marigot",
        "America/Martinique",
        "America/Mendoza",
        "America/Miquelon",
        "America/Montevideo",
        "America/Montreal",
        "America/Montserrat",
        "America/Nassau",
        "America/Nipigon",
        "America/Pangnirtung",
        "America/Paramaribo",
        "America/Port-au-Prince",
        "America/Port_of_Spain",
        "America/Porto_Acre",
        "America/Rainy_River",
        "America/Rosario",
        "America/Santa_Isabel",
        "America/Santo_Domingo",
        "America/Shiprock",
        "America/St_Barthelemy",
        "America/St_Kitts",
        "America/St_Lucia",
        "America/St_Thomas",
        "America/St_Vincent",
        "America/Tegucigalpa",
        "America/Thunder_Bay",
        "America/Tortola",
        "America/Virgin",
        "America/Yellowknife",
        "Antarctica/South_Pole",
        "Arctic/Longyearbyen",
        "Asia/Aden",
        "Asia/Amman",
        "Asia/Ashgabat",
        "Asia/Ashkhabad",
        "Asia/Baghdad",
        "Asia/Bahrain",
        "Asia/Baku",
        "Asia/Beirut",
        "Asia/Bishkek",
        "Asia/Brunei",
        "Asia/Calcutta",
        "Asia/Choibalsan",
        "Asia/Chongqing",
        "Asia/Chungking",
        "Asia/Colombo",
        "Asia/Dacca",
        "Asia/Damascus",
        "Asia/Dhaka",
        "Asia/Dili",
        "Asia/Dushanbe",
        "Asia/Harbin",
        "Asia/Hong_Kong",
        "Asia/Istanbul",
        "Asia/Jerusalem",
        "Asia/Kabul",
        "Asia/Karachi",
        "Asia/Kashgar",
        "Asia/Kathmandu",
        "Asia/Katmandu",
        "Asia/Kolkata",
        "Asia/Kuwait",
        "Asia/Macao",
        "Asia/Macau",
        "Asia/Manila",
        "Asia/Muscat",
        "Asia/Phnom_Penh",
        "Asia/Pyongyang",
        "Asia/Qatar",
        "Asia/Rangoon",
        "Asia/Saigon",
        "Asia/Seoul",
        "Asia/Taipei",
        "Asia/Tbilisi",
        "Asia/Tehran",
        "Asia/Tel_Aviv",
        "Asia/Thimbu",
        "Asia/Thimphu",
        "Asia/Ujung_Pandang",
        "Asia/Ulan_Bator",
        "Asia/Vientiane",
        "Asia/Yangon",
        "Asia/Yerevan",
        "Atlantic/Bermuda",
        "Atlantic/Cape_Verde",
        "Atlantic/Faeroe",
        "Atlantic/Faroe",
        "Atlantic/Jan_Mayen",
        "Atlantic/Reykjavik",
        "Atlantic/South_Georgia",
        "Atlantic/St_Helena",
        "Atlantic/Stanley",
        "Australia/ACT",
        "Australia/Canberra",
        "Australia/Currie",
        "Australia/LHI",
        "Australia/North",
        "Australia/NSW",
        "Australia/Queensland",
        "Australia/South",
        "Australia/Tasmania",
        "Australia/Victoria",
        "Australia/West",
        "Australia/Yancowinna",
        "Brazil/Acre",
        "Brazil/DeNoronha",
        "Brazil/East",
        "Brazil/West",
        "Canada/Atlantic",
        "Canada/Central",
        "Canada/Eastern",
        "Canada/Mountain",
        "Canada/Newfoundland",
        "Canada/Pacific",
        "Canada/Saskatchewan",
        "Canada/Yukon",
        "CET",
        "Chile/Continental",
        "Chile/EasterIsland",
        "CST6CDT",
        "Cuba",
        "EET",
        "Egypt",
        "Eire",
        "EST",
        "EST5EDT",
        "Etc/GMT",
        "Etc/GMT+0",
        "Etc/GMT+1",
        "Etc/GMT+10",
        "Etc/GMT+11",
        "Etc/GMT+12",
        "Etc/GMT+2",
        "Etc/GMT+3",
        "Etc/GMT+4",
        "Etc/GMT+5",
        "Etc/GMT+6",
        "Etc/GMT+7",
        "Etc/GMT+8",
        "Etc/GMT+9",
        "Etc/GMT-0",
        "Etc/GMT-1",
        "Etc/GMT-10",
        "Etc/GMT-11",
        "Etc/GMT-12",
        "Etc/GMT-13",
        "Etc/GMT-14",
        "Etc/GMT-2",
        "Etc/GMT-3",
        "Etc/GMT-4",
        "Etc/GMT-5",
        "Etc/GMT-6",
        "Etc/GMT-7",
        "Etc/GMT-8",
        "Etc/GMT-9",
        "Etc/GMT0",
        "Etc/Greenwich",
        "Etc/UCT",
        "Etc/Universal",
        "Etc/UTC",
        "Etc/Zulu",
        "Europe/Amsterdam",
        "Europe/Andorra",
        "Europe/Athens",
        "Europe/Belfast",
        "Europe/Belgrade",
        "Europe/Bratislava",
        "Europe/Brussels",
        "Europe/Bucharest",
        "Europe/Budapest",
        "Europe/Chisinau",
        "Europe/Copenhagen",
        "Europe/Dublin",
        "Europe/Gibraltar",
        "Europe/Guernsey",
        "Europe/Helsinki",
        "Europe/Isle_of_Man",
        "Europe/Istanbul",
        "Europe/Jersey",
        "Europe/Kiev",
        "Europe/Ljubljana",
        "Europe/London",
        "Europe/Luxembourg",
        "Europe/Malta",
        "Europe/Mariehamn",
        "Europe/Minsk",
        "Europe/Monaco",
        "Europe/Nicosia",
        "Europe/Oslo",
        "Europe/Paris",
        "Europe/Podgorica",
        "Europe/Prague",
        "Europe/Riga",
        "Europe/Rome",
        "Europe/San_Marino",
        "Europe/Sarajevo",
        "Europe/Skopje",
        "Europe/Sofia",
        "Europe/Stockholm",
        "Europe/Tallinn",
        "Europe/Tirane",
        "Europe/Tiraspol",
        "Europe/Uzhgorod",
        "Europe/Vaduz",
        "Europe/Vatican",
        "Europe/Vienna",
        "Europe/Vilnius",
        "Europe/Warsaw",
        "Europe/Zagreb",
        "Europe/Zaporozhye",
        "Factory",
        "GB",
        "GB-Eire",
        "GMT",
        "GMT+0",
        "GMT-0",
        "GMT0",
        "Greenwich",
        "Hongkong",
        "HST",
        "Iceland",
        "Indian/Antananarivo",
        "Indian/Chagos",
        "Indian/Christmas",
        "Indian/Cocos",
        "Indian/Comoro",
        "Indian/Kerguelen",
        "Indian/Mahe",
        "Indian/Mauritius",
        "Indian/Mayotte",
        "Indian/Reunion",
        "Iran",
        "Israel",
        "Jamaica",
        "Japan",
        "Kwajalein",
        "Libya",
        "MET",
        "Mexico/BajaNorte",
        "Mexico/BajaSur",
        "Mexico/General",
        "MST",
        "MST7MDT",
        "Navajo",
        "NZ",
        "NZ-CHAT",
        "Pacific/Apia",
        "Pacific/Efate",
        "Pacific/Enderbury",
        "Pacific/Fakaofo",
        "Pacific/Fiji",
        "Pacific/Funafuti",
        "Pacific/Guam",
        "Pacific/Johnston",
        "Pacific/Nauru",
        "Pacific/Niue",
        "Pacific/Norfolk",
        "Pacific/Noumea",
        "Pacific/Palau",
        "Pacific/Pitcairn",
        "Pacific/Ponape",
        "Pacific/Rarotonga",
        "Pacific/Saipan",
        "Pacific/Samoa",
        "Pacific/Tongatapu",
        "Pacific/Truk",
        "Pacific/Wallis",
        "Pacific/Yap",
        "Poland",
        "Portugal",
        "PRC",
        "PST8PDT",
        "ROC",
        "ROK",
        "Singapore",
        "Turkey",
        "UCT",
        "Universal",
        "US/Alaska",
        "US/Aleutian",
        "US/Arizona",
        "US/Central",
        "US/East-Indiana",
        "US/Eastern",
        "US/Hawaii",
        "US/Indiana-Starke",
        "US/Michigan",
        "US/Mountain",
        "US/Pacific",
        "US/Samoa",
        "UTC",
        "W-SU",
        "WET",
        "Zulu",
        "America/Rio_Branco",
        "America/Maceio",
        "America/Metlakatla",
        "America/Juneau",
        "America/Sitka",
        "America/Adak",
        "America/Yakutat",
        "America/Anchorage",
        "America/Nome",
        "America/Manaus",
        "America/Eirunepe",
        "Asia/Aqtobe",
        "America/Blanc-Sablon",
        "America/Puerto_Rico",
        "America/Goose_Bay",
        "America/Moncton",
        "America/Glace_Bay",
        "America/Halifax",
        "America/Noronha",
        "Asia/Atyrau",
        "Atlantic/Azores",
        "America/Bahia",
        "America/Bahia_Banderas",
        "America/Tijuana",
        "America/Mazatlan",
        "Asia/Hovd",
        "Asia/Shanghai",
        "Asia/Makassar",
        "Asia/Pontianak",
        "Pacific/Bougainville",
        "America/Fortaleza",
        "America/Sao_Paulo",
        "America/Argentina/Buenos_Aires",
        "Europe/Busingen",
        "Europe/Zurich",
        "America/Merida",
        "Atlantic/Canary",
        "Antarctica/Casey",
        "America/Argentina/Catamarca",
        "America/Indiana/Tell_City",
        "America/Indiana/Knox",
        "America/Menominee",
        "America/North_Dakota/Beulah",
        "America/North_Dakota/New_Salem",
        "America/North_Dakota/Center",
        "America/Rankin_Inlet",
        "America/Resolute",
        "America/Winnipeg",
        "America/Chicago",
        "Africa/Maputo",
        "America/Mexico_City",
        "Africa/Ceuta",
        "Pacific/Chatham",
        "America/Chihuahua",
        "America/Ojinaga",
        "America/Ciudad_Juarez",
        "Pacific/Chuuk",
        "America/Matamoros",
        "Europe/Simferopol",
        "Asia/Dubai",
        "America/Swift_Current",
        "America/Regina",
        "Antarctica/Davis",
        "Africa/Lubumbashi",
        "Africa/Kinshasa",
        "Antarctica/DumontDUrville",
        "America/Monterrey",
        "Pacific/Easter",
        "America/Indiana/Marengo",
        "America/Indiana/Vincennes",
        "America/Indiana/Indianapolis",
        "America/Indiana/Petersburg",
        "America/Indiana/Winamac",
        "America/Indiana/Vevay",
        "America/Kentucky/Louisville",
        "America/Kentucky/Monticello",
        "America/Detroit",
        "America/Iqaluit",
        "America/Toronto",
        "America/New_York",
        "America/Guayaquil",
        "America/Atikokan",
        "America/Panama",
        "Asia/Tokyo",
        "Pacific/Galapagos",
        "Pacific/Gambier",
        "Asia/Gaza",
        "Pacific/Tarawa",
        "Pacific/Honolulu",
        "Asia/Jakarta",
        "America/Argentina/Jujuy",
        "Indian/Maldives",
        "Pacific/Kosrae",
        "Pacific/Kwajalein",
        "America/Argentina/La_Rioja",
        "Pacific/Kiritimati",
        "Australia/Lord_Howe",
        "Antarctica/Macquarie",
        "Atlantic/Madeira",
        "Asia/Kuala_Lumpur",
        "Asia/Aqtau",
        "Pacific/Marquesas",
        "America/Cuiaba",
        "America/Campo_Grande",
        "Antarctica/Mawson",
        "America/Argentina/Mendoza",
        "Pacific/Pago_Pago",
        "Pacific/Midway",
        "America/Argentina/Cordoba",
        "America/Santiago",
        "Asia/Nicosia",
        "Europe/Berlin",
        "America/Nuuk",
        "Asia/Almaty",
        "Pacific/Majuro",
        "Asia/Ulaanbaatar",
        "Europe/Kyiv",
        "America/Edmonton",
        "America/Boise",
        "America/Inuvik",
        "America/Cambridge_Bay",
        "America/Denver",
        "Europe/Kaliningrad",
        "Europe/Kirov",
        "Europe/Moscow",
        "Europe/Volgograd",
        "Europe/Astrakhan",
        "Europe/Samara",
        "Europe/Saratov",
        "Europe/Ulyanovsk",
        "Asia/Yekaterinburg",
        "Asia/Omsk",
        "Asia/Barnaul",
        "Asia/Novokuznetsk",
        "Asia/Krasnoyarsk",
        "Asia/Novosibirsk",
        "Asia/Tomsk",
        "Asia/Irkutsk",
        "Asia/Yakutsk",
        "Asia/Khandyga",
        "Asia/Chita",
        "Asia/Vladivostok",
        "Asia/Ust-Nera",
        "Asia/Magadan",
        "Asia/Srednekolymsk",
        "Asia/Sakhalin",
        "Asia/Anadyr",
        "Asia/Kamchatka",
        "America/Phoenix",
        "America/Creston",
        "America/Dawson_Creek",
        "America/Fort_Nelson",
        "America/Whitehorse",
        "America/Dawson",
        "America/Danmarkshavn",
        "Asia/Jayapura",
        "Australia/Sydney",
        "Australia/Broken_Hill",
        "Pacific/Auckland",
        "Antarctica/McMurdo",
        "America/St_Johns",
        "Asia/Bangkok",
        "Asia/Famagusta",
        "Australia/Darwin",
        "America/Los_Angeles",
        "America/Vancouver",
        "Antarctica/Palmer",
        "Pacific/Port_Moresby",
        "America/Belem",
        "America/Santarem",
        "Asia/Singapore",
        "America/Recife",
        "Pacific/Kanton",
        "Pacific/Guadalcanal",
        "Pacific/Pohnpei",
        "Europe/Lisbon",
        "Asia/Qostanay",
        "Australia/Brisbane",
        "Australia/Lindeman",
        "America/Cancun",
        "Asia/Qyzylorda",
        "America/Punta_Arenas",
        "America/Porto_Velho",
        "America/Boa_Vista",
        "Antarctica/Rothera",
        "Asia/Kuching",
        "America/Argentina/Salta",
        "America/Argentina/San_Juan",
        "America/Argentina/San_Luis",
        "America/Argentina/Rio_Gallegos",
        "America/Scoresbysund",
        "Pacific/Tahiti",
        "America/Hermosillo",
        "Australia/Adelaide",
        "Asia/Ho_Chi_Minh",
        "Europe/Madrid",
        "Antarctica/Syowa",
        "Asia/Riyadh",
        "Australia/Hobart",
        "America/Thule",
        "America/Argentina/Ushuaia",
        "America/Araguaina",
        "Antarctica/Troll",
        "America/Argentina/Tucuman",
        "Asia/Tashkent",
        "Asia/Samarkand",
        "Australia/Melbourne",
        "Antarctica/Vostok",
        "Pacific/Wake",
        "Africa/Lagos",
        "Asia/Hebron",
        "Asia/Oral",
        "Australia/Eucla",
        "Australia/Perth",
        "Asia/Urumqi",
    ];

    #[test]
    fn test_tzname_buffer_fits_all_iana_names() {
        let buf = tzname_buf();
        let max_len = buf.len();

        let mut failed_tz_names = vec![];

        for &tz in KNOWN_TIMEZONE_NAMES {
            // Require max_len + 1 to account for an optional NUL terminator.
            if tz.len() >= max_len {
                failed_tz_names.push(tz);
            }
        }

        assert!(
            failed_tz_names.is_empty(),
            "One or more timezone names exceed the buffer length of {}. Max length of found timezone: {}\n{:?}",
            max_len,
            failed_tz_names.iter().map(|s| s.len()).max().unwrap(),
            failed_tz_names
        );
    }

    #[test]
    fn test_tzname_buffer_correct_size() {
        assert_eq!(
            MAX_LEN, 64,
            "Buffer length changed unexpectedly, ensure consistency with documented limit."
        );
        assert_eq!(
            tzname_buf().len(),
            MAX_LEN,
            "Buffer length changed unexpectedly, ensure consistency with documented limit."
        );
        assert_eq!(
            size_of_val(&tzname_buf()),
            MAX_LEN,
            "Buffer length changed unexpectedly, ensure consistency with documented limit."
        );
    }
}
