//
//
//
#ifndef GRAINS_H
#define GRAINS_H

#include <cstdint>
#include <limits>
#include <cassert>

namespace grains {
    static inline uint64_t square(const uint32_t number) {
        assert(number <= 64);

        return 1ULL << (number - 1);
    }

    static inline constexpr uint64_t total() {
        //
        // 2^64 - 1
        //
        return std::numeric_limits<uint64_t>::max();
    }
}
#endif
