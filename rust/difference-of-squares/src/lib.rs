pub fn square_of_sum(x: u32) -> u64 {
    let v = u64::from(x);

    let r = (v + 1) * v / 2;

    (r * r)
}

pub fn sum_of_squares(x: u32) -> u64 {
    let v = u64::from(x);

    v * (v + 1) * (2 * v + 1) / 6
}

pub fn difference(x: u32) -> u64 {
    let v = u64::from(x);

    v * (6 * v + 4) * (v + 1) * (v - 1) / 24
}
