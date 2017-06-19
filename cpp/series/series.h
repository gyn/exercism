//
//
//
#ifndef SERIES_H
#define SERIES_H

#include <string>
#include <vector>
#include <stdexcept>


namespace series {
    std::vector<int> digits(const std::string &number) {
        std::vector<int> result;

        for (const auto &d : number) {
            const int v = d - '0';

            result.push_back(v);
        }

        return result;
    }

    std::vector<std::vector<int>> slice(std::string number, std::string::size_type length) {
        if (number.size() < length) {
            throw std::domain_error("Too large length");
        }

        const auto size = number.size() - length + 1;

        std::vector<std::vector<int>> result;

        for (auto i = 0; i < size; i++) {
            auto s = number.substr(i, length);

            auto r = digits(s);

            result.push_back(r);
        }

        return result;
    }
}
#endif