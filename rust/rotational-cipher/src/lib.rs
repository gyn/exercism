#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn rotate(text: &str, key: usize) -> String {
    const LA: u8 = b'a';
    const UA: u8 = b'A';
    text.chars()
        .map(|x| match x {
            _ if x.is_ascii_lowercase() => {
                let v = (x as u8 - LA + key as u8) % 26 + LA;
                v as char
            }
            _ if x.is_ascii_uppercase() => {
                let v = (x as u8 - UA + key as u8) % 26 + UA;
                v as char
            }
            _ => x,
        })
        .collect()
}
