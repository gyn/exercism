#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn is_pangram(words: &str) -> bool {
    let mut score = 0u32;

    for v in words.chars() {
        if v.is_ascii_alphabetic() {
            let index = v.to_ascii_lowercase() as usize - 'a' as usize;
            score |= 1 << index;
        }
    }

    score == (1 << 26) - 1
}
