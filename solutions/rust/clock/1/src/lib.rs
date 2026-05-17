#[derive(PartialEq, Debug)]
pub struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
        let overflow_hours = minutes.div_euclid(60);
        let mod_minutes = minutes.rem_euclid(60);
        let mod_hours = (hours + overflow_hours).rem_euclid(24);
        Clock {
            hours: mod_hours,
            minutes: mod_minutes,
        }
    }
    pub fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl std::fmt::Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // Use write! to define the output format
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}
