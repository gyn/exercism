//
//
//
#include <cmath>
#include <stdexcept>
#include <limits>

namespace triangle {
    enum triangle_type {
        equilateral,
        isosceles,
        scalene,
    };

    //
    // kind function for integer parameters
    //
    enum triangle_type kind(int a, int b, int c) {
        if ((a <= 0) || (b <= 0) || (c <= 0)) {
            throw std::domain_error("invalid length");
        }

        if ((a + b < c) || (a + c < b) || (c + b < a)) {
            throw std::domain_error("impossible triangle");
        }

        if ((a == b) && (a == c)) {
            return equilateral;
        }

        if ((a == b) || (a == c) || (b == c)) {
            return isosceles;
        }

        return scalene;
    }

    //
    // kind function for float point parameters
    //
    static inline bool float_equal(double x, double y) {
        return std::fabs(x - y) < std::numeric_limits<double>::epsilon();

    }

    enum triangle_type kind(double a, double b, double c) {
        if ((a < 0) || (b < 0) || (c < 0)) {
            throw std::domain_error("invalid length");
        }

        if (float_equal(a, 0) || float_equal(b, 0) || float_equal(c, 0)) {
            throw std::domain_error("invalid length");
        }

        if ((a + b < c) || (a + c < b) || (c + b < a)) {
            throw std::domain_error("impossible triangle");
        }

        if (float_equal(a, b) && float_equal(a, c)) {
            return  equilateral;
        }

        if (float_equal(a, b) || float_equal(a, c) || float_equal(b, c)) {
            return isosceles;
        }

        return scalene;
    }
}