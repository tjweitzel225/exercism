#[derive(PartialEq, Eq, Debug)]
pub enum Bucket {
    One,
    Two,
}

fn gcd(mut a: u8, mut b: u8) -> u8 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
/// A struct to hold your results in.
#[derive(PartialEq, Eq, Debug)]
pub struct BucketStats {
    /// The total number of "moves" it should take to reach the desired number of liters, including
    /// the first fill.
    pub moves: u8,
    /// Which bucket should end up with the desired number of liters? (Either "one" or "two")
    pub goal_bucket: Bucket,
    /// How many liters are left in the other bucket?
    pub other_bucket: u8,
}

/// Solve the bucket problem
pub fn solve(
    capacity_1: u8,
    capacity_2: u8,
    goal: u8,
    start_bucket: &Bucket,
) -> Option<BucketStats> {
    // Swap arguments when start_bucket is Bucket::Two, then uncross goal_bucket.
    // Then the implementation can assume start_bucket == &Bucket::One;
    if start_bucket == &Bucket::Two {
        return solve(capacity_2, capacity_1, goal, &Bucket::One).map(|stats| BucketStats {
            moves: stats.moves,
            goal_bucket: match stats.goal_bucket {
                Bucket::One => Bucket::Two,
                Bucket::Two => Bucket::One,
            },
            other_bucket: stats.other_bucket,
        });
    }
    let mut moves: u8 = 0;
    let mut state = (capacity_1, 0);
    loop {
        moves += 1;
        match state {
            // Invalid state per rules
            (0, b) if b == capacity_2 => break None,
            // Skip to solution if filling b gets you there
            (a, 0) if goal == capacity_2 => {
                break Some(BucketStats {
                    moves: moves + 1,
                    goal_bucket: Bucket::Two,
                    other_bucket: a,
                });
            }
            (a, b) if a == goal => {
                break Some(BucketStats {
                    moves,
                    goal_bucket: Bucket::One,
                    other_bucket: b,
                });
            }
            (a, b) if b == goal => {
                break Some(BucketStats {
                    moves,
                    goal_bucket: Bucket::Two,
                    other_bucket: a,
                });
            }
            // Can only fill left
            (0, b) => state = (capacity_1, b),
            // Can only dump right
            (a, b) if b == capacity_2 => state = (a, 0),
            // Pour left into right
            (a, b) => {
                state = if a + b <= capacity_2 {
                    // can transfer everything
                    (0, a + b)
                } else {
                    // some stays in bucket 1
                    (a + b - capacity_2, capacity_2)
                }
            }
        }
    }
}
