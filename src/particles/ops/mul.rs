use std::ops::{Mul, MulAssign};

use super::super::Particles;
use crate::uncertain::Numeric;

// Canonical
impl Mul for &Particles {
    type Output = Particles;

    fn mul(self, rhs: Self) -> Self::Output {
        self.sample_map(rhs, |(a, b)| a * b)
    }
}

// Forwards to Mul<&Self> for ParticleSet (commutation)
impl Mul<Particles> for &Particles {
    type Output = Particles;

    fn mul(self, rhs: Particles) -> Self::Output {
        rhs * self
    }
}

// Forwards to MulAssign<&Self> for ParticleSet (assign and return)
impl Mul<&Self> for Particles {
    type Output = Self;

    fn mul(mut self, rhs: &Self) -> Self::Output {
        self *= rhs;
        self
    }
}

// Forwards to MulAssign for ParticleSet (assign and return)
impl Mul for Particles {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

// Canonical
impl MulAssign<&Self> for Particles {
    fn mul_assign(&mut self, rhs: &Self) {
        self.sample_map_in_place(rhs, |(a, b)| *a *= b);
    }
}

// Forwards to MulAssign<&Self> for ParticleSet
impl MulAssign for Particles {
    fn mul_assign(&mut self, rhs: Self) {
        *self *= &rhs;
    }
}

// Canonical
impl Mul<Numeric> for &Particles {
    type Output = Particles;

    fn mul(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Multiplying particle set by +∞, -∞ or NaN");
        self.map(|a| a * rhs)
    }
}

// Forwards to MulAssign<Numeric> for ParticleSet (assign and return)
impl Mul<Numeric> for Particles {
    type Output = Self;

    fn mul(mut self, rhs: Numeric) -> Self::Output {
        self *= rhs;
        self
    }
}

// Forwards to Mul<Numeric> for &ParticleSet (commutation)
impl Mul<&Particles> for Numeric {
    type Output = Particles;

    fn mul(self, rhs: &Particles) -> Self::Output {
        rhs * self
    }
}

// Forwards to Mul<Numeric> for ParticleSet (commutation)
impl Mul<Particles> for Numeric {
    type Output = Particles;

    fn mul(self, rhs: Particles) -> Self::Output {
        rhs * self
    }
}

// Canonical
impl MulAssign<Numeric> for Particles {
    fn mul_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Multiplying particle set by +∞, -∞ or NaN");
        self.map_in_place(|a| *a *= rhs);
    }
}
