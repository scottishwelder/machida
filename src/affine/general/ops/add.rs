//! All operations that take a scalar will panic if given +∞, -∞, or NaN.

use std::ops::{Add, AddAssign};

use super::super::{combine_noise, combine_noise_in_place, GeneralAffineForm};
use crate::uncertain::Numeric;

// Canonical
impl Add for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn add(self, rhs: Self) -> Self::Output {
        let center = self.center + rhs.center;
        let noise_set = combine_noise(&self.noise_set, &rhs.noise_set, |a, b| a + b);
        GeneralAffineForm { center, noise_set }
    }
}

// Forwards to Add<&Self> for GeneralAffineForm (commutation)
impl Add<GeneralAffineForm> for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn add(self, rhs: GeneralAffineForm) -> Self::Output {
        rhs + self
    }
}

// Forwards to AddAssign<&Self> for GeneralAffineForm (assign and return)
impl Add<&Self> for GeneralAffineForm {
    type Output = Self;

    fn add(mut self, rhs: &Self) -> Self::Output {
        self += rhs;
        self
    }
}

// Forwards to AddAssign for GeneralAffineForm (assign and return)
impl Add for GeneralAffineForm {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

// Canonical
impl AddAssign<&Self> for GeneralAffineForm {
    fn add_assign(&mut self, rhs: &Self) {
        self.center += rhs.center;
        combine_noise_in_place(&mut self.noise_set, &rhs.noise_set, |a, b| a + b);
    }
}

// Forwards to AddAssign<&Self> for GeneralAffineForm
impl AddAssign for GeneralAffineForm {
    fn add_assign(&mut self, rhs: Self) {
        *self += &rhs;
    }
}

// Canonical
impl Add<Numeric> for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn add(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Adding +∞, -∞ or NaN to general affine form");
        let center = self.center + rhs;
        let noise_set = self.noise_set.clone();
        GeneralAffineForm { center, noise_set }
    }
}

// Forwards to AddAssign<Numeric> for GeneralAffineForm (assign and return)
impl Add<Numeric> for GeneralAffineForm {
    type Output = Self;

    fn add(mut self, rhs: Numeric) -> Self::Output {
        self += rhs;
        self
    }
}

// Forwards to Add<Numeric> for &GeneralAffineForm (commutation)
impl Add<&GeneralAffineForm> for Numeric {
    type Output = GeneralAffineForm;

    fn add(self, rhs: &GeneralAffineForm) -> Self::Output {
        rhs + self
    }
}

// Forwards to Add<Numeric> for GeneralAffineForm (commutation)
impl Add<GeneralAffineForm> for Numeric {
    type Output = GeneralAffineForm;

    fn add(self, rhs: GeneralAffineForm) -> Self::Output {
        rhs + self
    }
}

// Canonical
impl AddAssign<Numeric> for GeneralAffineForm {
    fn add_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Adding +∞, -∞ or NaN to general affine form");
        self.center += rhs;
    }
}
