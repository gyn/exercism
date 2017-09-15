package diffsquares

const testVersion = 1

// SquareOfSums returns the (1 + 2 + ... + n)^2
func SquareOfSums(v int) int {
	r := (v + 1) * v / 2
	return r * r
}

// SumOfSquares returns the (1^2 + 2^2 + ... + n^2)
func SumOfSquares(v int) int {
	r := v * (v + 1) * (2*v + 1) / 6
	return r
}

// Difference returns (1 + 2 + ... + n)^2 - (1^2 + 2^2 + ... + n^2)
func Difference(v int) int {
	r := v * (6*v + 4) * (v + 1) * (v - 1) / 24
	return r
}
