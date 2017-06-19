pub fn sum_of_multiples(limit: usize, vec: &[usize]) -> usize {
    let mut sum: usize = 0;

    for i in 1..limit {
        let rc = vec.iter().find(|&&x| i % x == 0);

        if rc.is_some() {
            sum += i;
        }
    }

    sum
}
