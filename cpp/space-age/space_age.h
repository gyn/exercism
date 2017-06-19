//
//
//
#include <cstdint>

namespace space_age {
    class space_age {
    public:
        space_age(uint64_t sec);

    public:
        uint64_t seconds() const;

    public:
        double on_earth() const;
        double on_mercury() const;
        double on_venus() const;
        double on_mars() const;
        double on_jupiter() const;
        double on_saturn() const;
        double on_uranus() const;
        double on_neptune() const;

    private:
        static constexpr double earth_orbital_period   = 31557600.0;
        static constexpr double mercury_orbital_period = 0.2408467 * earth_orbital_period;
        static constexpr double venus_orbital_period   = 0.61519726 * earth_orbital_period;
        static constexpr double mars_orbital_period    = 1.8808158 * earth_orbital_period;
        static constexpr double jupiter_orbital_period = 11.862615 * earth_orbital_period;
        static constexpr double saturn_orbital_period  = 29.447498 * earth_orbital_period;
        static constexpr double uranus_orbital_period  = 84.016846 * earth_orbital_period;
        static constexpr double neptune_orbital_period = 164.79132 * earth_orbital_period;

    private:
        uint64_t period;
    };

    space_age::space_age(uint64_t seconds) : period(seconds) {
    }

    uint64_t space_age::seconds() const {
        return period;
    }

    double space_age::on_earth() const {
        return period / earth_orbital_period;
    }

    double space_age::on_mercury() const {
        return period / mercury_orbital_period;
    }

    double space_age::on_venus() const {
        return period / venus_orbital_period;
    }

    double space_age::on_mars() const {
        return period / mars_orbital_period;
    }

    double space_age::on_jupiter() const {
        return period / jupiter_orbital_period;
    }

    double space_age::on_saturn() const {
        return period / saturn_orbital_period;
    }

    double space_age::on_uranus() const {
        return period / uranus_orbital_period;
    }

    double space_age::on_neptune() const {
        return period / neptune_orbital_period;
    }
}