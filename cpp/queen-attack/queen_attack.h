//
//
//
#include <utility>
#include <string>
#include <stdexcept>


namespace queen_attack {
    class chess_board {
    public:
        chess_board() : white_pair{0, 3}, black_pair{7, 3} { };

        chess_board(std::pair<int, int> w,  std::pair<int, int> b) : white_pair(w), black_pair(b) {
            if (b == w) {
                throw std::domain_error("duplicated postion");
            }
        }

    public:
        std::pair<int, int> white() const { return white_pair; };
        std::pair<int, int> black() const { return black_pair; };

        bool can_attack() const {
            if (white_pair.first == black_pair.first) { return true; }
            if (white_pair.second == black_pair.second) { return true; }

            const auto xdelta = std::abs(white_pair.first - black_pair.first);
            const auto ydelta = std::abs(white_pair.second - black_pair.second);

            return (xdelta == ydelta);
        }

    public:
        explicit operator std::string() const {
            std::string board = "_ _ _ _ _ _ _ _\n"
                                "_ _ _ _ _ _ _ _\n"
                                "_ _ _ _ _ _ _ _\n"
                                "_ _ _ _ _ _ _ _\n"
                                "_ _ _ _ _ _ _ _\n"
                                "_ _ _ _ _ _ _ _\n"
                                "_ _ _ _ _ _ _ _\n"
                                "_ _ _ _ _ _ _ _\n";

            auto pos = white_pair.first * 16 + white_pair.second * 2;

            board[pos] = 'W';

            pos = black_pair.first * 16 + black_pair.second * 2;

            board[pos] = 'B';

            return board;
        }

    private:
        std::pair<int, int> white_pair;
        std::pair<int, int> black_pair;
    };
}