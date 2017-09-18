package perfect

import (
	"errors"
)

const testVersion = 1

// Classification is for the return value for Classify function
type Classification uint

//
// Classification results
//
const (
	ClassificationDeficient = iota
	ClassificationAbundant
	ClassificationPerfect
)

// ErrOnlyPositive is the error when number is 0
var ErrOnlyPositive = errors.New("positive number only")

// Classify classify number into Classification grades
func Classify(number uint64) (Classification, error) {
	if number < 1 {
		return 0, ErrOnlyPositive
	}

	factors := []uint64{}
	for i := uint64(1); i <= number/2; i++ {
		if number%i == 0 {
			factors = append(factors, i)
		}
	}

	sum := uint64(0)
	for _, v := range factors {
		sum += v
	}

	if sum == number {
		return ClassificationPerfect, nil
	} else if sum > number {
		return ClassificationAbundant, nil
	} else {
		return ClassificationDeficient, nil
	}
}
