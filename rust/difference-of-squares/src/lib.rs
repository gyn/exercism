pub fn square_of_sum(x: u32) -> u64 {
    let v: u64 = x as u64;

    let r = (v + 1) * v / 2;

    (r * r)
}

pub fn sum_of_squares(x: u32) -> u64 {
    let v: u64 = x as u64;

    v * (v + 1) * (2 * v + 1) / 6
}

pub fn difference(x: u32) -> u64 {
    let v: u64 = x as u64;

    v * (6 * v + 4) * (v + 1) * (v - 1) / 24
}
