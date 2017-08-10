pub fn factors(number: u64) -> Vec<u64> {
    let mut i = 2;
    let mut cur = number;
    let mut result = Vec::new();

    while cur > 1 {
        if cur % i == 0 {
            result.push(i);

            cur /= i;
        } else {
            i += 1;
        }
    }

    result
}
