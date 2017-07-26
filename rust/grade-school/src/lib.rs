const MAX_GRADES: usize = 9;

#[derive(Default)]
pub struct School {
    db: Vec<Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            db: vec![Vec::new(); MAX_GRADES],
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let index = grade as usize;

        assert!(index != 0 && index <= MAX_GRADES);

        self.db[index - 1].push(student.to_string());

        self.db[index - 1].sort();
    }

    pub fn grades(&self) -> Vec<u32> {
        self.db
            .iter()
            .enumerate()
            .filter_map(|(i, v)| {
                if v.is_empty() {
                    return None;
                }

                Some(i as u32 + 1)
            })
            .collect::<Vec<u32>>()
    }

    pub fn grade(&self, grade: u32) -> Option<Vec<String>> {
        let index = grade as usize;

        assert!(index != 0 && index <= MAX_GRADES);

        if self.db[index - 1].is_empty() {
            return None;
        }

        Some(self.db[index - 1].clone())
    }
}
