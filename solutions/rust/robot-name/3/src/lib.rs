use std::{cell::RefCell, collections::HashSet, rc::Rc};

use rand::Rng;

/// A `RobotFactory` is responsible for ensuring that all robots produced by
/// it have a unique name. Robots from different factories can have the same
/// name.
pub struct RobotFactory {
    used_names: Rc<RefCell<HashSet<String>>>,
}

pub struct Robot {
    name: String,
    used_names: Rc<RefCell<HashSet<String>>>,
}

const ALPHABET: &[u8] = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".as_bytes();
fn gen_name(rng: &mut impl Rng) -> String {
    let n = rng.next_u32() as usize;
    let char_1 = ALPHABET[n % 26] as char;
    let char_2 = ALPHABET[(n << 8) % 26] as char;
    let num = (n << 16) % 1000;
    dbg!(format!("{}{}{:03}", char_1, char_2, num))
}

impl RobotFactory {
    pub fn new() -> Self {
        RobotFactory {
            used_names: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot(&mut self, _rng: &mut impl Rng) -> Robot {
        let mut name = gen_name(_rng);
        while !self.used_names.borrow_mut().insert(name.clone()) {
            name = gen_name(_rng);
        }
        Robot {
            name,
            used_names: self.used_names.clone(),
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset(&mut self, rng: &mut impl Rng) {
        self.used_names.borrow_mut().remove(&self.name);
        let mut name = gen_name(rng);
        while !self.used_names.borrow_mut().insert(name.clone()) {
            name = gen_name(rng);
        }
        self.name = name;
    }
}
