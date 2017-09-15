package isogram

import "unicode"

const testVersion = 1

// IsIsogram check if a word is an isogram
func IsIsogram(word string) bool {
	dict := make(map[rune]int)

	for _, v := range word {
		if !unicode.IsLetter(v) {
			continue
		}

		r := unicode.ToLower(v)
		if t, ok := dict[r]; !ok {
			dict[r] = t + 1
		} else {
			return false
		}
	}

	return true
}
