// return Ok(x) where x is the number of steps required to reach 1
pub fn collatz(n: u64) -> Result<u64, &'static str> {
    if n < 1 {
        return Err("Invalid number");
    }

    let mut result = n;
    let mut steps = 0;

    while result != 1 {
        if result % 2 == 0 {
            result /= 2;
        } else {
            result = 3 * result + 1;
        }

        steps += 1;
    }

    Ok(steps)
}
