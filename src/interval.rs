use std::{
    fmt::{Display, Formatter},
    ops::Add,
};

type NumberType = f64;

#[derive(Debug)]
pub struct Interval(pub NumberType, pub NumberType);

impl Interval {
    pub fn new(lo: NumberType, hi: NumberType) -> Self {
        Interval(lo, hi)
    }

    pub fn is_singleton(&self) -> bool {
        self.0 == self.1
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Interval(self.0 + other.0, self.1 + other.1)
    }
}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.is_singleton() {
            write!(f, "<{}>", self.0)
        } else {
            write!(f, "[{}, {}]", self.0, self.1)
        }
    }
}
