package pythagorean

const testVersion = 1

// Triplet records the length for three side of a triangle
type Triplet struct {
	a int
	b int
	c int
}

// Range returns all the possible triangles that a^2 + b^2 = c^2
// and a, b, c are bewtten lower and upper
func Range(lower, upper int) []Triplet {
	var result []Triplet

	for c := lower; c < upper+1; c++ {
		for b := lower; b < c; b++ {
			for a := lower; a < b; a++ {
				if a*a+b*b != c*c {
					continue
				}

				result = append(result, Triplet{a, b, c})
			}
		}
	}

	return result
}

// Sum returns all the possible triangles that a^2 + b^2 = c^2
// and a + b + c = sum
func Sum(sum int) []Triplet {
	var result []Triplet

	lower := sum / 3
	upper := sum / 2

	for c := upper - 1; c >= lower; c-- {
		for b := c / 2; b < c; b++ {
			for a := 1; a < b; a++ {
				if a*a+b*b != c*c {
					continue
				}

				if a+b+c != sum {
					continue
				}

				result = append(result, Triplet{a, b, c})
			}
		}
	}

	return result
}
