package matrix

import (
	"bufio"
	"errors"
	"fmt"
	"strconv"
	"strings"
)

// Matrx hold a matrix
type Matrx struct {
	row int
	col int
	p   []int
}

// Rows return a matrix
func (m *Matrx) Rows() [][]int {
	p := make([][]int, 0, m.row)

	for i := 0; i < m.row; i++ {
		pos := i * m.col
		v := make([]int, m.col)
		copy(v, m.p[pos:pos+m.col])
		p = append(p, v)
	}

	return p
}

// Cols return a martix
func (m *Matrx) Cols() [][]int {
	p := make([][]int, 0, m.col)

	for i := 0; i < m.col; i++ {
		v := make([]int, 0, m.row)
		for j := 0; j < m.row; j++ {
			pos := j*m.col + i

			v = append(v, m.p[pos])
		}

		p = append(p, v)
	}

	return p
}

// Set set a value
func (m *Matrx) Set(r, c, v int) bool {
	if r < 0 || c < 0 || r >= m.row || c >= m.col {
		return false
	}

	pos := r*m.col + c

	m.p[pos] = v

	return true
}

// New returns a pointer for a matrix and also a error code
func New(input string) (*Matrx, error) {
	if strings.HasSuffix(input, "\n") {
		return nil, errors.New("end up with empty line")
	}

	reader := strings.NewReader(input)
	scanner := bufio.NewScanner(reader)

	m := &Matrx{0, 0, nil}

	for scanner.Scan() {
		line := strings.TrimPrefix(scanner.Text(), " ")

		vec := strings.Split(line, " ")

		for _, v := range vec {
			i, err := strconv.Atoi(v)
			if err != nil {
				return nil, err
			}

			m.p = append(m.p, i)
		}

		if m.col == 0 {
			m.col = len(vec)
		} else if m.col != len(vec) {
			return nil, fmt.Errorf("invalid line %s", scanner.Text())
		}

		m.row++
	}

	return m, nil
}
