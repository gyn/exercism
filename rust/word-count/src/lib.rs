#![feature(ascii_ctype)]
use std::ascii::AsciiExt;
use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    let mut result: HashMap<String, u32> = HashMap::new();

    let iter = text.split(|x: char| x.is_ascii_punctuation() || x == ' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_lowercase());

    for i in iter {
        let entry = result.entry(i).or_insert(0);
        *entry += 1;
    }

    result
}
