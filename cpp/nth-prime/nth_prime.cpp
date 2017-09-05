#include <stdexcept>
#include <vector>
#include <algorithm>

#include "nth_prime.h"

unsigned int prime::nth(unsigned int index) {
    if (index == 0) {
        throw std::domain_error("invalid index");
    }

    unsigned int number = 3;
    std::vector<unsigned int> primes{2};

    while (primes.size() < index) {
        bool is_prime = std::all_of(primes.cbegin(),
                                    primes.cend(),
                                    [&](unsigned int i) {
                                        return number % i != 0;
                                    });

        if (is_prime) {
            primes.push_back(number);
        }

        number += 2;
    }

    return primes.back();
}
