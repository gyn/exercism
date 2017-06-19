//
//
//
#include <string>

class phone_number {
public:
    phone_number(const std::string);

public:
    std::string number() const;
    std::string area_code() const;

    operator std::string() const;

private:
    std::string digits;
};