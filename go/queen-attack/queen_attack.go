package queenattack

import (
	"errors"
	"unicode/utf8"
)

const testVersion = 2

func parsePostion(pos string) (int, int, error) {
	if utf8.RuneCountInString(pos) != 2 {
		return 0, 0, errors.New("invalid format")
	}

	x := pos[0]
	if x > 'h' || x < 'a' {
		return 0, 0, errors.New("invalid x")
	}

	y := pos[1]
	if y > '8' || y < '1' {
		return 0, 0, errors.New("invalid y")
	}

	return int(x - 'a'), int(y - '0'), nil
}

func absolute(v int) int {
	if v < 0 {
		return -v
	}

	return v
}

// CanQueenAttack check if white and black queue could attack
func CanQueenAttack(white, black string) (bool, error) {
	wx, wy, err := parsePostion(white)
	if err != nil {
		return false, err
	}

	bx, by, err := parsePostion(black)
	if err != nil {
		return false, err
	}

	xdelta := wx - bx
	ydelta := wy - by

	if xdelta == 0 && ydelta == 0 {
		return false, errors.New("at same position")
	}

	return xdelta == 0 || ydelta == 0 || absolute(xdelta) == absolute(ydelta), nil
}
