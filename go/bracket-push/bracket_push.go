package brackets

const testVersion = 5

func checkBracket(brackets []rune, b rune) bool {
	l := len(brackets)
	if l != 0 && brackets[l-1] == b {
		return true
	}

	return false
}

// Bracket check if the brackets in text pair
func Bracket(text string) (bool, error) {
	var bracketStack []rune
	for _, v := range []rune(text) {
		switch v {
		case '[', '{', '(':
			bracketStack = append(bracketStack, v)
		case ']':
			if !checkBracket(bracketStack, '[') {
				return false, nil
			}

			bracketStack = bracketStack[:len(bracketStack)-1]
		case '}':
			if !checkBracket(bracketStack, '{') {
				return false, nil
			}

			bracketStack = bracketStack[:len(bracketStack)-1]
		case ')':
			if !checkBracket(bracketStack, '(') {
				return false, nil
			}

			bracketStack = bracketStack[:len(bracketStack)-1]
		}
	}

	return len(bracketStack) == 0, nil
}
