use std::ops::Neg;

use super::super::Affine;

impl Neg for &Affine {
    type Output = Affine;

    fn neg(self) -> Self::Output {
        match self {
            Affine::R => Affine::R,
            Affine::Empty => Affine::Empty,
            Affine::General(general) => (-general).into(),
        }
    }
}

impl Neg for Affine {
    type Output = Self;

    fn neg(self) -> Self::Output {
        match self {
            Self::R => Self::R,
            Self::Empty => Self::Empty,
            Self::General(general) => (-general).into(),
        }
    }
}
