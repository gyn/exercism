package pangram

import "unicode"

const testVersion = 2

// IsPangram check if text is a pangram
func IsPangram(text string) bool {
	result := 0

	for _, v := range text {
		if unicode.IsLetter(v) {
			shift := uint(unicode.ToLower(v)) - 'a'
			result |= 1 << shift
		}
	}

	return result == (1<<26 - 1)
}
