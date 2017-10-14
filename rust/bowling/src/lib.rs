type Frame = (u32, u32, u32);

const MAX_FRAME: usize = 10;
const FRAME_WINDOW: usize = 3;
const FRAME_SIZE: usize = MAX_FRAME + FRAME_WINDOW - 1;
const INVALID_SCORE: u32 = std::u32::MAX;

#[derive(Default)]
pub struct BowlingGame {
    record: [Frame; FRAME_SIZE],
    count: usize,
}

impl BowlingGame {
    pub fn new() -> BowlingGame {
        BowlingGame {
            record: [(INVALID_SCORE, INVALID_SCORE, INVALID_SCORE); FRAME_SIZE],
            count: 0,
        }
    }

    pub fn roll(&mut self, pins: u32) -> Result<u32, String> {
        if pins > 10 {
            return Err("Invalid pins".to_string());
        }

        match self.count {
            0...8 => roll_in_normal_frame(self, pins),
            9 => roll_in_final_frame(self, pins),
            _ => Err("No more pins".to_string()),
        }
    }

    pub fn score(&self) -> Result<u32, String> {
        if self.count != MAX_FRAME {
            return Err("Unfinished games".to_string());
        }

        let r = &self.record;
        let v = r.windows(FRAME_WINDOW).map(|v| count(v)).sum();

        Ok(v)
    }
}

fn roll_in_normal_frame(v: &mut BowlingGame, pins: u32) -> Result<u32, String> {
    let f = &mut v.record[v.count];

    match *f {
        (INVALID_SCORE, _, _) if pins == 10 => {
            f.0 = pins;
            v.count += 1;
            Ok(0)
        }
        (INVALID_SCORE, _, _) if pins < 10 => {
            f.0 = pins;
            Ok(0)
        }
        (a, INVALID_SCORE, _) if pins > 10 - a => Err("Invalid second pins".to_string()),
        (a, INVALID_SCORE, _) if pins <= 10 - a => {
            f.1 = pins;
            v.count += 1;
            Ok(1)
        }
        _ => panic!("Should not reach here"),
    }
}

fn roll_in_final_frame(v: &mut BowlingGame, pins: u32) -> Result<u32, String> {
    let f = &mut v.record[v.count];

    match *f {
        (INVALID_SCORE, _, _) => {
            f.0 = pins;
            Ok(0)
        }
        (10, INVALID_SCORE, _) => {
            f.1 = pins;
            Ok(1)
        }
        (a, INVALID_SCORE, _) if pins > 10 - a => Err("Invalid second pins".to_string()),
        (a, INVALID_SCORE, _) if pins < 10 - a => {
            f.1 = pins;
            v.count += 1;
            Ok(1)
        }
        (a, INVALID_SCORE, _) if pins == 10 - a => {
            f.1 = pins;
            Ok(1)
        }
        (10, 10, INVALID_SCORE) => {
            f.2 = pins;
            v.count += 1;
            Ok(2)
        }
        (10, b, INVALID_SCORE) if b != 10 && pins > 10 - b => Err("Invalid third pins".to_string()),
        (a, b, INVALID_SCORE) if a + b < 10 => Err("Invalid third pins".to_string()),
        (a, b, INVALID_SCORE) if a + b >= 10 => {
            f.2 = pins;
            v.count += 1;
            Ok(2)
        }
        _ => panic!("Should not reach here"),
    }
}

fn count(frames: &[Frame]) -> u32 {
    let f = &frames[1];
    if f.0 == INVALID_SCORE {
        return count_final_frame(frames);
    }

    count_normal_frame(frames)
}

fn count_normal_frame(frames: &[Frame]) -> u32 {
    let f = &frames[0];
    let s = &frames[1];
    let t = &frames[2];

    match *f {
        (10, _, _) if s.0 == 10 && t.0 != INVALID_SCORE => 10 + s.0 + t.0,
        (10, _, _) if s.0 == 10 && t.0 == INVALID_SCORE => 10 + s.0 + s.1,
        (10, _, _) if s.0 != 10 => 10 + s.0 + s.1,
        (a, b, _) if a + b == 10 => 10 + s.0,
        (a, b, _) if a + b < 10 => a + b,
        _ => panic!("Should not reach here"),
    }
}

fn count_final_frame(frames: &[Frame]) -> u32 {
    let f = &frames[0];

    match *f {
        (10, b, c) => 10 + b + c,
        (a, b, c) if a + b == 10 => a + b + c,
        (a, b, INVALID_SCORE) => a + b,
        _ => panic!("Should not reach here"),
    }
}
