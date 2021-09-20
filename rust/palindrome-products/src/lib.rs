use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
pub struct Palindrome {
    v: Vec<(u64, u64)>,
}

impl Palindrome {
    pub fn new(a: u64, b: u64) -> Palindrome {
        Palindrome { v: vec![(a, b)] }
    }

    //pub fn value(&self) -> u64 {
    //    unimplemented!("return the value of this palindrome")
    //}

    pub fn insert(&mut self, a: u64, b: u64) {
        self.v.push((a, b))
    }
}

fn is_palindrome(value: u64) -> bool {
    let mut v = value;
    let mut r = 0;

    while v > 0 {
        r = r * 10 + v % 10;
        v /= 10;
    }

    r == value
}

pub fn palindrome_products(min: u64, max: u64) -> Option<(Palindrome, Palindrome)> {
    if min > max {
        return None;
    }

    let mut min_value = u64::MAX;
    let mut max_value = u64::MIN;

    let mut min_set = Palindrome::new(0, 0);
    let mut max_set = Palindrome::new(0, 0);

    for i in min..=max {
        for j in i..=max {
            let v = i * j;
            if !is_palindrome(v) {
                continue;
            }

            match v.cmp(&min_value) {
                Ordering::Less => {
                    min_value = v;
                    min_set = Palindrome::new(i, j);
                }
                Ordering::Equal => {
                    min_set.insert(i, j);
                }
                _ => {}
            }

            match v.cmp(&max_value) {
                Ordering::Greater => {
                    max_value = v;
                    max_set = Palindrome::new(i, j);
                }
                Ordering::Equal => {
                    max_set.insert(i, j);
                }
                _ => {}
            }
        }
    }

    if min_value == u64::MAX {
        None
    } else {
        Some((min_set, max_set))
    }
}
