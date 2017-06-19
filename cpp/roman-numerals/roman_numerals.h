//
//
//
#ifndef ROMAN_NUMERALS_H
#define ROMAN_NUMERALS_H

#include <string>
#include <cassert>
#include <vector>

namespace roman {
    using digit_map  = std::vector<std::string>;
    using digit_pair = std::pair<int ,digit_map>;

    std::string convert(unsigned int number) {
        assert(number <= 3000);

        const std::vector<digit_pair> map{
                                          {1000, {"", "M", "MM", "MMM"}},
                                          {100,  {"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"}},
                                          {10,   {"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"}},
                                          {1,    {"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"}}
                                         };

        std::string roman_number;

        for (auto &m : map) {
            auto i = number / m.first;

            roman_number += m.second[i];

            number -= i * m.first;
        }

        return roman_number;
    }
}

#endif