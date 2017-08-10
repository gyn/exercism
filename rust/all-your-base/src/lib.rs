pub fn convert(number: &[u32], from_base: u32, to_base: u32) -> Result<Vec<u32>, ()> {
    if from_base < 2 || to_base < 2 {
        return Err(());
    }

    if number.iter().any(|&x| x >= from_base) {
        return Err(());
    }

    let limit = number.iter().fold(0, |acc, &x| acc * from_base + x);
    let mut r = limit;

    let mut result = (0..)
        .take_while(|&x| limit >= to_base.pow(x as u32))
        .map(|_| {
            let d = r % to_base;
            r /= to_base;
            d
        })
        .collect::<Vec<_>>();

    result.reverse();

    Ok(result)
}
