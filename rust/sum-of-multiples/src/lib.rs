pub fn sum_of_multiples(limit: usize, vec: &[usize]) -> usize {
    (0..limit).filter(|x| vec.iter().any(|y| x % y == 0)).sum()
}
