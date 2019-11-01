const MAX_TOP_SCORE_ITEMS: usize = 3;

#[derive(Debug)]
pub struct HighScores<'a> {
    record: &'a [u32],
}

impl<'a> HighScores<'a> {
    pub fn new(scores: &'a [u32]) -> Self {
        HighScores { record: scores }
    }

    pub fn scores(&self) -> &[u32] {
        &self.record
    }

    pub fn latest(&self) -> Option<u32> {
        self.record.last().copied()
    }

    pub fn personal_best(&self) -> Option<u32> {
        self.record.iter().max().copied()
    }

    pub fn personal_top_three(&self) -> Vec<u32> {
        let mut result = self.record.to_vec();
        result.sort_by(|a, b| b.cmp(a));
        result.into_iter().take(MAX_TOP_SCORE_ITEMS).collect()
    }
}
