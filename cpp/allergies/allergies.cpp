#include "allergies.h"

namespace allergies {
    const std::map<std::string, int> allergy_test::map = {
            {"eggs",         0},
            {"peanuts",      1},
            {"shellfish",    2},
            {"strawberries", 3},
            {"tomatoes",     4},
            {"chocolate",    5},
            {"pollen",       6},
            {"cats",         7},
    };

    const std::vector<std::string> allergy_test::vec = {
            "eggs",
            "peanuts",
            "shellfish",
            "strawberries",
            "tomatoes",
            "chocolate",
            "pollen",
            "cats"
    };

    bool allergy_test::is_allergic_to(const std::string &food) const {
        auto postion = map.find(food);
        if (postion == map.end()) {
            return false;
        }

        auto result = mark & (1u << postion->second);

        return result != 0;
    }

    std::unordered_set<std::string> allergy_test::get_allergies() const {
        std::unordered_set<std::string> result;

        for (int i = 0; i < vec.size(); i++) {
            if ((mark & (1u << i)) != 0) {
                result.insert(vec[i]);
            }
        }

        return result;
    }
}