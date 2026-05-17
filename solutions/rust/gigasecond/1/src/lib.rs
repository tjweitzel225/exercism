use time::PrimitiveDateTime as DateTime;
use time::ext::NumericalDuration;

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let gigasec = 10_i64.pow(9).seconds();
    start.checked_add(gigasec).expect("Overflow!")
}
