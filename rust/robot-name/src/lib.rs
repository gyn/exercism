extern crate rand;

use rand::distributions::{IndependentSample, Range};

#[derive(Debug, Default)]
pub struct Robot {
    tag: String,
}

impl Robot {
    pub fn new() -> Robot {
        let mut r = String::new();

        random_name(&mut r);

        Robot { tag: r }
    }

    pub fn name(&self) -> &str {
        &self.tag
    }

    pub fn reset_name(&mut self) {
        random_name(&mut self.tag);
    }
}

fn random_name(name: &mut String) {
    let char_range = Range::new(b'A', b'Z');
    let mut rng = rand::thread_rng();

    let c1 = char_range.ind_sample(&mut rng);
    let c2 = char_range.ind_sample(&mut rng);

    let digi_range = Range::new(1, 999);

    let d = digi_range.ind_sample(&mut rng);

    *name = format!("{}{}{:03}", char::from(c1), char::from(c2), d);
}
