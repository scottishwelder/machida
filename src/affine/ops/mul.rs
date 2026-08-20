//! All operations that take a scalar will panic if given +∞, -∞, or NaN.

use std::ops::{Mul, MulAssign};

use super::super::Affine;
use crate::uncertain::Numeric;

// Canonical
impl Mul for &Affine {
    type Output = Affine;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Affine::Empty, _) | (_, Affine::Empty) => Affine::Empty,
            (Affine::General(general), Affine::R) if general.is_zero() => Affine::singleton(0.0),
            (Affine::R, Affine::General(general)) if general.is_zero() => Affine::singleton(0.0),
            (Affine::R, _) | (_, Affine::R) => Affine::R,
            (Affine::General(gen_1), Affine::General(gen_2)) => (gen_1 * gen_2).into(),
        }
    }
}

// Forwards to Mul<&Self> for Affine (commutation)
impl Mul<Affine> for &Affine {
    type Output = Affine;

    fn mul(self, rhs: Affine) -> Self::Output {
        rhs * self
    }
}

// Forwards to MulAssign<&Self> for Affine (assign and return)
impl Mul<&Self> for Affine {
    type Output = Self;

    fn mul(mut self, rhs: &Self) -> Self::Output {
        self *= rhs;
        self
    }
}

// Forwards to MulAssign for Affine (assign and return)
impl Mul for Affine {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

// Canonical
impl MulAssign<&Self> for Affine {
    fn mul_assign(&mut self, rhs: &Self) {
        match (&mut *self, rhs) {
            (Affine::Empty, _) | (_, Affine::Empty) => *self = Affine::Empty,
            (Affine::General(general), Affine::R) if general.is_zero() => {
                *self = Affine::singleton(0.0);
            }
            (Affine::R, Affine::General(general)) if general.is_zero() => {
                *self = Affine::singleton(0.0);
            }
            (Affine::R, _) | (_, Affine::R) => *self = Affine::R,
            (Affine::General(gen_1), Affine::General(gen_2)) => *gen_1 *= gen_2,
        }
    }
}

// Forwards to MulAssign<&Self> for Affine
impl MulAssign for Affine {
    fn mul_assign(&mut self, rhs: Self) {
        *self *= &rhs;
    }
}

// Canonical
impl Mul<Numeric> for &Affine {
    type Output = Affine;

    fn mul(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Multiplying affine form by +∞, -∞ or NaN");
        match self {
            Affine::Empty => Affine::Empty,
            _ if rhs == 0.0 => Affine::singleton(0.0),
            Affine::R => Affine::R,
            Affine::General(general) => (general * rhs).into(),
        }
    }
}

// Forwards to MulAssign<Numeric> for Affine (assign and return)
impl Mul<Numeric> for Affine {
    type Output = Self;

    fn mul(mut self, rhs: Numeric) -> Self::Output {
        self *= rhs;
        self
    }
}

// Forwards to Mul<Numeric> for &Affine (commutation)
impl Mul<&Affine> for Numeric {
    type Output = Affine;

    fn mul(self, rhs: &Affine) -> Self::Output {
        rhs * self
    }
}

// Forwards to Mul<Numeric> for Affine (commutation)
impl Mul<Affine> for Numeric {
    type Output = Affine;

    fn mul(self, rhs: Affine) -> Self::Output {
        rhs * self
    }
}

// Canonical
impl MulAssign<Numeric> for Affine {
    fn mul_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Multiplying affine form by +∞, -∞ or NaN");
        match self {
            Affine::Empty => {}
            _ if rhs == 0.0 => *self = Affine::singleton(0.0),
            Affine::R => {}
            Affine::General(general) => *general *= rhs,
        }
    }
}
