pub fn is_valid(id: &str) -> bool {
    // check the length
    if id.len() < 2 {
        return false;
    }

    let mut index: u32 = 0;
    let mut number: Vec<u32> = Vec::new();

    for c in id.chars().rev() {
        if c.is_whitespace() {
            continue;
        }

        let mut v: u32;

        match c.to_digit(10) {
            Some(d) => v = d,
            None => return false,
        }

        if index % 2 != 0 {
            v *= 2;

            if v > 9 {
                v -= 9;
            }

        }

        index += 1;

        number.push(v);
    }

    if number.len() == 1 {
        return false;
    }

    let r: u32 = number.iter().sum();

    (r % 10) == 0
}
