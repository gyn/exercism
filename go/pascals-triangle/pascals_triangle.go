package pascal

const testVersion = 1

func pascal(index int) []int {
	var result []int

	result = append(result, 1)

	for i := 0; i < index; i++ {
		v := result[i] * (index - i) / (i + 1)

		result = append(result, v)
	}

	return result
}

// Triangle returns a pascal triangle which has rows specified by limit
func Triangle(rows int) [][]int {
	var result [][]int
	for i := 0; i < rows; i++ {
		result = append(result, pascal(i))
	}

	return result
}
