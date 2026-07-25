fn zigzag_idxs(len: usize, rails: usize) -> impl Iterator<Item = usize> {
    let period = 2 * (rails - 1);
    let row = |i: &usize| {
        let p = i % period;
        p.min(period - p)
    };
    let mut order: Vec<usize> = (0..len).collect();
    order.sort_by_key(row);
    order.into_iter()
}

pub struct RailFence {
    rails: u32,
}
impl RailFence {
    pub fn new(rails: u32) -> RailFence {
        RailFence { rails }
    }

    pub fn encode(&self, text: &str) -> String {
        let rails = self.rails as usize;
        if rails == 1 {
            return text.to_string();
        }
        zigzag_idxs(text.chars().count(), rails)
            .map(|i| text.chars().nth(i).expect("i is valid by construction"))
            .collect()
    }

    pub fn decode(&self, cipher: &str) -> String {
        let rails = self.rails as usize;
        if rails == 1 {
            return cipher.to_string();
        }
        zigzag_idxs(cipher.chars().count(), rails)
            .zip(cipher.chars())
            .fold(vec!['\0'; cipher.chars().count()], |mut acc, (i, c)| {
                acc[i] = c;
                acc
            })
            .into_iter()
            .collect()
    }
}
