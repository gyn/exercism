pub fn find() -> Option<u32> {
    const LOWER: u32 = 1000/3;
    const UPPER: u32 = 1000/2;

    for c in LOWER..UPPER {
        for b in c/2..c {
            for a in 1..b {
                if a*a + b*b != c*c {
                    continue;
                }

                if a + b + c == 1000 {
                    return Some(a*b*c);
                }
            }
        }
    }

    None
}
