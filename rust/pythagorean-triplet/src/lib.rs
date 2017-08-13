pub fn find() -> Option<u32> {
    const VALUE: u32 = 1000;
    const LOWER: u32 = VALUE / 3;
    const UPPER: u32 = VALUE / 2;

    for c in LOWER..UPPER {
        for b in c / 2..c {
            for a in 1..b {
                if a * a + b * b != c * c {
                    continue;
                }

                if a + b + c == VALUE {
                    return Some(a * b * c);
                }
            }
        }
    }

    None
}
