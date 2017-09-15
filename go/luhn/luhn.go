package luhn

import (
	"unicode"
)

const testVersion = 2

// Valid returns if number is a valid SIN
func Valid(number string) bool {
	var digits []int

	for _, c := range number {
		if unicode.IsSpace(c) {
			continue
		}

		if !unicode.IsDigit(c) {
			return false
		}

		digits = append(digits, int(c-'0'))
	}

	if len(digits) < 2 {
		return false
	}

	sum := 0
	ruler := len(digits) & 1
	for i, v := range digits {
		if i&1 == ruler {
			v *= 2

			if v > 9 {
				v -= 9
			}
		}

		sum += v
	}

	return sum%10 == 0
}
