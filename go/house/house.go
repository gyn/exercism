package house

const testVersion = 1

var last = "This is "
var lastAlt = "that lay in "
var lastPost = "the house that Jack built."

var firstPre = "This is the "
var animals = []string{
	"malt",
	"rat",
	"cat",
	"dog",
	"cow with the crumpled horn",
	"maiden all forlorn",
	"man all tattered and torn",
	"priest all shaven and shorn",
	"rooster that crowed in the morn",
	"farmer sowing his corn",
	"horse and the hound and the horn",
}
var verbs = []string{
	"ate",
	"killed",
	"worried",
	"tossed",
	"milked",
	"kissed",
	"married",
	"woke",
	"kept",
	"belonged to",
	"",
}

// Verse returns the nth paragraph of nursery rhyme
func Verse(index int) string {
	if index == 1 {
		return last + lastPost
	}

	s := firstPre + animals[index-2] + "\n"

	for i := index - 1; i > 1; i-- {
		s += "that " + verbs[i-2] + " the " + animals[i-2] + "\n"
	}

	return s + lastAlt + lastPost
}

// Song returns the nursery rhyme
func Song() string {
	max := 12
	min := 1

	var s string
	for i := min; i <= max; i++ {
		s += Verse(i)
		if i != max {
			s += "\n\n"
		}
	}

	return s
}
