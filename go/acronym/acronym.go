package acronym

import (
	"bytes"
	"unicode"
)

const testVersion = 3

// Abbreviate make a abbreviation from text
func Abbreviate(text string) string {
	var buffer bytes.Buffer

	nextCapital := true
	prevLowercase := false

	for _, v := range text {
		if nextCapital {
			nextCapital = false

			buffer.WriteRune(unicode.ToUpper(v))

			continue
		}

		if v == '-' || v == ' ' {
			nextCapital = true

			continue
		}

		if unicode.IsUpper(v) && !nextCapital && prevLowercase {
			prevLowercase = false

			buffer.WriteRune(unicode.ToUpper(v))

			continue
		}

		if unicode.IsLower(v) && !nextCapital {
			prevLowercase = true

			continue
		}
	}

	return buffer.String()
}
