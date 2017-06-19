//
//
//
#include "boost/date_time/posix_time/posix_time.hpp"

namespace gigasecond {
    boost::posix_time::ptime advance(const boost::posix_time::ptime base){
        const auto gigasecond = 1000 * 1000 * 1000;
        return base + boost::posix_time::seconds(gigasecond);
    };
}