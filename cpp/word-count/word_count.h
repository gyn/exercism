//
//
//
#include <map>
#include <string>
#include <regex>
#include <iostream>

namespace word_count {
    const std::regex re("[A-Za-z0-9]+('[A-Za-z]+)?");

    std::map<std::string, int> words(std::string phrase) {
        std::transform(phrase.begin(), phrase.end(),
                       phrase.begin(), ::tolower);

        std::smatch sm;
        std::map<std::string, int> stats;
        while(std::regex_search(phrase, sm, re)) {
            auto result = sm.str(0);

            stats[result]++;

            phrase = sm.suffix();
        }

        return stats;
    };
}
