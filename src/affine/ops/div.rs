//! All operations that take a scalar will panic if given +∞, -∞, or NaN.

use std::ops::{Div, DivAssign};

use super::super::Affine;
use crate::uncertain::Numeric;

// Canonical
impl Div for &Affine {
    type Output = Affine;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Affine::Empty, _) | (_, Affine::Empty) => Affine::Empty,
            (_, Affine::General(general)) if general.is_zero() => Affine::Empty,
            (Affine::General(general), _) if general.is_zero() => Affine::singleton(0.0),
            (Affine::R, _) | (_, Affine::R) => Affine::R,
            (Affine::General(_), Affine::General(gen_2)) if gen_2.contains_zero() => Affine::R,
            (Affine::General(gen_1), Affine::General(gen_2)) => (gen_1 / gen_2).into(),
        }
    }
}

// Canonical.
// Cannot forward to Div<&Self> for Affine because division is not commutative.
impl Div<Affine> for &Affine {
    type Output = Affine;

    fn div(self, rhs: Affine) -> Self::Output {
        match (self, rhs) {
            (Affine::Empty, _) | (_, Affine::Empty) => Affine::Empty,
            (_, Affine::General(general)) if general.is_zero() => Affine::Empty,
            (Affine::General(general), _) if general.is_zero() => Affine::singleton(0.0),
            (Affine::R, _) | (_, Affine::R) => Affine::R,
            (Affine::General(_), Affine::General(gen_2)) if gen_2.contains_zero() => Affine::R,
            (Affine::General(gen_1), Affine::General(gen_2)) => (gen_1 / gen_2).into(),
        }
    }
}

// Forwards to DivAssign<&Self> for Affine (assign and return)
impl Div<&Self> for Affine {
    type Output = Self;

    fn div(mut self, rhs: &Self) -> Self::Output {
        self /= rhs;
        self
    }
}

// Forwards to DivAssign for Affine (assign and return)
impl Div for Affine {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

// Canonical
impl DivAssign<&Self> for Affine {
    fn div_assign(&mut self, rhs: &Self) {
        match (&mut *self, rhs) {
            (Affine::Empty, _) | (_, Affine::Empty) => *self = Affine::Empty,
            (_, Affine::General(general)) if general.is_zero() => *self = Affine::Empty,
            (Affine::General(general), _) if general.is_zero() => *self = Affine::singleton(0.0),
            (Affine::R, _) | (_, Affine::R) => *self = Affine::R,
            (Affine::General(_), Affine::General(gen_2)) if gen_2.contains_zero() => {
                *self = Affine::R;
            }
            (Affine::General(gen_1), Affine::General(gen_2)) => *gen_1 /= gen_2,
        }
    }
}

// Forwards to DivAssign<&Self> for Affine
impl DivAssign for Affine {
    fn div_assign(&mut self, rhs: Self) {
        *self /= &rhs;
    }
}

// Canonical
impl Div<Numeric> for &Affine {
    type Output = Affine;

    fn div(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Dividing affine form by +∞, -∞ or NaN");
        match (self, rhs) {
            (_, 0.0) | (Affine::Empty, _) => Affine::Empty,
            (Affine::R, _) => Affine::R,
            (Affine::General(general), rhs) => (general / rhs).into(),
        }
    }
}

// Forwards to DivAssign<Numeric> for Affine (assign and return)
impl Div<Numeric> for Affine {
    type Output = Self;

    fn div(mut self, rhs: Numeric) -> Self::Output {
        self /= rhs;
        self
    }
}

// Canonical.
// Cannot forward to Div<Numeric> for &Affine because division is not commutative.
impl Div<&Affine> for Numeric {
    type Output = Affine;

    fn div(self, rhs: &Affine) -> Self::Output {
        assert!(self.is_finite(), "Dividing +∞, -∞ or NaN by affine form");
        match (self, rhs) {
            (_, Affine::Empty) => Affine::Empty,
            (_, Affine::General(general)) if general.is_zero() => Affine::Empty,
            (0.0, _) => Affine::singleton(0.0),
            (_, Affine::R) => Affine::R,
            (_, Affine::General(general)) if general.contains_zero() => Affine::R,
            (_, Affine::General(general)) => (self / general).into(),
        }
    }
}

// Canonical.
// Cannot forward to Div<Numeric> for Affine because division is not commutative.
impl Div<Affine> for Numeric {
    type Output = Affine;

    fn div(self, rhs: Affine) -> Self::Output {
        assert!(self.is_finite(), "Dividing +∞, -∞ or NaN by affine form");
        match (self, rhs) {
            (_, Affine::Empty) => Affine::Empty,
            (_, Affine::General(general)) if general.is_zero() => Affine::Empty,
            (0.0, _) => Affine::singleton(0.0),
            (_, Affine::R) => Affine::R,
            (_, Affine::General(general)) if general.contains_zero() => Affine::R,
            (_, Affine::General(general)) => (self / general).into(),
        }
    }
}

// Canonical
impl DivAssign<Numeric> for Affine {
    fn div_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Dividing affine form by +∞, -∞ or NaN");
        match (&mut *self, rhs) {
            (Affine::Empty, _) => {}
            (_, 0.0) => *self = Affine::Empty,
            (Affine::R, _) => {}
            (Affine::General(general), _) => *general /= rhs,
        }
    }
}
