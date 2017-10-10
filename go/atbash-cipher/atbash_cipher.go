package atbash

import (
	"unicode"
)

const testVersion = 2

// Atbash encodes text into a string
func Atbash(text string) string {
	var result []rune
	count := 0
	for _, v := range text {
		if unicode.IsLetter(v) {
			result = append(result, 'z'-unicode.ToLower(v)+'a')
			count++
		}

		if unicode.IsDigit(v) {
			result = append(result, v)
			count++
		}

		if count == 5 {
			result = append(result, ' ')
			count = 0
		}
	}

	if unicode.IsSpace(result[len(result)-1]) {
		result = result[:len(result)-1]
	}

	return string(result)
}
