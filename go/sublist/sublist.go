package sublist

import (
	"reflect"
)

// Relation is a string for the relationship between two lists
type Relation string

func search(haystack, needle []int) bool {
	max := len(haystack) - len(needle) + 1
	for i := 0; i < max; i++ {
		if reflect.DeepEqual(haystack[i:i+len(needle)], needle) {
			return true
		}
	}

	return false
}

// Sublist check the relationship between two lists
func Sublist(lista, listb []int) Relation {
	var needle []int
	var haystack []int
	var result Relation

	if len(lista) < len(listb) {
		needle = lista
		haystack = listb
		result = "sublist"
	} else if len(lista) > len(listb) {
		needle = listb
		haystack = lista
		result = "superlist"
	} else {
		needle = lista
		haystack = listb
		result = "equal"
	}

	if len(needle) == 0 || search(haystack, needle) {
		return result
	}

	return "unequal"
}
