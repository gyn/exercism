pub fn hamming_distance(a: &str, b: &str) -> Result<usize, &'static str> {
    if a.len() != b.len() {
        return Result::Err("Length is not same");
    }

    let count = a.chars()
        .zip(b.chars())
        .filter(|&(an, bn)| an != bn)
        .count();

    Result::Ok(count)
}
