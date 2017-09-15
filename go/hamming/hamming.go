package hamming

import (
	"errors"
	"unicode/utf8"
)

const testVersion = 6

// Distance count the different characters between string a and b
func Distance(a, b string) (int, error) {
	if len(a) != len(b) {
		return 0, errors.New("Length is not same")
	}

	count := 0
	for i := 0; i < utf8.RuneCountInString(a); i++ {
		if a[i] != b[i] {
			count++
		}
	}

	return count, nil
}
