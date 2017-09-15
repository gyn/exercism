package grains

import (
	"errors"
	"math"
)

const testVersion = 1

// Square return the numbers in the square indexed by shift
// and error. If shift is a invalid, error is a error message
func Square(shift int) (uint64, error) {
	if shift <= 0 || shift > 64 {
		return 0, errors.New("invalid number")
	}

	return 1 << uint32(shift-1), nil
}

// Total return sum of the number in all squares
func Total() uint64 {
	return math.MaxUint64
}
