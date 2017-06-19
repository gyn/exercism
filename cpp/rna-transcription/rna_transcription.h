//
//
//
#ifndef RNA_TRANSCRIPTION_H
#define RNA_TRANSCRIPTION_H

#include <string>
#include <algorithm>
#include <stdexcept>


namespace transcription {
    char to_rna(char nucleotide) {
        char result;

        switch (nucleotide) {
            case 'A':
                result = 'U';
                break;

            case 'C':
                result = 'G';
                break;

            case 'G':
                result = 'C';
                break;

            case 'T':
                result = 'A';
                break;

            default:
                throw std::invalid_argument("Invalid nucleotide");
        }

        return result;
    }

    std::string to_rna(const std::string &dna) {
        std::string rna;

        std::transform(dna.cbegin(),
                       dna.cend(),
                       std::back_inserter(rna),
                       static_cast<char(*)(char)>(to_rna));

        return rna;
    }
}
#endif