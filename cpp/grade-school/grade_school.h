//
//
//
#include <string>
#include <vector>
#include <map>
#include <algorithm>

namespace grade_school {
    using name_list = std::vector<std::string>;
    using result    = std::map<int, std::vector<std::string>>;

    class school {
    public:
        void add(std::string name, int grade) {
            auto & l = db[grade];

            auto upper = std::upper_bound(l.begin(), l.end(), name);

            l.insert(upper, name);
        };

        result roster() const {
            return db;
        };

        name_list grade(int grade) {
            auto list = db[grade];

            return list;
        };

    private:
        result db;
    };
}