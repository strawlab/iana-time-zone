#pragma once

#include "rust/cxx.h"

#include <cstddef>

namespace tz_haiku {
size_t get_tz(rust::Slice<uint8_t> buf);
}
