package clock

import "fmt"

const testVersion = 4

// Clock means a real clock
type Clock struct {
	hour   int
	minute int
}

const (
	perMinutes = 60
	perHour    = 24
)

func normalize(hour, minute int) (int, int) {
	c := 0
	m := minute % perMinutes
	if m < 0 {
		m += perMinutes
		c--
	}

	h := (c + minute/perMinutes + hour) % perHour
	if h < 0 {
		h += perHour
	}

	return h, m
}

// New create a new Clock
func New(hour, minute int) Clock {
	h, m := normalize(hour, minute)

	return Clock{hour: h, minute: m}
}

func (c Clock) String() string {
	return fmt.Sprintf("%02d:%02d", c.hour, c.minute)
}

// Add minutes into this Clock and made a new Clock
func (c Clock) Add(minutes int) Clock {
	h, m := normalize(c.hour, c.minute+minutes)

	return Clock{hour: h, minute: m}
}
