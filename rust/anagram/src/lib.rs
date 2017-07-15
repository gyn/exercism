use std::collections::HashMap;

fn anagrams(word: &str) -> HashMap<char, u32> {
    let mut result: HashMap<char, u32> = HashMap::new();

    for k in word.chars() {
        let v = result.get(&k).map_or(1, |v| v + 1);

        result.insert(k, v);
    }

    result
}

pub fn anagrams_for(word: &str, input: &[&'static str]) -> Vec<&'static str> {
    let lword = word.to_lowercase();

    let dict = anagrams(&lword);

    let mut result: Vec<&str> = Vec::new();

    for s in input {
        let ls = s.to_lowercase();

        if ls == lword {
            continue;
        }

        if anagrams(&ls) == dict {
            result.push(s);
        }
    }

    result
}
