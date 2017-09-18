package protein

const testVersion = 1

// FromCodon translates a condon to it name
func FromCodon(codon string) string {
	switch codon {
	case "AUG":
		return "Methionine"
	case "UUU", "UUC":
		return "Phenylalanine"
	case "UUA", "UUG":
		return "Leucine"
	case "UCU", "UCC", "UCA", "UCG":
		return "Serine"
	case "UAU", "UAC":
		return "Tyrosine"
	case "UGU", "UGC":
		return "Cysteine"
	case "UGG":
		return "Tryptophan"
	case "UAA", "UAG", "UGA":
		return "STOP"
	default:
		return ""
	}
}

func windowN(s string, n int) []string {
	sub := ""
	subs := []string{}

	runes := []rune(s)
	for i, r := range runes {
		sub = sub + string(r)
		if (i+1)%n == 0 {
			subs = append(subs, sub)
			sub = ""
		}
	}

	if len(sub) != 0 {
		subs = append(subs, sub)
	}

	return subs
}

// FromRNA translates a RNA serial to codon serial
func FromRNA(rna string) []string {
	result := []string{}
	codons := windowN(rna, 3)

	for _, v := range codons {
		r := FromCodon(v)
		if r == "STOP" {
			break
		}

		result = append(result, r)
	}

	return result
}
