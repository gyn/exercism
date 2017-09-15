package triangle

import "math"

const testVersion = 3

// Kind is type for the constant variables below
type Kind uint

// constant variables for the return value of KindFromSides
const (
	NaT = iota // not a triangle
	Equ        // equilateral
	Iso        // isosceles
	Sca        // scalene
)

func isEqual(a, b float64) bool {
	const TOLERANCE = 1e-6
	if diff := math.Abs(a - b); diff > TOLERANCE {
		return false
	}

	return true
}

func isValidTriangle(a, b, c float64) bool {
	if a <= 0 || b <= 0 || c <= 0 {
		return false
	}

	if math.IsNaN(a) || math.IsNaN(b) || math.IsNaN(c) ||
		math.IsInf(a, 1) || math.IsInf(b, 1) || math.IsInf(c, 1) {
		return false
	}

	if (a+b) < c || (a+c) < b || (b+c) < a {
		return false
	}

	return true
}

// KindFromSides checks what kind of triangle a, b,c could build
func KindFromSides(a, b, c float64) Kind {
	if !isValidTriangle(a, b, c) {
		return NaT
	}

	if isEqual(a, b) && isEqual(b, c) {
		return Equ
	}

	if isEqual(a, b) || isEqual(b, c) || isEqual(c, a) {
		return Iso
	}

	return Sca
}
