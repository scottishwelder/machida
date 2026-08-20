//! All operations that take a scalar will panic if given +∞, -∞, or NaN.

use std::ops::{Sub, SubAssign};

use super::super::{combine_noise, combine_noise_in_place, GeneralAffineForm};
use crate::uncertain::Numeric;

// Canonical
impl Sub for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn sub(self, rhs: Self) -> Self::Output {
        let center = self.center - rhs.center;
        let noise_set = combine_noise(&self.noise_set, &rhs.noise_set, |a, b| a - b);
        GeneralAffineForm { center, noise_set }
    }
}

// Canonical.
// Cannot forward to Sub<&Self> for GeneralAffineForm because subtraction is not commutative.
impl Sub<GeneralAffineForm> for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn sub(self, mut rhs: GeneralAffineForm) -> Self::Output {
        rhs.center = self.center - rhs.center;
        combine_noise_in_place(&mut rhs.noise_set, &self.noise_set, |a, b| b - a);
        rhs
    }
}

// Forwards to SubAssign<&Self> for GeneralAffineForm (assign and return)
impl Sub<&Self> for GeneralAffineForm {
    type Output = Self;

    fn sub(mut self, rhs: &Self) -> Self::Output {
        self -= rhs;
        self
    }
}

// Forwards to SubAssign for GeneralAffineForm (assign and return)
impl Sub for GeneralAffineForm {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

// Canonical
impl SubAssign<&Self> for GeneralAffineForm {
    fn sub_assign(&mut self, rhs: &Self) {
        self.center -= rhs.center;
        combine_noise_in_place(&mut self.noise_set, &rhs.noise_set, |a, b| a - b);
    }
}

// Forwards to SubAssign<&Self> for GeneralAffineForm
impl SubAssign for GeneralAffineForm {
    fn sub_assign(&mut self, rhs: Self) {
        *self -= &rhs;
    }
}

// Canonical
impl Sub<Numeric> for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn sub(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Subtracting +∞, -∞ or NaN to general affine form");
        let center = self.center - rhs;
        let noise_set = self.noise_set.clone();
        GeneralAffineForm { center, noise_set }
    }
}

// Forwards to SubAssign<Numeric> for GeneralAffineForm (assign and return)
impl Sub<Numeric> for GeneralAffineForm {
    type Output = Self;

    fn sub(mut self, rhs: Numeric) -> Self::Output {
        self -= rhs;
        self
    }
}

// Canonical.
// Cannot forward to Sub<Numeric> for &GeneralAffineForm because subtraction is not commutative.
impl Sub<&GeneralAffineForm> for Numeric {
    type Output = GeneralAffineForm;

    fn sub(self, rhs: &GeneralAffineForm) -> Self::Output {
        assert!(self.is_finite(), "Subtracting +∞, -∞ or NaN to general affine form");
        let center = self - rhs.center;
        let noise_set = rhs.noise_set.clone();
        GeneralAffineForm { center, noise_set }
    }
}

// Canonical.
// Cannot forward to Sub<Numeric> for GeneralAffineForm because subtraction is not commutative.
impl Sub<GeneralAffineForm> for Numeric {
    type Output = GeneralAffineForm;

    fn sub(self, mut rhs: GeneralAffineForm) -> Self::Output {
        assert!(self.is_finite(), "Subtracting +∞, -∞ or NaN to general affine form");
        rhs.center = self - rhs.center;
        rhs
    }
}

// Canonical
impl SubAssign<Numeric> for GeneralAffineForm {
    fn sub_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Subtracting +∞, -∞ or NaN to general affine form");
        self.center -= rhs;
    }
}
