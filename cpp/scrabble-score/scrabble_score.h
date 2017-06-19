//
//
//
#include <string>

namespace scrabble_score {
    int score(std::string word) {
        int total_score = 0;

        for (const auto &c : word) {
            auto v = tolower(c);

            switch (v) {
                case 'a' :
                case 'e' :
                case 'i' :
                case 'o' :
                case 'u' :
                case 'l' :
                case 'n' :
                case 'r' :
                case 's' :
                case 't' :
                    total_score += 1;
                    break;

                case 'd' :
                case 'g' :
                    total_score += 2;
                    break;

                case 'b' :
                case 'c' :
                case 'm' :
                case 'p' :
                    total_score += 3;
                    break;

                case 'f' :
                case 'h' :
                case 'v' :
                case 'w' :
                case 'y' :
                    total_score += 4;
                    break;

                case 'k' :
                    total_score += 5;
                    break;

                case 'j' :
                case 'x' :
                    total_score += 8;
                    break;

                case 'q' :
                case 'z' :
                    total_score += 10;
                    break;

                default:
                    break;
            }
        }

        return total_score;
    }
}