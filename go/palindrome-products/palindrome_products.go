package palindrome

import (
	"errors"
	"math"
)

const testVersion = 1

// Product store a product and a vector of integer pairs to get the product
type Product struct {
	Product        int
	Factorizations [][2]int
}

func isPalindromic(product int) bool {
	if product < 10 {
		return true
	}

	var digits []int
	for reminder := product; reminder != 0; reminder /= 10 {
		digits = append(digits, reminder%10)
	}

	for i, j := 0, len(digits)-1; i < len(digits)/2; i, j = i+1, j-1 {
		if digits[i] != digits[j] {
			return false
		}
	}

	return true
}

// Products returns two Product for min and max product and a error
func Products(fmin, fmax int) (Product, Product, error) {
	minProduct := Product{math.MaxInt32, nil}
	maxProduct := Product{math.MinInt32, nil}
	if fmin > fmax {
		return minProduct, minProduct, errors.New("fmin > fmax...")
	}

	for i := fmin; i <= fmax; i++ {
		for j := fmin; j <= fmax; j++ {
			product := i * j
			if !isPalindromic(product) {
				continue
			}

			if product < minProduct.Product {
				minProduct.Product = product
				minProduct.Factorizations = [][2]int{{i, j}}
			} else if product == minProduct.Product {
				if i <= j {
					minProduct.Factorizations = append(minProduct.Factorizations, [2]int{i, j})
				}
			}

			if product > maxProduct.Product {
				maxProduct.Product = product
				maxProduct.Factorizations = [][2]int{{i, j}}
			} else if product == maxProduct.Product {
				if i <= j {
					maxProduct.Factorizations = append(maxProduct.Factorizations, [2]int{i, j})
				}
			}
		}
	}

	if len(minProduct.Factorizations) == 0 || len(maxProduct.Factorizations) == 0 {
		return minProduct, minProduct, errors.New("no palindromes...")
	}

	return minProduct, maxProduct, nil
}
