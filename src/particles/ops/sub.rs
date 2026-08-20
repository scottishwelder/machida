use std::ops::{Sub, SubAssign};

use super::super::Particles;
use crate::uncertain::Numeric;

// Canonical
impl Sub for &Particles {
    type Output = Particles;

    fn sub(self, rhs: Self) -> Self::Output {
        self.sample_map(rhs, |(a, b)| a - b)
    }
}

// Canonical.
// Cannot forward to Sub<&Self> for ParticleSet because subtraction is not commutative.
impl Sub<Particles> for &Particles {
    type Output = Particles;

    fn sub(self, mut rhs: Particles) -> Self::Output {
        rhs.sample_map_in_place(self, |(b, a)| *b = a - *b);
        rhs
    }
}

// Forwards to SubAssign<&Self> for ParticleSet (assign and return)
impl Sub<&Self> for Particles {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

// Forwards to SubAssign for ParticleSet (assign and return)
impl Sub for Particles {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

// Canonical
impl SubAssign<&Self> for Particles {
    fn sub_assign(&mut self, rhs: &Self) {
        self.sample_map_in_place(rhs, |(a, b)| *a -= b);
    }
}

// Forwards to SubAssign<&Self> for ParticleSet
impl SubAssign for Particles {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

// Canonical
impl Sub<Numeric> for &Particles {
    type Output = Particles;

    fn sub(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Subtracting +∞, -∞ or NaN from particle set");
        self.map(|a| a - rhs)
    }
}

// Forwards to SubAssign<Numeric> for ParticleSet (assign and return)
impl Sub<Numeric> for Particles {
    type Output = Self;

    fn sub(mut self, rhs: Numeric) -> Self::Output {
        self -= rhs;
        self
    }
}

// Canonical.
// Cannot forward to Sub<Numeric> for &ParticleSet because subtraction is not commutative.
impl Sub<&Particles> for Numeric {
    type Output = Particles;

    fn sub(self, rhs: &Particles) -> Self::Output {
        assert!(self.is_finite(), "Subtracting particle set from +∞, -∞ or NaN");
        rhs.map(|b| self - b)
    }
}

// Canonical.
// Cannot forward to Sub<Numeric> for ParticleSet because subtraction is not commutative.
impl Sub<Particles> for Numeric {
    type Output = Particles;

    fn sub(self, mut rhs: Particles) -> Self::Output {
        assert!(self.is_finite(), "Subtracting particle set from +∞, -∞ or NaN");
        rhs.map_in_place(|b| *b = self - *b);
        rhs
    }
}

// Canonical
impl SubAssign<Numeric> for Particles {
    fn sub_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Subtracting +∞, -∞ or NaN from particle set");
        self.map_in_place(|a| *a -= rhs);
    }
}
