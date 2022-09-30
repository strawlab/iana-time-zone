#pragma once

#include "rust/cxx.h"

#include <cstddef>

namespace tz_haiku {
size_t get_tz(uint8_t *buf, size_t buf_len);
}
