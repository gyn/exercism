//
//
//
#include <vector>
#include <numeric>

#include "food_chain.h"

namespace food_chain {
    const static std::vector<std::string> animals {
        "fly", "spider", "bird", "cat", "dog", "goat", "cow", "horse"
    };

    const static std::vector<std::string> second {
        "",
        "It wriggled and jiggled and tickled inside her.\n",
        "How absurd to swallow a bird!\n",
        "Imagine that, to swallow a cat!\n",
        "What a hog, to swallow a dog!\n",
        "Just opened her throat and swallowed a goat!\n",
        "I don't know how she swallowed a cow!\n",
        "She's dead, of course!\n"
    };

    const static std::string last {
        "I don't know why she swallowed the fly. Perhaps she'll die.\n"
    };

    std::string verse(unsigned long number) {
        auto index = number - 1;

        std::string result = "I know an old lady who swallowed a " + animals.at(index) + ".\n";

        result += second.at(index);

        if (number < 8) {
            for (int i = number; i > 1; i--) {
                result += "She swallowed the " + animals[i - 1] + " to catch the " + animals[i - 2];
                if (i == 3) {
                    result += " that wriggled and jiggled and tickled inside her";
                }

                result += ".\n";
            }

            result += "I don't know why she swallowed the fly. Perhaps she'll die.\n";
        }

        return result;
    }

    std::string verses(unsigned long lower, unsigned long upper) {
        std::string result;

        for (auto i = lower; i <= upper; i++) {
            result += verse(i) + "\n";
        }

        return result;
    }

    std::string sing() {
        return verses(1, 8);
    }
}
