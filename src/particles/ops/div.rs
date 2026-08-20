use std::ops::{Div, DivAssign};

use super::super::Particles;
use crate::uncertain::Numeric;

// Canonical
impl Div for &Particles {
    type Output = Particles;

    fn div(self, rhs: Self) -> Self::Output {
        self.sample_map(rhs, |(a, b)| a / b)
    }
}

// Canonical.
// Cannot forward to Div<&Self> for ParticleSet because division is not commutative.
impl Div<Particles> for &Particles {
    type Output = Particles;

    fn div(self, mut rhs: Particles) -> Self::Output {
        rhs.sample_map_in_place(self, |(b, a)| *b = a / *b);
        rhs
    }
}

// Forwards to DivAssign<&Self> for ParticleSet (assign and return)
impl Div<&Self> for Particles {
    type Output = Self;

    fn div(mut self, rhs: &Self) -> Self::Output {
        self /= rhs;
        self
    }
}

// Forwards to DivAssign for ParticleSet (assign and return)
impl Div for Particles {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

// Canonical
impl DivAssign<&Self> for Particles {
    fn div_assign(&mut self, rhs: &Self) {
        self.sample_map_in_place(rhs, |(a, b)| *a /= b);
    }
}

// Forwards to DivAssign<&Self> for ParticleSet
impl DivAssign for Particles {
    fn div_assign(&mut self, rhs: Self) {
        *self /= &rhs;
    }
}

// Canonical
impl Div<Numeric> for &Particles {
    type Output = Particles;

    fn div(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Dividing particle set by +∞, -∞ or NaN");
        self.map(|a| a / rhs)
    }
}

// Forwards to DivAssign<Numeric> for ParticleSet (assign and return)
impl Div<Numeric> for Particles {
    type Output = Self;

    fn div(mut self, rhs: Numeric) -> Self::Output {
        self /= rhs;
        self
    }
}

// Canonical.
// Cannot forward to Div<Numeric> for &ParticleSet because division is not commutative.
impl Div<&Particles> for Numeric {
    type Output = Particles;

    fn div(self, rhs: &Particles) -> Self::Output {
        assert!(self.is_finite(), "Dividing +∞, -∞ or NaN by particle set");
        rhs.map(|b| self / b)
    }
}

// Canonical.
// Cannot forward to Div<Numeric> for ParticleSet because division is not commutative.
impl Div<Particles> for Numeric {
    type Output = Particles;

    fn div(self, mut rhs: Particles) -> Self::Output {
        assert!(self.is_finite(), "Dividing +∞, -∞ or NaN by particle set");
        rhs.map_in_place(|b| *b = self / *b);
        rhs
    }
}

// Canonical
impl DivAssign<Numeric> for Particles {
    fn div_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Dividing particle set by +∞, -∞ or NaN");
        self.map_in_place(|a| *a /= rhs);
    }
}
