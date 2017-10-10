package romannumerals

import "errors"

const testVersion = 4

type table struct {
	base   int
	number [10]string
}

var romanDigits = [...]table{
	{1000, [...]string{"", "M", "MM", "MMM", "", "", "", "", "", ""}},
	{100, [...]string{"", "C", "CC", "CCC", "CD", "D", "DC", "DCC", "DCCC", "CM"}},
	{10, [...]string{"", "X", "XX", "XXX", "XL", "L", "LX", "LXX", "LXXX", "XC"}},
	{1, [...]string{"", "I", "II", "III", "IV", "V", "VI", "VII", "VIII", "IX"}},
}

// ToRomanNumeral returns the roman number for the number and a error
func ToRomanNumeral(number int) (string, error) {
	if number < 1 || number > 3000 {
		return "", errors.New("invalid number")
	}

	var result string
	for _, v := range romanDigits {
		i := number / v.base
		result += v.number[i]
		number -= i * v.base
	}

	return result, nil
}
