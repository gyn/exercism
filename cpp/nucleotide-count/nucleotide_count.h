//
//
//
#include <string>
#include <map>
#include <algorithm>
#include <stdexcept>
#include <iostream>

namespace dna {
    class counter {
    public:
        counter(std::string serial);
    public:
        int count(char code) const;
        std::map<char, int> nucleotide_counts() const;

    private:
        std::map<char, int> stats;
    };

    static void checkCode(char code) {
        switch (code) {
            case 'A': break;
            case 'T': break;
            case 'C': break;
            case 'G': break;
            default : throw std::invalid_argument("unknown code");
        }
    }

    counter::counter(std::string serial) {
        stats['A'] = 0;
        stats['T'] = 0;
        stats['C'] = 0;
        stats['G'] = 0;

        for (const auto & c : serial) {
            checkCode(c);

            stats[c] ++;
        }
    }

    std::map<char, int> counter::nucleotide_counts() const {
        return stats;
    }

    int counter::count(const char code) const {
        checkCode(code);

        int count = stats.at(code);

        return count;
    }
}