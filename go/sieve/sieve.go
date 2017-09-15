package sieve

import (
	"math"
)

const testVersion = 1

// Sieve return all the prime numbers less than or equal limit
func Sieve(limit int) []int {
	if limit < 2 {
		return nil
	}

	upper := math.Ceil(math.Sqrt(float64(limit)))

	board := make([]int, limit)

	for i := 2; i < int(upper); i++ {
		if board[i-2] == 1 {
			continue
		}

		for v := i * i; v <= limit; v += i {
			board[v-2] = 1
		}

	}

	var result []int

	for i := 2; i <= limit; i++ {
		if board[i-2] == 0 {
			result = append(result, i)
		}
	}

	return result
}
