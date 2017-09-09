//
//
//
#ifndef CLOCK_H
#define CLOCK_H

#include <string>


namespace date_independent {
    class clock {
    public:
        clock(int hour, int minutes) : hour(hour), minutes(minutes) {}

    public:
        static clock at(int hour, int minutes);

        clock &plus(int minutes);

        operator std::string() const;

        bool operator==(const clock &rhs) const;

        bool operator!=(const clock &rhs) const;

    private:
        int hour;
        int minutes;
    };
}
#endif