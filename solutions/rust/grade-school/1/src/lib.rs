use std::collections::HashMap;

pub struct School {
    roster: HashMap<String, u32>,
}

impl School {
    pub fn new() -> School {
        School {
            roster: HashMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        self.roster.entry(student.to_string()).or_insert(grade);
    }

    pub fn grades(&self) -> Vec<u32> {
        let mut g: Vec<_> = self.roster.values().copied().collect();
        g.sort_unstable();
        g.dedup();
        g
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        let mut v: Vec<_> = self
            .roster
            .iter()
            .filter_map(|(name, g)| (*g == grade).then_some(name))
            .cloned()
            .collect();
        v.sort_unstable();
        v
    }
}
