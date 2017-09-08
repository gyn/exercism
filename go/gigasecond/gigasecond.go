package gigasecond

import "time"

const testVersion = 4

// AddGigasecond adds 1000000000 seconds to t and made a new time.Time
func AddGigasecond(t time.Time) time.Time {
	return t.Add(1000000000 * time.Second)
}
