fn binary_serch(list: &[u32], key: u32, base: usize) -> Option<usize> {
    let upper = list.len();

    match upper {
        0 => None,
        1 => if list[0] == key {
            Some(base)
        } else {
            None
        },
        _ => {
            let needle = upper / 2;
            let middle = list[needle];

            if key > middle {
                binary_serch(&list[needle..], key, base + needle)
            } else if key < middle {
                binary_serch(&list[..needle], key, base)
            } else {
                Some(base + needle)
            }
        }
    }
}

pub fn find(list: &[u32], key: u32) -> Option<usize> {
    binary_serch(list, key, 0)
}
