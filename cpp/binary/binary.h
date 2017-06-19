//
//
//
#ifndef BINARY_H
#define BINARY_H

#include <cstdint>
#include <string>

namespace binary {
    uint64_t convert(std::string data) {
        //
        // convert functions only supports 64-bit data
        //
        if (data.size() > 64) {
            return 0;
        }

        uint64_t number{0};

        for (const auto &c : data) {
            number = (number << 1);

            switch (c) {
                case '0':
                    break;

                case '1':
                    number += 1;
                    break;

                default:
                    // it is not a valid binary number
                    return 0;
                    // some lint tool like this
                    break;
            }
        }

        return number;
    }
}

#endif