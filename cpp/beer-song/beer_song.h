//
//
//

#ifndef BEER_SONG_H
#define BEER_SONG_H

#include <string>


namespace beer {
    std::string verse(int no) {
        std::string first;
        std::string second;
        std::string third;
        std::string last;

        switch (no) {
            case 2:
                first  = "2 bottles";
                second = "2 bottles";
                third  = "Take one down and pass it around, ";
                last   = "1 bottle of";
                break;

            case 1:
                first  = "1 bottle";
                second = "1 bottle";
                third  = "Take it down and pass it around, ";
                last   = "no more bottles of";
                break;

            case 0:
                first  = "No more bottles";
                second = "no more bottles";
                third  = "Go to the store and buy some more, ";
                last   = "99 bottles of";
                break;

            default:
                first  = std::to_string(no) + " bottles";
                second = std::to_string(no) + " bottles";
                third  = "Take one down and pass it around, ";
                last   = std::to_string(no - 1) + " bottles of";
                break;
        }

        return first + " of beer on the wall, " + second + " of beer.\n" +
               third + last + " beer on the wall.\n";
    }

    std::string sing(int upper, int lower = 0) {
        std::string result;

        for (auto i = upper; i >= lower; i--) {
            result += verse(i) + "\n";
        }

        // Remove the last LF
        result.pop_back();

        return result;
    }
}
#endif