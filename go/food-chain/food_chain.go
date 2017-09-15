package foodchain

const testVersion = 3

var animals = []string{
	"", "fly", "spider", "bird", "cat", "dog", "goat", "cow", "horse",
}
var first = "I know an old lady who swallowed a "
var second = []string{
	"",
	"",
	"It wriggled and jiggled and tickled inside her.\n",
	"How absurd to swallow a bird!\n",
	"Imagine that, to swallow a cat!\n",
	"What a hog, to swallow a dog!\n",
	"Just opened her throat and swallowed a goat!\n",
	"I don't know how she swallowed a cow!\n",
	"She's dead, of course!",
}
var third = " that wriggled and jiggled and tickled inside her"
var last = "I don't know why she swallowed the fly. Perhaps she'll die."

// Verse returns the index-th sentence of the song
func Verse(index int) string {
	result := first + animals[index] + ".\n" + second[index]

	if index < 8 {
		for i := index; i > 1; i-- {
			result += "She swallowed the " + animals[i] + " to catch the " + animals[i-1]
			if i == 3 {
				result += third
			}
			result += ".\n"
		}

		result += last
	}

	return result
}

// Verses returns lower-th to upper-th sentence of the song
func Verses(lower, upper int) string {
	var result string

	for i := lower; i < upper; i++ {
		result += Verse(i) + "\n\n"
	}

	result += Verse(upper)

	return result
}

// Song returns the whole song
func Song() string {
	return Verses(1, 8)
}
