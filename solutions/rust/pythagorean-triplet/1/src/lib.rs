use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut set = HashSet::new();
    for a in 1..sum.div_ceil(3) {
        let remaining = sum - a;
        for b in (a + 1)..remaining.div_ceil(2) {
            let c = remaining - b;
            if a * a + b * b == c * c {
                set.insert([a, b, c]);
            }
        }
    }
    set
}
