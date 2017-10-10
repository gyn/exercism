package allergies

import (
	"math/bits"
)

const testVersion = 1

var table = [...]string{
	"eggs",
	"peanuts",
	"shellfish",
	"strawberries",
	"tomatoes",
	"chocolate",
	"pollen",
	"cats",
}

// Allergies returns the food list for a certain number
func Allergies(number uint) []string {
	var result []string
	for i := 0; i < len(table); i++ {
		if number&(1<<uint(i)) != 0 {
			result = append(result, table[i])
		}
	}

	return result
}

// AllergicTo returns if a certain number is allergic to the food
func AllergicTo(number uint, food string) bool {
	for r := bits.TrailingZeros(number); r != bits.UintSize; {
		if food == table[r] {
			return true
		}

		number ^= 1 << uint(r)
		r = bits.TrailingZeros(number)
	}

	return false
}
