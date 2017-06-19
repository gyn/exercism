//
//
//
#include "bob.h"

std::string bob::hey(std::string message)
{
    auto containsUpper = false;
    auto containsLower = false;
    auto lastNonSpace  = ' ';

    for (auto &c : message) {
        if (std::isspace(c)) {
            continue;
        }

        if (std::isalpha(c)) {
            if (std::isupper(c)) {
                containsUpper = true;
            } else {
                containsLower = true;
            }
        }

        lastNonSpace = c;
    }

    const auto isShouting   = containsUpper && !containsLower;
    const auto isAsking     = (lastNonSpace == '?');
    const auto isAddressing = (lastNonSpace == ' ');

    if (isShouting) {
        return "Whoa, chill out!";
    }

    if (isAsking) {
        return "Sure.";
    }

    if (isAddressing) {
        return "Fine. Be that way!";
    }

    return "Whatever.";
}
