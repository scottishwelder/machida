use super::super::{GeneralAffineForm, NoiseSymbol};
use crate::uncertain::{ExtraOps, Numeric};

impl ExtraOps for &GeneralAffineForm {
    type SqrtResult = GeneralAffineForm;

    type RecipResult = GeneralAffineForm;

    /// Calculates the square root of self.
    /// The approximation will consider self to be non-negative
    ///
    /// # Panics
    ///
    /// Panics if self is entirely non-positive
    fn sqrt(self) -> Self::SqrtResult {
        let (mut lo, hi) = self.get_bounds();
        assert!(hi > 0.0, "Square root of non-positive general affine form.");
        lo = lo.max(0.0);
        let (a, m, error) = chebyshev_sqrt(lo, hi);
        let mut result = a * self + m;
        if error != 0.0 {
            result.noise_set.insert(NoiseSymbol::new(), error);
        }
        result
    }

    /// Calculates the reciprocal of self.
    ///
    /// # Panics
    ///
    /// Panics if self contains zero.
    fn recip(self) -> Self::RecipResult {
        let (lo, hi) = self.get_bounds();
        assert!(!(lo <= 0.0 && hi >= 0.0), "Reciprocal of general affine form containing zero.");
        let (a, m, error) = min_range_reciprocal(lo, hi);
        let mut result = a * self + m;
        if error != 0.0 {
            result.noise_set.insert(NoiseSymbol::new(), error);
        }
        result
    }
}

impl ExtraOps for GeneralAffineForm {
    type SqrtResult = GeneralAffineForm;

    type RecipResult = GeneralAffineForm;

    /// Calculates the square root of self.
    /// The approximation will consider self to be non-negative
    ///
    /// # Panics
    ///
    /// Panics if self is entirely non-positive.
    fn sqrt(self) -> Self::SqrtResult {
        let (mut lo, hi) = self.get_bounds();
        assert!(hi > 0.0, "Square root of non-positive general affine form.");
        lo = lo.max(0.0);
        let (a, m, error) = chebyshev_sqrt(lo, hi);
        let mut result = a * self + m;
        if error != 0.0 {
            result.noise_set.insert(NoiseSymbol::new(), error);
        }
        result
    }

    /// Calculates the reciprocal of self.
    ///
    /// # Panics
    ///
    /// Panics if self contains zero.
    fn recip(self) -> Self::RecipResult {
        let (lo, hi) = self.get_bounds();
        assert!(!(lo <= 0.0 && hi >= 0.0), "Reciprocal of general affine form containing zero.");
        let (a, m, error) = min_range_reciprocal(lo, hi);
        let mut result = a * self + m;
        if error != 0.0 {
            result.noise_set.insert(NoiseSymbol::new(), error);
        }
        result
    }
}

fn min_range_reciprocal(mut lo: Numeric, mut hi: Numeric) -> (Numeric, Numeric, Numeric) {
    if hi < 0.0 {
        (lo, hi) = (hi, lo);
    }
    let a = -hi.powi(2).recip();
    let m_max = lo.recip() - a * lo;
    let m_min = hi.recip() - a * hi;
    let m = m_min.midpoint(m_max);
    let error = (m_max - m_min) / 2.0;
    (a, m, error)
}

// TODO: Fix doc
fn chebyshev_sqrt(lo: Numeric, hi: Numeric) -> (Numeric, Numeric, Numeric) {
    let lo_sqrt = lo.sqrt();
    let hi_sqrt = hi.sqrt();
    let a = (lo_sqrt + hi_sqrt).recip();
    let m = (lo_sqrt + hi_sqrt) / 8.0 + lo_sqrt * hi_sqrt / (2.0 * (lo_sqrt + hi_sqrt));
    let error = (hi_sqrt - lo_sqrt).powi(2) / (8.0 * (lo_sqrt + hi_sqrt));
    (a, m, error)
}
