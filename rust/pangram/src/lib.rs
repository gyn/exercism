#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn is_pangram(words: &str) -> bool {
    let r = words
        .chars()
        .filter(|x| x.is_ascii_alphabetic())
        .map(|x| x.to_ascii_lowercase() as u8 - b'a')
        .collect::<Vec<_>>();

    let score = &mut [0u8; 26];

    for v in r {
        score[v as usize] = 1;
    }

    score.iter().all(|&x| x == 1u8)
}
