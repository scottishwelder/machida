//! All operations that take a scalar will panic if given +∞, -∞, or NaN.

use std::ops::{Sub, SubAssign};

use super::super::Affine;
use crate::uncertain::Numeric;

// Canonical
impl Sub for &Affine {
    type Output = Affine;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Affine::Empty, _) | (_, Affine::Empty) => Affine::Empty,
            (Affine::R, _) | (_, Affine::R) => Affine::R,
            (Affine::General(gen_1), Affine::General(gen_2)) => (gen_1 - gen_2).into(),
        }
    }
}

// Canonical.
// Cannot forward to Sub<&Self> for Affine because subtraction is not commutative.
impl Sub<Affine> for &Affine {
    type Output = Affine;

    fn sub(self, rhs: Affine) -> Self::Output {
        match (self, rhs) {
            (Affine::Empty, _) | (_, Affine::Empty) => Affine::Empty,
            (Affine::R, _) | (_, Affine::R) => Affine::R,
            (Affine::General(gen_1), Affine::General(gen_2)) => (gen_1 - gen_2).into(),
        }
    }
}

// Forwards to SubAssign<&Self> for Affine (assign and return)
impl Sub<&Self> for Affine {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

// Forwards to SubAssign for Affine (assign and return)
impl Sub for Affine {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

// Canonical
impl SubAssign<&Self> for Affine {
    fn sub_assign(&mut self, rhs: &Self) {
        match (&mut *self, rhs) {
            (Self::Empty, _) | (_, Self::Empty) => *self = Self::Empty,
            (Self::R, _) | (_, Self::R) => *self = Self::R,
            (Self::General(gen_1), Self::General(gen_2)) => *gen_1 -= gen_2,
        }
    }
}

// Forwards to SubAssign<&Self> for Affine
impl SubAssign for Affine {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

// Canonical
impl Sub<Numeric> for &Affine {
    type Output = Affine;

    fn sub(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Subtracting +∞, -∞ or NaN from affine form");
        match self {
            Affine::Empty => Affine::Empty,
            Affine::R => Affine::R,
            Affine::General(general) => (general - rhs).into(),
        }
    }
}

// Forwards to SubAssign<Numeric> for Affine (assign and return)
impl Sub<Numeric> for Affine {
    type Output = Self;

    fn sub(mut self, rhs: Numeric) -> Self::Output {
        self -= rhs;
        self
    }
}

// Canonical.
// Cannot forward to Sub<Numeric> for &Affine because subtraction is not commutative.
impl Sub<&Affine> for Numeric {
    type Output = Affine;

    fn sub(self, rhs: &Affine) -> Self::Output {
        assert!(self.is_finite(), "Subtracting affine form from +∞, -∞ or NaN");
        match rhs {
            Affine::Empty => Affine::Empty,
            Affine::R => Affine::R,
            Affine::General(general) => (self - general).into(),
        }
    }
}

// Canonical.
// Cannot forward to Sub<Numeric> for Affine because subtraction is not commutative.
impl Sub<Affine> for Numeric {
    type Output = Affine;

    fn sub(self, rhs: Affine) -> Self::Output {
        assert!(self.is_finite(), "Subtracting affine form from +∞, -∞ or NaN");
        match rhs {
            Affine::Empty => Affine::Empty,
            Affine::R => Affine::R,
            Affine::General(general) => (self - general).into(),
        }
    }
}

// Canonical
impl SubAssign<Numeric> for Affine {
    fn sub_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Subtracting +∞, -∞ or NaN from affine form");
        match self {
            Self::Empty | Self::R => {}
            Self::General(general) => *general -= rhs,
        }
    }
}
