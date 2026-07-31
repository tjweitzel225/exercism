pub struct Matrix {
    data: Option<Vec<Vec<u32>>>,
}

impl Matrix {
    pub fn new(input: &str) -> Self {
        Matrix {
            data: input
                .lines()
                .map(|l| l.split(' ').map(|i| i.parse().ok()).collect())
                .collect(),
        }
    }

    pub fn row(&self, row_no: usize) -> Option<Vec<u32>> {
        self.data
            .as_ref()
            .and_then(|rows| rows.get(row_no - 1).cloned())
    }

    pub fn column(&self, col_no: usize) -> Option<Vec<u32>> {
        self.data.as_ref().and_then(|rows| {
            rows.iter()
                .map(|row| row.get(col_no - 1).copied())
                .collect()
        })
    }
}
