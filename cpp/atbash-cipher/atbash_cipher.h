#ifndef ATASH_CIPHER_H
#define ATASH_CIPHER_H

#include <string>

namespace atbash {
    std::string encode(std::string input) {
        int index = 1;
        std::string output;

        for (auto c : input) {
            switch (c) {
                case 'a'...'z':
                    output.push_back('z' - c + 'a');
                    break;
                case 'A'...'Z':
                    output.push_back('Z' - c + 'a');
                    break;
                case '0'...'9':
                    output.push_back(c);
                    break;
                default:
                    continue;
            }

            index++;
            if (index == 6) {
                output.push_back(' ');
                index = 1;
            }
        }

        if (output.back() == ' ') {
            output.pop_back();
        }

        return output;
    }

    std::string decode(std::string input) {
        std::string output;

        for (auto c : input) {
            switch (c) {
                case 'a'...'z':
                    output.push_back('z' - c + 'a');
                    break;
                case 'A'...'Z':
                    output.push_back('Z' - c + 'A');
                    break;
                case '0'...'9':
                    output.push_back(c);
                    break;
                default:
                    continue;
            }
        }

        return output;

    }
}

#endif//ATASH_CIPHER_H