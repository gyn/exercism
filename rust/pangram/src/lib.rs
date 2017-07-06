#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn is_pangram(words: &str) -> bool {
    let score = &mut [0u8; 26];

    for v in words.chars() {
        if !v.is_ascii_alphabetic() {
            continue;
        }

        score[v.to_ascii_lowercase() as usize - 'a' as usize] = 1;
    }

    score.iter().all(|&x| x == 1u8)
}
