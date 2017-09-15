package summultiples

const testVersion = 2

// SumMultiples works out the sum value
func SumMultiples(limt int, vec ...int) int {
	sum := 0

	for i := 0; i < limt; i++ {
		for _, v := range vec {
			if i%v == 0 {
				sum += i
				break
			}
		}
	}

	return sum
}
