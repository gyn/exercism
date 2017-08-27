#ifndef ALLERGIES_H
#define ALLERGIES_H

#include <cstdint>
#include <string>
#include <map>
#include <unordered_set>
#include <vector>

namespace allergies {
    class allergy_test {
    public:
        explicit allergy_test(uint32_t v) : mark(v) {}

    public:
        bool is_allergic_to(const std::string &food) const;

        std::unordered_set<std::string> get_allergies() const;

    private:
        static const std::vector<std::string> vec;
        static const std::map<std::string, int> map;

    private:
        uint32_t mark;
    };
}

#endif//ALLERGIES_H