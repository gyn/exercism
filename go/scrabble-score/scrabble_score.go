package scrabble

const testVersion = 5

// Score calculates score of a string
func Score(input string) int {
	result := 0

	for _, v := range input {
		switch v {
		case 'A', 'E', 'I', 'O', 'U', 'L', 'N', 'R', 'S', 'T':
			fallthrough
		case 'a', 'e', 'i', 'o', 'u', 'l', 'n', 'r', 's', 't':
			result++
		case 'D', 'G':
			fallthrough
		case 'd', 'g':
			result += 2
		case 'B', 'C', 'M', 'P':
			fallthrough
		case 'b', 'c', 'm', 'p':
			result += 3
		case 'F', 'H', 'V', 'W', 'Y':
			fallthrough
		case 'f', 'h', 'v', 'w', 'y':
			result += 4
		case 'K':
			fallthrough
		case 'k':
			result += 5
		case 'J', 'X':
			fallthrough
		case 'j', 'x':
			result += 8
		case 'Q', 'Z':
			fallthrough
		case 'q', 'z':
			result += 10
		}
	}

	return result
}
