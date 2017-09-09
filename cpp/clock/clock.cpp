//
//
//
#include <sstream>
#include <iomanip>
#include "clock.h"

static void normalize(int &hour, int &minutes) {
    const int HOURS = 24;
    const int MINUTES = 60;

    int c = 0;
    int m = minutes % MINUTES;
    if (m < 0) {
        m += 60;
        c = -1;
    }

    int h = (c + minutes / MINUTES + hour) % HOURS;
    if (h < 0) {
        h += 24;
    }

    hour = h;
    minutes = m;
}

namespace date_independent {
    clock clock::at(int hour, int minutes) {
        normalize(hour, minutes);

        clock c(hour, minutes);

        return c;
    }

    clock &clock::plus(int period) {
        minutes += period;

        normalize(hour, minutes);

        return *this;
    }

    clock::operator std::string() const {
        std::ostringstream buf;

        buf << std::setw(2) << std::setfill('0') << hour << ":"
            << std::setw(2) << std::setfill('0') << minutes;

        return buf.str();
    }

    bool clock::operator==(const clock &rhs) const {
        return hour == rhs.hour && minutes == rhs.minutes;
    }

    bool clock::operator!=(const clock &rhs) const {
        return hour != rhs.hour || minutes != rhs.minutes;
    }
}
