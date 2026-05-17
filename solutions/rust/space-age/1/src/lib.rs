// The code below is a stub. Just enough to satisfy the compiler.
// In order to pass the tests you can add-to or change any of this code.

use std::{collections::HashMap, sync::LazyLock};
const SECONDS_IN_EARTH_YEAR: f64 = 31557600.0;
#[derive(Debug)]
pub struct Duration {
    seconds: u64,
}

impl From<u64> for Duration {
    fn from(s: u64) -> Self {
        Duration { seconds: s }
    }
}

static PERIODS: LazyLock<HashMap<String, f64>> =
    LazyLock::new(|| read_period_map().expect("Failed to load periods.csv."));

pub trait Planet {
    fn name() -> &'static str;
    fn years_during(d: &Duration) -> f64 {
        let period = PERIODS.get(Self::name()).expect("No planet found!");
        d.seconds as f64 / (SECONDS_IN_EARTH_YEAR * period)
    }
}

pub fn read_period_map() -> Result<HashMap<String, f64>, Box<dyn std::error::Error>> {
    csv::Reader::from_path("/Users/tyler/Exercism/rust/space-age/periods.csv")?
        .records()
        .map(|r| {
            let record = r?;
            let key = record[0].to_string();
            let val: f64 = record[1].parse()?;
            Ok((key, val))
        })
        .collect()
}

pub struct Mercury;
pub struct Venus;
pub struct Earth;
pub struct Mars;
pub struct Jupiter;
pub struct Saturn;
pub struct Uranus;
pub struct Neptune;

macro_rules! impl_planet {
    ($($t:ty),* $(,)?) => {
        $(
            impl Planet for $t {
                fn name() -> &'static str {
                    stringify!($t)

                }
            }
        )*
    };
}
impl_planet!(
    Mercury, Venus, Earth, Mars, Jupiter, Saturn, Uranus, Neptune,
);
