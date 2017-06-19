//
//
//
#ifndef CRYPTO_SQUARE_H
#define CRYPTO_SQUARE_H

#include <string>
#include <vector>


namespace crypto_square {
    using text_t      = std::string;
    using text_size_t = text_t::size_type;

    class cipher {
    public:
        cipher(text_t text);

    public:
        text_size_t size() const;
        text_t cipher_text() const;
        text_t normalized_cipher_text() const;
        text_t normalize_plain_text() const;
        std::vector<std::string> plain_text_segments() const;

    private:
        void squared();

    private:
        text_t      normal_text;
        text_size_t square_width;
        text_size_t square_length;
    };


    cipher::cipher(text_t text) {
        for (const auto &c : text) {
            if (std::isalnum(c)) {
                normal_text.push_back((char) std::tolower(c));
            }
        }

        squared();
    }

    void cipher::squared() {
        auto length = normal_text.size();
        decltype(length) i = 1;

        do {
            if (i * i == length) {
                square_width  = i;
                square_length = i;
                break;
            }

            if (i * (i - 1) >= length) {
                square_width  = i - 1;
                square_length = i;
                break;
            }

            i++;
        } while (i < length);
    }

    text_t cipher::normalize_plain_text() const {
        return normal_text;
    }

    text_size_t cipher::size() const {
        return square_length;
    }

    std::vector<text_t> cipher::plain_text_segments() const {
        std::vector<text_t> result;

        auto length = normal_text.size();
        for (decltype(length) i = 0; i < length; i += square_length) {
            auto n = ((length - i) < square_length) ? (length - i) : square_length;

            result.push_back(normal_text.substr(i, n));
        }

        return result;
    }

    text_t cipher::cipher_text() const {
        text_t result;

        auto length = normal_text.size();
        for (decltype(length) i = 0; i < square_width; i++) {
            for (decltype(length) j = 0; j < square_length; j++) {
                auto pos = j * square_width + i;
                if (pos >= length) {
                    continue;
                }

                auto v = normal_text[pos];

                result.push_back(v);
            }
        }

        return result;
    }

    text_t cipher::normalized_cipher_text() const {
        text_t result;

        auto length = normal_text.size();
        decltype(length) nr = 0;
        for (decltype(length) i = 0; i < square_length; i++) {
            for (decltype(length) j = 0; j < square_width; j++) {
                auto pos = j * square_length + i;
                if ( pos >= length) {
                    continue;
                }

                auto v = normal_text[pos];

                result.push_back(v);

                ++nr;

                if (((nr % square_length) == 0) && (nr != length)) {
                    result.push_back(' ');
                }
            }
        }

        return result;
    }
}
#endif