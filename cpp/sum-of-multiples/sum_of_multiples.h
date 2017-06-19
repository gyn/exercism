//
//
//

#include <cstdint>
#include <vector>
#include <algorithm>
#include <numeric>
#include <iostream>

namespace sum_of_multiples {
    uint64_t to(std::vector<uint32_t> factors, uint32_t limit) {
        uint64_t sum = 0;

        for (auto i = 1U; i < limit; i++) {
            auto n = std::find_if(factors.cbegin(),
                                  factors.cend(),
                                  [&](const uint32_t n){ return (i % n == 0);});

            if (n != std::end(factors)) {
                sum += i;
            }
        }

        return sum;
    }
}