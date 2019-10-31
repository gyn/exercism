pub fn is_armstrong_number(num: u32) -> bool {
    let mut result = num;
    let mut numbers = Vec::new();

    while result > 0 {
        numbers.push(result % 10);
        result /= 10;
    }

    let width = numbers.len() as u32;

    num == numbers.iter().fold(0, |acc, x| acc + x.pow(width))
}
