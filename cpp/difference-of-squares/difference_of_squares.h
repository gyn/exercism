//
//
//

#include <cstdint>

namespace squares {
    uint64_t square_of_sums(uint32_t v) {
        uint32_t r;

        r = (v + 1) * v / 2;

        return (r * r);
    }

    uint64_t sum_of_squares(uint32_t v) {
        uint64_t result;

        result = v * (v + 1) * (2 * v + 1) / 6;

        return result;
    }

    uint64_t difference(uint32_t v) {
        uint64_t result;

        //
        //   (v + 1)^2 * v^2 / 4 - v * (v + 1) * (2 * v + 1) / 6
        // = (6 * (v^2 + 2 * v + 1) * v^2 - 4 * v (2 * v^2 + 3 * v + 1)) / 24
        // = (6 * (v^4 + 2 * v^3 + v^2) - 4 * (2 * v^3 + 3 * v^2 + v)) / 24
        // = ((6 * v^4 + 12 * v^3 + 6 * v^2) - (8 * v^3 + 12 * v^2 + 4 * v)) / 24
        // = (6 * v^4 + 12 * v^3 + 6 * v^2 - 8 * v^3 - 12 * v^2 - 4 * v)) / 24
        // = (6 * v^4 + 4 * v^3 - 6 * v^2 - 4 * v) / 24
        // = v * (6v + 4) * (v + 1) * (v - 1) / 24

        result = v * (6 * v + 4) * (v + 1) * (v - 1) / 24;

        return result;
    }
}