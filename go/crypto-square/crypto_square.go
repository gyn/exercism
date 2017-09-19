package cryptosquare

import (
	"unicode"
)

const testVersion = 2

func matrix(length int) (int, int) {
	for i := 1; i <= length; i++ {
		if i*i >= length {
			return i, i
		}

		if i*(i-1) >= length {
			return i, i - 1
		}
	}

	return 0, 0
}

// Encode return encoded text
func Encode(text string) string {
	var letters []rune

	for _, v := range []rune(text) {
		if unicode.IsLetter(v) || unicode.IsDigit(v) {
			letters = append(letters, unicode.ToLower(v))
		}
	}

	rows, columns := matrix(len(letters))

	var result []rune

	for i := 0; i < rows; i++ {
		for j := 0; j < columns; j++ {
			pos := j*rows + i
			if pos < len(letters) {
				result = append(result, letters[pos])
			}
		}

		if i != rows-1 {
			result = append(result, ' ')
		}
	}

	return string(result)
}
