#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn is_pangram(words: &str) -> bool {
    let score = &mut [false; 26];

    for v in words.chars() {
        if v.is_ascii_alphabetic() {
            let index = v.to_ascii_lowercase() as usize - 'a' as usize;

            score[index] = true;
        }
    }

    score.iter().all(|&x| x)
}
