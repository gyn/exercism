//
//
//
#include <string>
#include <stdexcept>

namespace hamming {
    int compute(const std::string a, const std::string b) {
        if (a.length() != b.length()) {
            throw std::domain_error("Length is not same");
        };

        auto count = 0;

        for (auto i = 0; i < a.size(); i++) {
            if (a[i] != b[i]) {
                count++;
            }
        }

        return count;
    };
}