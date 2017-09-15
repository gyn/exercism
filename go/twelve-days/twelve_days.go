package twelve

const testVersion = 1

var numberTable = [12]string{
	"first",
	"second",
	"third",
	"fourth",
	"fifth",
	"sixth",
	"seventh",
	"eighth",
	"ninth",
	"tenth",
	"eleventh",
	"twelfth",
}

var textTable = [12]string{
	"a Partridge in a Pear Tree.",
	"two Turtle Doves, ",
	"three French Hens, ",
	"four Calling Birds, ",
	"five Gold Rings, ",
	"six Geese-a-Laying, ",
	"seven Swans-a-Swimming, ",
	"eight Maids-a-Milking, ",
	"nine Ladies Dancing, ",
	"ten Lords-a-Leaping, ",
	"eleven Pipers Piping, ",
	"twelve Drummers Drumming, ",
}

// Verse returns the sentence specified by number of the song
func Verse(number int) string {
	index := number - 1
	prefix := "On the " + numberTable[index] + " day of Christmas my true love gave to me, "

	var text string
	for i := index; i > 0; i-- {
		text += textTable[i]
	}

	if len(text) != 0 {
		text += "and "
	}

	text += textTable[0]

	return prefix + text
}

// Song returns the who song
func Song() string {
	var text string

	for i := 0; i < len(numberTable); i++ {
		text += Verse(i+1) + "\n"
	}

	return text
}
