package tournament

import (
	"bufio"
	"fmt"
	"io"
	"sort"
	"strings"
)

type score struct {
	win  int32
	draw int32
	lose int32
}

type record struct {
	name string
	stat *score
}

// Tally analyzes the input and makes a better scoreboard
func Tally(reader io.Reader, writer io.Writer) error {
	scoreboard := make(map[string]*score)

	scanner := bufio.NewScanner(reader)

	for scanner.Scan() {
		text := scanner.Text()
		if text == "" {
			continue
		}

		if strings.HasPrefix(text, "#") {
			continue
		}

		v := strings.Split(text, ";")
		if len(v) != 3 {
			return fmt.Errorf("unexpected format %s", text)
		}

		const first = 0
		const second = 1
		const action = 2

		switch v[action] {
		case "win":
			w, ok := scoreboard[v[first]]
			if !ok {
				scoreboard[v[first]] = &score{1, 0, 0}
			} else {
				w.win++
			}

			l, ok := scoreboard[v[second]]
			if !ok {
				scoreboard[v[second]] = &score{0, 0, 1}
			} else {
				l.lose++
			}

		case "draw":
			w, ok := scoreboard[v[first]]
			if !ok {
				scoreboard[v[first]] = &score{0, 1, 0}
			} else {
				w.draw++
			}

			l, ok := scoreboard[v[second]]
			if !ok {
				scoreboard[v[second]] = &score{0, 1, 0}
			} else {
				l.draw++
			}
		case "loss":
			w, ok := scoreboard[v[second]]
			if !ok {
				scoreboard[v[second]] = &score{1, 0, 0}
			} else {
				w.win++
			}

			l, ok := scoreboard[v[first]]
			if !ok {
				scoreboard[v[first]] = &score{0, 0, 1}
			} else {
				l.lose++
			}

		default:
			return fmt.Errorf("unexpected result %s", v[action])
		}
	}

	result := make([]*record, 0, len(scoreboard))

	for k, v := range scoreboard {
		result = append(result, &record{k, v})
	}

	sort.Slice(result, func(i, j int) bool {
		iScore := 3*result[i].stat.win + result[i].stat.draw
		jScore := 3*result[j].stat.win + result[j].stat.draw
		if iScore < jScore {
			return false
		} else if iScore == jScore {
			return result[i].name < result[j].name
		} else {
			return true
		}
	})

	fmt.Fprint(writer, "Team                           | MP |  W |  D |  L |  P\n")

	for _, v := range result {
		fmt.Fprintf(writer, "%-30s | %2d | %2d | %2d | %2d | %2d\n",
			v.name, v.stat.win+v.stat.draw+v.stat.lose, v.stat.win, v.stat.draw, v.stat.lose, 3*v.stat.win+v.stat.draw)
	}

	return scanner.Err()
}
