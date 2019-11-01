use std::mem;

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
        let mut top_scores = Vec::with_capacity(MAX_TOP_SCORE_ITEMS);

        for i in self.record.iter() {
            let mut hold = *i;

            for i in 0..MAX_TOP_SCORE_ITEMS {
                if let Some(v) = top_scores.get_mut(i) {
                    if hold > *v {
                        mem::swap(&mut hold, v);
                    }
                } else {
                    top_scores.push(hold);
                    break;
                }
            }
        }

        top_scores
    }
}
