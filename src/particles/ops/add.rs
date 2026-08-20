use std::ops::{Add, AddAssign};

use super::super::Particles;
use crate::uncertain::Numeric;

// Canonical
impl Add for &Particles {
    type Output = Particles;

    fn add(self, rhs: Self) -> Self::Output {
        self.sample_map(rhs, |(a, b)| a + b)
    }
}

// Forwards to Add<&Self> for Particles (commutation)
impl Add<Particles> for &Particles {
    type Output = Particles;

    fn add(self, rhs: Particles) -> Self::Output {
        rhs + self
    }
}

// Forwards to AddAssign<&Self> for Particles (assign and return)
impl Add<&Self> for Particles {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

// Forwards to AddAssign for Particles (assign and return)
impl Add for Particles {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

// Canonical
impl AddAssign<&Self> for Particles {
    fn add_assign(&mut self, rhs: &Self) {
        self.sample_map_in_place(rhs, |(a, b)| *a += b);
    }
}

// Forwards to AddAssign<&Self> for Particles
impl AddAssign for Particles {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

// Canonical
impl Add<Numeric> for &Particles {
    type Output = Particles;

    fn add(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Adding +∞, -∞ or NaN to particle set");
        self.map(|a| a + rhs)
    }
}

// Forwards to AddAssign<Numeric> for Particles (assign and return)
impl Add<Numeric> for Particles {
    type Output = Self;

    fn add(mut self, rhs: Numeric) -> Self::Output {
        self += rhs;
        self
    }
}

// Forwards to Add<Numeric> for &Particles (commutation)
impl Add<&Particles> for Numeric {
    type Output = Particles;

    fn add(self, rhs: &Particles) -> Self::Output {
        rhs + self
    }
}

// Forwards to Add<Numeric> for Particles (commutation)
impl Add<Particles> for Numeric {
    type Output = Particles;

    fn add(self, rhs: Particles) -> Self::Output {
        rhs + self
    }
}

// Canonical
impl AddAssign<Numeric> for Particles {
    fn add_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Adding +∞, -∞ or NaN to particle set");
        self.map_in_place(|a| *a += rhs);
    }
}
