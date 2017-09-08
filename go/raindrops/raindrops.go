package raindrops

import "strconv"

const testVersion = 3

// Convert convert a number to raindrop-speak language
func Convert(number int) string {
	var result string

	if number%3 == 0 {
		result += "Pling"
	}

	if number%5 == 0 {
		result += "Plang"
	}

	if number%7 == 0 {
		result += "Plong"
	}

	if len(result) == 0 {
		return strconv.Itoa(number)
	}

	return result
}
