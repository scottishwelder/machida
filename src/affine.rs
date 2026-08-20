mod general;
mod ops;

use std::fmt::{Display, Formatter};

use crate::{
    uncertain::{Numeric, Uncertain},
    Interval,
};
use general::GeneralAffineForm;

#[must_use]
#[derive(Clone, Debug, PartialEq)]
pub enum Affine {
    R,
    Empty,
    General(GeneralAffineForm),
}

impl Affine {
    pub fn singleton(value: Numeric) -> Self {
        GeneralAffineForm::singleton(value).into()
    }

    #[must_use]
    pub fn get_radius(&self) -> Numeric {
        match self {
            Affine::R => Numeric::INFINITY,
            Affine::Empty => 0.0,
            Affine::General(general) => general.get_radius(),
        }
    }
}

impl From<Interval> for Affine {
    fn from(value: Interval) -> Self {
        if value.is_empty() {
            return Self::Empty;
        }
        if value == Interval::R {
            return Self::R;
        }
        let value: GeneralAffineForm = value.into();
        value.into()
    }
}

impl Display for Affine {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Affine::R => write!(f, "ℝ"),
            Affine::Empty => write!(f, "∅"),
            Affine::General(general) => general.fmt(f),
        }
    }
}

impl Uncertain for Affine {}
