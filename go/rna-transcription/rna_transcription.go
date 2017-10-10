package strand

import "strings"

const testVersion = 3

// ToRNA translate dna to rna serial
func ToRNA(dna string) string {
	mapFunction := func(d rune) rune {
		switch d {
		case 'C':
			return 'G'
		case 'G':
			return 'C'
		case 'T':
			return 'A'
		case 'A':
			return 'U'
		}

		return ' '
	}

	return strings.Map(mapFunction, dna)
}
