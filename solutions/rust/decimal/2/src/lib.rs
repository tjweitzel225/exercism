use std::{
    cmp::Ordering,
    ops::{Add, Mul, Sub},
};

use num_bigint::{BigInt, BigUint};
use num_integer::Integer;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decimal {
    numerator: BigInt,
    denominator: BigUint,
}

impl Decimal {
    fn new(numerator: BigInt, denominator: BigUint) -> Self {
        let g = numerator.magnitude().gcd(&denominator);
        Self {
            numerator: numerator / BigInt::from(g.clone()),
            denominator: denominator / g,
        }
    }

    pub fn from_ints(numer: i64, denom: u64) -> Self {
        Self::new(BigInt::from(numer), BigUint::from(denom))
    }

    pub fn try_from(input: &str) -> Option<Self> {
        let (int, frac) = input.split_once('.').unwrap_or((input, ""));
        if !frac.bytes().all(|b| b.is_ascii_digit()) {
            return None;
        }
        let numerator: BigInt = format!("{int}{frac}").parse().ok()?;
        let denominator = BigUint::from(10u32).pow(frac.len() as u32);
        dbg!(&numerator, &denominator);
        Some(Self::new(numerator, denominator))
    }
}

impl Add for Decimal {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * BigInt::from(rhs.denominator.clone())
                + rhs.numerator * BigInt::from(self.denominator.clone()),
            self.denominator * rhs.denominator,
        )
    }
}

impl Sub for Decimal {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * BigInt::from(rhs.denominator.clone())
                - rhs.numerator * BigInt::from(self.denominator.clone()),
            self.denominator * rhs.denominator,
        )
    }
}

impl Mul for Decimal {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        Self::new(
            self.numerator * rhs.numerator,
            self.denominator * rhs.denominator,
        )
    }
}

impl Ord for Decimal {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = &self.numerator * BigInt::from(other.denominator.clone());
        let rhs = &other.numerator * BigInt::from(self.denominator.clone());
        lhs.cmp(&rhs)
    }
}

impl PartialOrd for Decimal {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
