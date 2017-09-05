pub fn nth(index: u32) -> Result<u32, String> {
    if index == 0 {
        return Err("Invalid index".to_string());
    }

    let mut number = 3;
    let mut primes = Vec::with_capacity(index as usize);
    primes.push(2);

    while primes.len() < index as usize {
        if primes.iter().all(|&x| number % x != 0) {
            primes.push(number);
        }

        number += 2;
    }

    Ok(*primes.last().unwrap())
}
