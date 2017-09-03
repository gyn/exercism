pub struct PascalsTriangle {
    row: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row: row_count }
    }

    fn pascal(row: u32) -> Vec<u32> {
        let mut result = Vec::with_capacity(row as usize);

        result.push(1);

        for k in 0..row {
            let v = result[k as usize] * (row - k) / (k + 1);

            result.push(v);
        }

        result
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row).map(PascalsTriangle::pascal).collect()
    }
}
