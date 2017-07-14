use std::collections::HashMap;

pub fn nucleotide_counts(serial: &str) -> Result<HashMap<char, usize>, &str> {
    let counter = &mut [0usize; 4];

    for s in serial.chars() {
        match s {
            'A' => counter[0] += 1,
            'C' => counter[1] += 1,
            'G' => counter[2] += 1,
            'T' => counter[3] += 1,
            _ => return Err("Invalid serial"),
        }
    }

    Ok(
        [
            ('A', counter[0]),
            ('C', counter[1]),
            ('G', counter[2]),
            ('T', counter[3]),
        ].iter()
            .cloned()
            .collect(),
    )
}

pub fn count(n: char, serial: &str) -> Result<usize, &str> {
    if n != 'A' && n != 'C' && n != 'G' && n != 'T' {
        return Err("Invalid nucleotide");
    }

    let mut counter = 0;

    for s in serial.chars() {
        if s == n {
            counter += 1;

            continue;
        }

        if s != 'A' && s != 'C' && s != 'G' && s != 'T' {
            return Err("Invalid serial");
        }
    }

    Ok(counter)
}
