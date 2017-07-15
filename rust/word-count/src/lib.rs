#![feature(ascii_ctype)]
use std::ascii::AsciiExt;
use std::collections::HashMap;

pub fn word_count(text: &str) -> HashMap<String, u32> {
    let iter = text.split(|x: char| x.is_ascii_punctuation() || x == ' ')
        .filter(|x| !x.is_empty())
        .map(|x| x.to_lowercase());

    let mut result: HashMap<String, u32> = HashMap::new();

    for i in iter {
        let v = result.get(&i).map_or(1, |v| v + 1);

        result.insert(i, v);
    }

    result
}
