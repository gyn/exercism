pub fn primes_up_to(limit: u32) -> Vec<u32> {
    if limit < 2 {
        return Vec::new();
    }

    let upper = (limit as f64).sqrt().ceil() as usize;

    let mut board = vec![true; limit as usize - 1];

    for i in 2..upper {
        if !board[i - 2] {
            continue;
        }

        let mut v = i * i;

        while v <= limit as usize {
            board[v - 2] = false;

            v += i;
        }
    }

    (2..limit + 1)
        .filter(|&x| board[x as usize - 2])
        .collect::<Vec<u32>>()
}
