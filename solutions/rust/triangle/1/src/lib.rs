use std::ops::{Add, Mul};

pub struct Triangle<T>(T, T, T);

impl<T> Triangle<T>
where
    T: Add<Output = T> + Mul + Copy + Default + PartialOrd + PartialEq,
{
    pub fn build(sides: [T; 3]) -> Option<Triangle<T>> {
        let [a, b, c] = sides;
        (a > T::default()
            && b > T::default()
            && c > T::default()
            && (a + b > c)
            && (b + c > a)
            && (c + a > b))
            .then_some(Triangle(a, b, c))
    }

    pub fn is_equilateral(&self) -> bool {
        (self.0 == self.1) && (self.1 == self.2)
    }

    pub fn is_scalene(&self) -> bool {
        self.0 != self.1 && self.1 != self.2 && self.2 != self.0
    }

    pub fn is_isosceles(&self) -> bool {
        self.0 == self.1 || self.1 == self.2 || self.2 == self.0
    }
}
