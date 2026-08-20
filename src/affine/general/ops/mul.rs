//! All operations that take a scalar will panic if given +∞, -∞, or NaN.

use std::{
    collections::HashMap,
    ops::{Mul, MulAssign},
};

use super::super::{combine_noise, combine_noise_in_place, GeneralAffineForm, NoiseSymbol};
use crate::uncertain::Numeric;

// Canonical
impl Mul for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn mul(self, rhs: Self) -> Self::Output {
        let center = self.center * rhs.center;
        let mut noise_set =
            combine_noise(&self.noise_set, &rhs.noise_set, |a, b| rhs.center * a + self.center * b);
        let error = self.get_radius() * rhs.get_radius();
        if error != 0.0 {
            noise_set.insert(NoiseSymbol::new(), error);
        }
        GeneralAffineForm { center, noise_set }
    }
}

// Forwards to Mul<&Self> for GeneralAffineForm (commutation)
impl Mul<GeneralAffineForm> for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn mul(self, rhs: GeneralAffineForm) -> Self::Output {
        rhs * self
    }
}

// Forwards to MulAssign<&Self> for GeneralAffineForm (assign and return)
impl Mul<&Self> for GeneralAffineForm {
    type Output = Self;

    fn mul(mut self, rhs: &Self) -> Self::Output {
        self *= rhs;
        self
    }
}

// Forwards to MulAssign for GeneralAffineForm (assign and return)
impl Mul for GeneralAffineForm {
    type Output = Self;

    fn mul(mut self, rhs: Self) -> Self::Output {
        self *= rhs;
        self
    }
}

// Canonical
impl MulAssign<&Self> for GeneralAffineForm {
    fn mul_assign(&mut self, rhs: &Self) {
        self.center *= rhs.center;
        combine_noise_in_place(&mut self.noise_set, &rhs.noise_set, |a, b| {
            rhs.center * a + self.center * b
        });
        let error = self.get_radius() * rhs.get_radius();
        if error != 0.0 {
            self.noise_set.insert(NoiseSymbol::new(), error);
        }
    }
}

// Forwards to MulAssign<&Self> for GeneralAffineForm
impl MulAssign for GeneralAffineForm {
    fn mul_assign(&mut self, rhs: Self) {
        *self *= &rhs;
    }
}

// Canonical
impl Mul<Numeric> for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn mul(self, rhs: Numeric) -> Self::Output {
        assert!(rhs.is_finite(), "Multiplying general affine form by +∞, -∞ or NaN.");
        let (center, noise_set) = if rhs == 0.0 {
            (0.0, HashMap::new())
        } else {
            (
                self.center * rhs,
                self.noise_set.iter().map(|(symbol, coef)| (*symbol, coef * rhs)).collect(),
            )
        };
        GeneralAffineForm { center, noise_set }
    }
}

// Forwards to MulAssign<Numeric> for GeneralAffineForm (assign and return)
impl Mul<Numeric> for GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn mul(mut self, rhs: Numeric) -> Self::Output {
        self *= rhs;
        self
    }
}

// Forwards to Mul<Numeric> for &GeneralAffineForm (commutation)
impl Mul<&GeneralAffineForm> for Numeric {
    type Output = GeneralAffineForm;

    fn mul(self, rhs: &GeneralAffineForm) -> Self::Output {
        rhs * self
    }
}

// Forwards to Mul<Numeric> for GeneralAffineForm (commutation)
impl Mul<GeneralAffineForm> for Numeric {
    type Output = GeneralAffineForm;

    fn mul(self, rhs: GeneralAffineForm) -> Self::Output {
        rhs * self
    }
}

// Canonical
impl MulAssign<Numeric> for GeneralAffineForm {
    fn mul_assign(&mut self, rhs: Numeric) {
        assert!(rhs.is_finite(), "Multiplying general affine form by +∞, -∞ or NaN.");
        if rhs == 0.0 {
            self.degenerate_to_zero();
        } else {
            self.center *= rhs;
            self.noise_set.values_mut().for_each(|coef| *coef = *coef * rhs);
        }
    }
}
