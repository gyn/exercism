pub fn score(string: &str) -> usize {
    let mut score: usize = 0;

    for c in string.chars() {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' => score += 1,
            'a' | 'e' | 'i' | 'o' | 'u' | 'l' => score += 1,
            'N' | 'R' | 'S' | 'T' => score += 1,
            'n' | 'r' | 's' | 't' => score += 1,
            'D' | 'G' => score += 2,
            'd' | 'g' => score += 2,
            'B' | 'C' | 'M' | 'P' => score += 3,
            'b' | 'c' | 'm' | 'p' => score += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => score += 4,
            'f' | 'h' | 'v' | 'w' | 'y' => score += 4,
            'K' => score += 5,
            'k' => score += 5,
            'J' | 'X' => score += 8,
            'j' | 'x' => score += 8,
            'Q' | 'Z' => score += 10,
            'q' | 'z' => score += 10,
            _ => score += 0,
        }
    }

    score
}
