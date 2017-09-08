package bob

import "unicode"

const testVersion = 3

// Hey function reply a greeting
func Hey(greeting string) string {
	containsUpper := false
	containsLower := false
	lastNonspace := ' '

	for _, v := range greeting {
		if v == ' ' {
			continue
		}

		if unicode.IsLetter(v) {
			if unicode.IsUpper(v) {
				containsUpper = true
			} else {
				containsLower = true
			}
		}

		lastNonspace = v
	}

	isShouting := containsUpper && !containsLower
	isAsking := lastNonspace == '?'
	isAddressing := unicode.IsSpace(lastNonspace)

	if isShouting {
		return "Whoa, chill out!"
	}

	if isAsking {
		return "Sure."
	}

	if isAddressing {
		return "Fine. Be that way!"
	}

	return "Whatever."
}
