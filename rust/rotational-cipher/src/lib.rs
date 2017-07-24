#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn rotate(text: &str, key: usize) -> String {
    text.chars()
        .map(|x| match x {
            _ if x.is_ascii_lowercase() => {
                let v = (x as u8 - b'a' + key as u8) % 26 + b'a';
                v as char
            }
            _ if x.is_ascii_uppercase() => {
                let v = (x as u8 - b'A' + key as u8) % 26 + b'A';
                v as char
            }
            _ => x,
        })
        .collect()
}
