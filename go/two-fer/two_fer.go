package twofer

// Share produces a string "One for (name|you), one for me."
func ShareWith(name string) string {
	object := name
	if len(name) == 0  {
		object = "you"
	}

	return "One for " + object + ", one for me."
}
