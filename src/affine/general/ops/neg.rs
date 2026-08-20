use std::ops::Neg;

use super::super::GeneralAffineForm;

impl Neg for &GeneralAffineForm {
    type Output = GeneralAffineForm;

    fn neg(self) -> Self::Output {
        let center = -self.center;
        let noise_set = self.noise_set.iter().map(|(symbol, coef)| (*symbol, -coef)).collect();
        GeneralAffineForm { center, noise_set }
    }
}

impl Neg for GeneralAffineForm {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.center = -self.center;
        self.noise_set.values_mut().for_each(|coef| *coef = -*coef);
        self
    }
}
