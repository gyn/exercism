//
//
//
#include <vector>
#include <map>

namespace etl {
    using etl_input  = std::map<int, std::vector<char>>;
    using etl_result = std::map<char, int>;

    etl_result transform(const etl_input stats) {
        etl_result result;

        for (const auto & kv : stats) {
            for (const auto c : kv.second) {
                result[std::tolower(c)] = kv.first;
            }
        }

        return result;
    }
}