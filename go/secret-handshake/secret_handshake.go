package secret

const testVersion = 2

func revert(result []string) {
	for left, right := 0, len(result)-1; left < right; left, right = left+1, right-1 {
		result[left], result[right] = result[right], result[left]
	}
}

// Handshake translate a number to secret handshake events
func Handshake(number uint) []string {
	var result []string

	if number&0x01 == 0x01 {
		result = append(result, "wink")
	}

	if number&0x02 == 0x02 {
		result = append(result, "double blink")
	}

	if number&0x04 == 0x04 {
		result = append(result, "close your eyes")
	}

	if number&0x08 == 0x08 {
		result = append(result, "jump")
	}

	if number&0x10 == 0x10 {
		revert(result)
	}

	return result
}
