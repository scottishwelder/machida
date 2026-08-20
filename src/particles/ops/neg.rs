use std::ops::Neg;

use super::super::Particles;

impl Neg for &Particles {
    type Output = Particles;

    fn neg(self) -> Self::Output {
        self.map(|a| -a)
    }
}

impl Neg for Particles {
    type Output = Self;

    fn neg(mut self) -> Self::Output {
        self.map_in_place(|a| *a = -*a);
        self
    }
}
