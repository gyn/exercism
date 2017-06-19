//
//
//

#ifndef HEXADECIMAL_H
#define HEXADECIMAL_H


#include <cstdint>
#include <string>


namespace hexadecimal {
    uint32_t convert(std::string number) {
        uint32_t result = 0;

        for (const auto &c : number) {
            int digit = 0;
            if (c >= '0' && c <= '9') {
                digit = c - '0';
            } else if (c >= 'a' && c <= 'f') {
                digit = c - 'a' + 10;
            } else if (c >= 'A' && c <= 'F') {
                digit = c - 'A' + 10;
            } else {
                return 0;
            }

            result = (result << 4) + digit;
        }

        return result;
    }
}

#endif