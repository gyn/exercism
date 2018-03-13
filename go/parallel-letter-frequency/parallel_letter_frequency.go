package letter

import (
	"sync"
)

// ConFreqMap is a mult-thread safe map
type ConFreqMap struct {
	sync.Mutex
	v FreqMap
}

// ConcurrentFrequency counts words in specific string list
func ConcurrentFrequency(list []string) FreqMap {
	var m ConFreqMap

	m.v = make(map[rune]int)

	var wg sync.WaitGroup

	for _, v := range list {
		wg.Add(1)
		go func(s string) {
			defer wg.Done()

			for _, r := range s {
				m.Lock()
				m.v[r]++
				m.Unlock()
			}
		}(v)
	}

	wg.Wait()

	return m.v
}
