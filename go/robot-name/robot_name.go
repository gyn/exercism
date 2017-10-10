package robotname

import (
	"fmt"
	"math/rand"
	"time"
)

const testVersion = 1

// Robot stands a new robot
type Robot struct {
	name string
}

var r *rand.Rand

func init() {
	seed := time.Now().UnixNano()
	r = rand.New(rand.NewSource(seed))
}

func newName() string {
	v := fmt.Sprintf("%c%c%03d",
		'A'+r.Int31n(26),
		'A'+r.Int31n(26),
		r.Int31n(1000))

	return v
}

// Name returns the robot's name
func (r *Robot) Name() string {
	if len(r.name) == 0 {
		r.name = newName()
	}

	return r.name
}

// Reset return a new name for the robot
func (r *Robot) Reset() {
	r.name = newName()
}
