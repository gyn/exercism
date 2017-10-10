package say

import "strings"

const testVersion = 1

const (
	ten     = 10
	hundred = 100
	thound  = 1000
)

var ntable = map[uint64]string{
	0:  "zero",
	1:  "one",
	2:  "two",
	3:  "three",
	4:  "four",
	5:  "five",
	6:  "six",
	7:  "seven",
	8:  "eight",
	9:  "nine",
	10: "ten",
	11: "eleven",
	12: "twelve",
	13: "thirteen",
	14: "fourteen",
	15: "fifteen",
	16: "sixteen",
	17: "seventeen",
	18: "eighteen",
	19: "nineteen",
	20: "twenty",
	30: "thirty",
	40: "forty",
	50: "fifty",
	60: "sixty",
	70: "seventy",
	80: "eighty",
	90: "ninety",
}

var units = []string{
	"",
	"thousand",
	"million",
	"billion",
	"trillion",
	"quadrillion",
	"quintillion",
}

func translate(number uint64) string {
	var result string
	t := number % hundred
	if h := number / hundred; h != 0 {
		result = ntable[h] + " hundred"

		if t == 0 {
			return result
		}

		result += " "
	}

	d := number % ten
	if t <= 20 || d == 0 {
		result += ntable[t]
	} else {
		result += ntable[t-d] + "-" + ntable[d]
	}

	return result
}

// Say translate a uint64 number into English
func Say(number uint64) string {
	if number < thound {
		return translate(number)
	}

	var s []uint64
	n := number
	for n > 0 {
		r := n % thound

		s = append(s, r)

		n /= thound
	}

	var result string
	for i := len(s) - 1; i >= 0; i-- {
		s := s[i]
		if s == 0 {
			continue
		}

		v := translate(s)
		u := units[i]
		if len(u) == 0 {
			result += v
		} else {
			result += v + " " + u + " "
		}
	}

	return strings.TrimSpace(result)
}
