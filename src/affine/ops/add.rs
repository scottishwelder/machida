//! All operations that take a scalar will panic if given +∞, -∞, or NaN.

use std::ops::{Add, AddAssign};

use super::super::Affine;
use crate::uncertain::Numeric;

// Canonical
impl Add for &Affine {
    type Output = Affine;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Affine::Empty, _) | (_, Affine::Empty) => Affine::Empty,
            (Affine::R, _) | (_, Affine::R) => Affine::R,
            (Affine::General(gen_1), Affine::General(gen_2)) => (gen_1 + gen_2).into(),
        }
    }
}

// Forwards to Add<&Self> for Affine (commutation)
impl Add<Affine> for &Affine {
    type Output = Affine;

    fn add(self, rhs: Affine) -> Self::Output {
        rhs + self
    }
}

// Forwards to AddAssign<&Self> for Affine (assign and return)
impl Add<&Self> for Affine {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

// Forwards to AddAssign for Affine (assign and return)
impl Add for Affine {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

// Canonical
impl AddAssign<&Self> for Affine {
    fn add_assign(&mut self, rhs: &Self) {
        match (&mut *self, rhs) {
            (Self::Empty, _) | (_, Self::Empty) => *self = Self::Empty,
            (Self::R, _) | (_, Self::R) => *self = Self::R,
            (Self::General(gen_1), Self::General(gen_2)) => *gen_1 += gen_2,
        }
    }
}

// Forwards to AddAssign<&Self> for Affine
impl AddAssign for Affine {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

// Canonical
impl Add<Numeric> for &Affine {
    type Output = Affine;

    fn add(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Adding +∞, -∞ or NaN to affine form");
        match self {
            Affine::Empty => Affine::Empty,
            Affine::R => Affine::R,
            Affine::General(general) => (general + rhs).into(),
        }
    }
}

// Forwards to AddAssign<Numeric> for Affine (assign and return)
impl Add<Numeric> for Affine {
    type Output = Self;

    fn add(mut self, rhs: Numeric) -> Self::Output {
        self += rhs;
        self
    }
}

// Forwards to Add<Numeric> for &Affine (commutation)
impl Add<&Affine> for Numeric {
    type Output = Affine;

    fn add(self, rhs: &Affine) -> Self::Output {
        rhs + self
    }
}

// Forwards to Add<Numeric> for Affine (commutation)
impl Add<Affine> for Numeric {
    type Output = Affine;

    fn add(self, rhs: Affine) -> Self::Output {
        rhs + self
    }
}

// Canonical
impl AddAssign<Numeric> for Affine {
    fn add_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Adding +∞, -∞ or NaN to affine form");
        match self {
            Self::Empty | Self::R => {}
            Self::General(general) => *general += rhs,
        }
    }
}
