#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

const LA: u8 = b'a';
const LZ: u8 = b'z';

pub fn encode(message: &str) -> String {
    let mut index = 0;
    message
        .chars()
        .filter_map(|c| match c {
            _ if c.is_ascii_digit() => Some(c),
            _ if c.is_ascii_uppercase() => {
                let v = LZ - c.to_ascii_lowercase() as u8 + LA;
                Some(v as char)
            }
            _ if c.is_ascii_lowercase() => {
                let v = LZ - c as u8 + LA;
                Some(v as char)
            }
            _ => None,
        })
        .fold(String::new(), |mut acc, x| {
            index += 1;
            if index == 6 {
                acc.push(' ');
                index = 1;
            }

            acc.push(x);
            acc
        })
}

pub fn decode(message: &str) -> String {
    message
        .chars()
        .filter_map(|c| match c {
            _ if c.is_ascii_digit() => Some(c),
            _ if c.is_ascii_lowercase() => {
                let v = LZ - c as u8 + LA;
                Some(v as char)
            }
            _ => None,
        })
        .collect()
}
