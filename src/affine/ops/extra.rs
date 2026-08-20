use super::super::Affine;
use crate::uncertain::ExtraOps;

impl ExtraOps for &Affine {
    type SqrtResult = Affine;

    type RecipResult = Affine;

    fn sqrt(self) -> Self::SqrtResult {
        match self {
            Affine::R => Affine::R,
            Affine::Empty => Affine::Empty,
            Affine::General(general) if general.is_non_positive() => Affine::Empty,
            Affine::General(general) => general.sqrt().into(),
        }
    }

    fn recip(self) -> Self::RecipResult {
        match self {
            Affine::R => todo!(),
            Affine::Empty => todo!(),
            Affine::General(general) if general.contains_zero() => Affine::R,
            Affine::General(general) => general.recip().into(),
        }
    }
}

impl ExtraOps for Affine {
    type SqrtResult = Affine;

    type RecipResult = Affine;

    fn sqrt(self) -> Self::SqrtResult {
        match self {
            Affine::R => Affine::R,
            Affine::Empty => Affine::Empty,
            Affine::General(general) if general.is_non_positive() => Affine::Empty,
            Affine::General(general) => general.sqrt().into(),
        }
    }

    fn recip(self) -> Self::RecipResult {
        match self {
            Affine::R => todo!(),
            Affine::Empty => todo!(),
            Affine::General(general) if general.contains_zero() => Affine::R,
            Affine::General(general) => general.recip().into(),
        }
    }
}
