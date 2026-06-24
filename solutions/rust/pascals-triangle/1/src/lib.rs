pub struct PascalsTriangle(Vec<Vec<u32>>);
fn fac(n: u32) -> u32 {
    (1..=n).product()
}
impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle(
            (0..row_count)
                .map(|n| (0..=n).map(|k| fac(n) / (fac(k) * fac(n - k))).collect())
                .collect(),
        )
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.0.clone()
    }
}
