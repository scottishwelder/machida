use super::super::Particles;
use crate::uncertain::ExtraOps;

impl ExtraOps for &Particles {
    type SqrtResult = Particles;

    type RecipResult = Particles;

    fn sqrt(self) -> Self::SqrtResult {
        self.iter().filter(|&&e| e >= 0.0).map(|e| e.sqrt()).collect()
    }

    fn recip(self) -> Self::RecipResult {
        self.map(|a| a.recip())
    }
}

impl ExtraOps for Particles {
    type SqrtResult = Particles;

    type RecipResult = Particles;

    fn sqrt(mut self) -> Self::SqrtResult {
        self.0.retain_mut(|e| {
            if *e < 0.0 {
                return false;
            }
            *e = e.sqrt();
            true
        });
        self
    }

    fn recip(mut self) -> Self::RecipResult {
        self.map_in_place(|a| *a = -*a);
        self
    }
}
