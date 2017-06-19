pub fn hamming_distance<'a, 'b>(a: &'a str, b: &'a str) -> Result<usize, &'b str> {
    if a.len() != b.len() {
        return Err("Length is not same");
    }

    let max = a.len();

    let mut ai = a.chars();
    let mut bi = b.chars();

    let mut count = 0;

    for _ in 0..max {
        if ai.next() != bi.next() {
            count += 1;
        }
    }

    Ok(count)
}
