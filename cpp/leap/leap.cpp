//
//
//
#include "leap.h"

bool leap::is_leap_year(int year)
{
    //
    // Check the algorithm at https://en.wikipedia.org/wiki/Leap_year
    //
    if (year % 4) { return false;}
    if (year % 100) { return true; }
    if (year % 400) { return false; }
    return true;
}