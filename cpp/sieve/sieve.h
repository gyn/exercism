//
//
//
#ifndef SIEVE_H
#define SIEVE_H


#include <vector>
#include <numeric>
#include <iostream>
#include <algorithm>

namespace sieve {
    std::vector<int> primes(unsigned int limit) {
        std::vector<int> numbers(limit - 1);

        std::iota(numbers.begin(), numbers.end(), 2);

        auto const size = numbers.size();
        for (auto i = 0; i < size; i++) {
            auto v = numbers[i];

            if (v == 0) {
                continue;
            }

            for (auto j = v + v; j <= limit; j += v) {
                numbers[j - 2] = 0;
            }
        }

        std::vector<int> result;

        std::copy_if(numbers.cbegin(),
                     numbers.cend(),
                     std::back_inserter(result),
                     [](int n){ return n;}
        );

        return result;
    }
}
#endif