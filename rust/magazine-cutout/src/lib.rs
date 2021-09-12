use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut map = HashMap::new();

    for elem in magazine {
        map.entry(elem).and_modify(|v| *v += 1).or_insert(1);
    }

    for elem in note {
        if let Some(v) = map.get_mut(elem) {
            *v -= 1;

            if *v < 0 {
                return false;
            }
        } else {
            return false;
        }
    }

    true
}
