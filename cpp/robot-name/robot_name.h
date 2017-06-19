//
//
//
#include <string>
#include <random>
#include <sstream>
#include <iomanip>

namespace robot_name {
    class robot {
    public:
        robot() {
            random_tag();
        };

    public:
        std::string name() const {
            return tag;
        }

        void reset() {
            random_tag();
        }

    private:
        void random_tag() {
            //
            // empty the tag
            //
            tag.clear();

            std::random_device rd;

            std::mt19937 gen(rd());

            const auto base = 'A';
            const auto limit = 'Z';

            std::uniform_int_distribution<char> alpha_dist(base, limit);

            tag.push_back(alpha_dist(gen));
            tag.push_back(alpha_dist(gen));

            std::uniform_int_distribution<int> digits_dist(1, 999);

            std::ostringstream buf;

            buf << std::setfill('0') << std::setw(3) << digits_dist(gen);

            tag += buf.str();
        }

    private:
        std::string tag;
    };
}