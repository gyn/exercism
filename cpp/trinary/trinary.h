//
//
//
#include <string>

namespace trinary {
    int to_decimal(const std::string &number) {
        int result = 0;

        for (const auto &c : number) {
            switch (c) {
                case '0':
                case '1':
                case '2':
                    result = result * 3 + c - '0';
                    break;

                default:
                    return 0;
                    //
                    //
                    //break;
            }
        }

        return result;
    }
};
