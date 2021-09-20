use std::cmp::Ordering;

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();

    for (line_nr, line) in input.iter().enumerate() {
        let mut max = u64::MIN;

        let mut points = Vec::new();

        for (colume_nr, &i) in line.iter().enumerate() {
            match i.cmp(&max) {
                Ordering::Greater => {
                    max = i;
                    points.clear();
                    points.push((line_nr, colume_nr, max));
                }
                Ordering::Equal => {
                    points.push((line_nr, colume_nr, max));
                }
                _ => {}
            }
        }
        saddle_points.append(&mut points);
    }

    saddle_points
        .into_iter()
        .filter(|&(_, colume_nr, value)| {
            for line in input {
                if value > line[colume_nr] {
                    return false;
                }
            }

            true
        })
        .map(|(line, colume, _)| (line, colume))
        .collect()
}
