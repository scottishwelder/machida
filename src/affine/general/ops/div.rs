//! All operations that take a scalar will panic if given +∞, -∞, or NaN.
//! Will panic if a divisor contains (or is) zero.

#![allow(clippy::suspicious_arithmetic_impl, reason = "div based on reciprocal")]
#![allow(clippy::suspicious_op_assign_impl, reason = "div based on reciprocal")]

use std::ops::{Div, DivAssign};

use super::super::GeneralAffineForm;
use crate::uncertain::{ExtraOps as _, Numeric};

// Canonical
impl Div for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.recip()
    }
}

// Canonical.
// Cannot forward to Div<&Self> for GeneralAffineForm because division is not commutative.
impl Div<GeneralAffineForm> for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn div(self, rhs: GeneralAffineForm) -> Self::Output {
        self * rhs.recip()
    }
}

// Forwards to DivAssign<&Self> for GeneralAffineForm (assign and return)
impl Div<&Self> for GeneralAffineForm {
    type Output = Self;

    fn div(mut self, rhs: &Self) -> Self::Output {
        self /= rhs;
        self
    }
}

// Forwards to DivAssign for GeneralAffineForm (assign and return)
impl Div for GeneralAffineForm {
    type Output = Self;

    fn div(mut self, rhs: Self) -> Self::Output {
        self /= rhs;
        self
    }
}

// Canonical
impl DivAssign<&Self> for GeneralAffineForm {
    fn div_assign(&mut self, rhs: &Self) {
        // TODO: Is it possible to avoid the reciprocal creating a new object?
        *self *= rhs.recip();
    }
}

// Canonical
// Won't forward to DivAssign<&Self> for GeneralAffineForm
// because we can avoid creating a new object here
impl DivAssign for GeneralAffineForm {
    fn div_assign(&mut self, rhs: Self) {
        // TODO: If the reciprocal creating a new object is fixed, this can be forwarded.
        *self *= rhs.recip();
    }
}

// Canonical
impl Div<Numeric> for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn div(self, rhs: Numeric) -> Self::Output {
        self * rhs.recip()
    }
}

// Forwards to DivAssign<Numeric> for GeneralAffineForm (assign and return)
impl Div<Numeric> for GeneralAffineForm {
    type Output = Self;

    fn div(mut self, rhs: Numeric) -> Self::Output {
        self /= rhs;
        self
    }
}

// Canonical.
// Cannot forward to Div<Numeric> for &GeneralAffineForm because division is not commutative.
impl Div<&GeneralAffineForm> for Numeric {
    type Output = GeneralAffineForm;

    fn div(self, rhs: &GeneralAffineForm) -> Self::Output {
        self * rhs.recip()
    }
}

// Canonical.
// Cannot forward to Div<Numeric> for GeneralAffineForm because division is not commutative.
impl Div<GeneralAffineForm> for Numeric {
    type Output = GeneralAffineForm;

    fn div(self, rhs: GeneralAffineForm) -> Self::Output {
        self * rhs.recip()
    }
}

// Canonical
impl DivAssign<Numeric> for GeneralAffineForm {
    fn div_assign(&mut self, rhs: Numeric) {
        *self *= rhs.recip();
    }
}
