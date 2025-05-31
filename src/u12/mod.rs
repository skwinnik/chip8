#[derive(Debug)]
pub struct U12(usize);

use std::{fmt::Display, ops::Add};

const MIN: usize = 0x000;
const MAX: usize = 0xFFF;

impl U12 {
    pub fn from_usize(x: usize) -> Self {
        if x < MIN || x > MAX {
            panic!("usize out of u12 range")
        }

        U12(x)
    }

    pub fn checked_add(self, rhs: Self) -> Option<Self> {
        match self.0 + rhs.0 {
            result @ 0..=4095 => Some(U12(result)),
            _ => None,
        }
    }
}

impl Add for U12 {
    type Output = U12;

    fn add(self, rhs: Self) -> Self::Output {
        match self.checked_add(rhs) {
            Some(result) => result,
            _ => panic!("arithmetic overflow"),
        }
    }
}

impl PartialEq for U12 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl Display for U12 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[macro_export]
macro_rules! u12 {
    ( $x:expr ) => {{
        let x: usize = $x;
        U12::from_usize(x)
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn from_usize() {
        U12::from_usize(MIN);
        U12::from_usize(MAX);
    }

    #[test]
    #[should_panic]
    fn from_usize_panic() {
        U12::from_usize(4096);
    }

    #[test]
    fn checked_add() {
        assert_eq!(U12::checked_add(u12![4], u12![4]).unwrap(), u12![8]);
    }

    #[test]
    #[should_panic]
    fn checked_add_panic() {
        U12::checked_add(u12![4], u12![4096]);
    }
}
