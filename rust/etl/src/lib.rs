use std::collections::BTreeMap;
use std::ascii::AsciiExt;

pub fn transform(input: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut result: BTreeMap<char, i32> = BTreeMap::new();

    for (k, v) in input {
        for c in v {
            result.entry((*c).to_ascii_lowercase()).or_insert(*k);
        }
    }

    result
}
