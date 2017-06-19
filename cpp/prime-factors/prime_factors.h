//
//
//
#ifndef PRIME_FACTORS_H
#define PRIME_FACTORS_H

#include <vector>

namespace prime_factors {
    std::vector<int> of(int number) {
        std::vector<int> result;

        int i = 2;

        while (i <= number) {
            if (number % i) {
                i++;
                continue;
            }

            number = number / i;

            result.push_back(i);
        }

        return result;
    }
}
#endif