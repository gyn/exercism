package accumulate

const testVersion = 1

// Accumulate maps a function f to string slice and get a new string slice
func Accumulate(text []string, f func(string) string) []string {
	var result []string

	for _, v := range text {
		result = append(result, f(v))
	}

	return result
}
