//
//
//
#ifndef SAY_H
#define SAY_H

#include <string>
#include <map>
#include <vector>
#include <numeric>

namespace say {
    static inline std::vector<int> delimit(long long number) {
        std::vector<int> result;

        do {
            const auto THOUSAND = 1000;

            const auto remainder = number % THOUSAND;

            number /= THOUSAND;

            result.push_back(remainder);
        } while (number);

        return result;
    }

    static std::map<int, std::string> dictionay {
            {0, "zero"},      {1, "one"},        {2, "two"},       {3, "three"},
            {4, "four"},      {5, "five"},       {6, "six"},       {7, "seven"},
            {8, "eight"},     {9, "nine"},       {10, "ten"},      {11, "eleven"},
            {12, "twelve"},   {13, "thirteen"},  {14, "fourteen"}, {15, "fifteen"},
            {16, "sixteen"},  {17, "seventeen"}, {18, "eighteen"}, {19, "nineteen"},
            {20, "twenty"},   {30, "thirty"},    {40, "forty"},    {50, "fifty"},
            {60, "sixty"},    {70, "seventy"},   {80, "eighty"},   {90, "ninety"},
    };

    static inline std::string translate(long long number) {
        std::string result;

        //
        // deal with the first digits
        //
        const auto HUNDRED = 100;

        auto hundreds = number/HUNDRED;
        auto left     = number - hundreds * HUNDRED;

        if (hundreds) {
            result += dictionay[hundreds] + " hundred";

            if (left == 0) {
                return result;
            } else {
                result += ' ';
            }
        }

        //
        // the last 2 digits
        //
        const auto TEN = 10;

        auto digit = left % TEN;

        if ((left < 20) || (digit == 0)) {
            result += dictionay[left];
        } else {
            result += dictionay[left - digit] + "-" + dictionay[digit];
        }

        return result;
    }

    static inline std::vector<std::string> add_units(std::vector<int> &numbers) {
        std::vector<std::string> vector;

        const auto length = numbers.size();

        const std::vector<std::string> units {"", " thousand", " million", " billion"};

        for (auto i = 0; i < length; i++) {
            auto v = numbers[i];

            if (v == 0) {
                continue;
            }

            auto translation = translate(v) + units[i];

            vector.push_back(translation);
        }

        return vector;
    }

    std::string in_english(long long value) {
        const auto ZERO = 0;

        if (value < ZERO) {
            throw std::domain_error("Negative value");
        }

        const auto TRILLION = 1000ULL*1000ULL*1000ULL*1000ULL;

        if (value >= TRILLION) {
            throw std::domain_error("Too large value");
        }

        //
        // Deal with the number between 0 and 1000
        //
        const auto THOUSAND = 1000;

        if (value < THOUSAND) {
            return translate(value);
        }

        //
        // Deal with the number between 1000 and 1000ULL*1000ULL*1000ULL*1000ULL
        //
        auto numbers = delimit(value);

        auto translation = add_units(numbers);

        auto result = std::accumulate(std::next(translation.begin()),
                                      translation.end(),
                                      translation[0],
                                      [](std::string a, std::string b) { return b + " " + a; }
        );

        return result;
    }
}
#endif