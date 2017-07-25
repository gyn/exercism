#![feature(ascii_ctype)]
use std::ascii::AsciiExt;

pub fn lsp(numbers: &str, length: usize) -> Result<usize, String> {
    if length == 0 {
        return Ok(1);
    }

    if numbers.len() < length {
        return Err("Invalid length".to_string());
    }

    let v = numbers
        .chars()
        .filter_map(|x| match x {
            _ if x.is_ascii_digit() => Some(x as usize - '0' as usize),
            _ => None,
        })
        .collect::<Vec<usize>>();

    if v.len() != numbers.len() {
        return Err("Non-digit found".to_string());
    }

    v.windows(length)
        .map(|x| x.iter().product())
        .max()
        .ok_or_else(|| "None".to_string())
}
