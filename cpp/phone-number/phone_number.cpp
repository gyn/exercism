//
//
//
#include <algorithm>
#include "phone_number.h"

#include <iostream>

phone_number::phone_number(const std::string str) {
    std::copy_if(str.cbegin(),
                 str.cend(),
                 std::back_inserter(digits),
                 [](const char c){ return std::isdigit(c); });

    auto size = digits.size();

    static const size_t digits10 = 10;
    static const size_t digits11 = 11;

    switch (size) {
        case digits10:
            break;

        case digits11:
            if (digits.front() == '1') {
                digits.erase(0, 1);
            } else {
                digits = "0000000000";
            }

            break;

        default:
            digits = "0000000000";

            break;
    }
}

std::string phone_number::number() const {
    return digits;
}

std::string phone_number::area_code() const {
    return digits.substr(0, 3);
}

phone_number::operator std::string() const {
    return "(" +
            digits.substr(0, 3) +
            ") " +
            digits.substr(3, 3) +
            "-" +
            digits.substr(6);
}