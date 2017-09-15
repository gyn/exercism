package etl

import "strings"

const testVersion = 1

// Transform transform a map[int][]string to a map[string]int
func Transform(input map[int][]string) map[string]int {
	result := make(map[string]int)

	for k, v := range input {
		for _, s := range v {
			result[strings.ToLower(s)] = k
		}
	}

	return result
}
