#![feature(ascii_ctype)]

pub fn check(words: &str) -> bool {
    let mut mask = 0u32;

    for c in words.chars() {
        if c.is_ascii_alphabetic() {
            let index = c.to_ascii_lowercase() as usize - 'a' as usize;

            let v = 1u32 << index;
            if mask & v == v {
                return false;
            }

            mask |= v;
        }
    }

    true
}
