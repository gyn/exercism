pub struct RailFence {
    rails: usize,
}

impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence {
            rails: rails as usize,
        }
    }

    pub fn encode(&self, text: &str) -> String {
        let mut rails = vec![String::new(); self.rails];
        let indexes = (0..self.rails).chain((1..self.rails - 1).rev()).cycle();

        text.chars()
            .zip(indexes)
            .for_each(|(c, i)| rails[i as usize].push(c));

        rails.concat()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let array = cipher.chars().collect::<Vec<_>>();

        let mut indices: Vec<_> = (0..self.rails - 1)
            .chain((1..self.rails).rev())
            .cycle()
            .zip(0..cipher.len())
            .collect();

        indices.sort_unstable();

        let mut text: Vec<char> = vec![' '; array.len()];

        (0..array.len()).for_each(|i| {
            let index = indices[i].1;
            text[index] = array[i];
        });

        text.iter().collect::<String>()
    }
}
