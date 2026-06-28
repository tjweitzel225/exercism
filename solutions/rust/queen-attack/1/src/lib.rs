#[derive(Debug)]
pub struct ChessPosition(i32, i32);

#[derive(Debug)]
pub struct Queen {
    position: ChessPosition,
}

impl ChessPosition {
    pub fn new(rank: i32, file: i32) -> Option<Self> {
        ((0..8).contains(&rank) && (0..8).contains(&file)).then_some(ChessPosition(rank, file))
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen { position }
    }

    pub fn can_attack(&self, other: &Queen) -> bool {
        let dx = other.position.0 - self.position.0;
        let dy = other.position.1 - self.position.1;
        dx == 0 || dy == 0 || dx.abs() == dy.abs()
    }
}
