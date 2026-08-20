//! An [`Interval`] in the real numbers, with lower and higher bounds.
//!
//! Represents an unknown value contained in the interval.
//!
//! Notable cases:
//!
//! - [`NaN`] is never a valid bound;
//!   - Operations that take a scalar will panic if given [`NaN`].
//! - [-∞] and [+∞] are valid bounds;
//! - However, [[+∞], [+∞]] and [[-∞], [-∞]] are not valid intervals;
//!   - Operations that take a scalar will panic if given [+∞] or [-∞].
//! - [[-∞], [+∞]] is ℝ; and
//! - For all `a > b`, [a, b] is ∅.
//!
//! [+∞]: Numeric::INFINITY
//! [-∞]: Numeric::NEG_INFINITY
//! [`NaN`]: Numeric::NAN
//! [`Add`]: std::ops::Add
//! [`Sub`]: std::ops::Sub
//! [`Mul`]: std::ops::Mul
//! [`Div`]: std::ops::Div
//! [`Neg`]: std::ops::Neg
//! [some extra operations]: crate::uncertain::ExtraOps

mod error;
mod ops;

use std::{
    fmt::{Display, Formatter},
    num::FpCategory::{Infinite, Nan},
};

use crate::{
    uncertain::{Numeric, Uncertain},
    Affine,
};

pub use error::FromError;
use srmfpa::{CielArithmetic, FloorArithmetic};

#[must_use]
#[derive(Clone, Copy, Debug, PartialEq)]
/// A real interval.
///
/// See the [module-level documentation](self) for more information.
pub struct Interval(Numeric, Numeric);

impl Interval {
    /// Creates the interval [lo, hi].
    /// # Panics
    ///
    /// Panics if given [[+∞], [+∞]] or [[-∞], [-∞]]; or if [`NaN`] is passed as either bounds.
    ///
    /// [`try_from`] returns an error instead.
    ///
    /// [`NaN`]: Numeric::NAN
    /// [+∞]: Numeric::INFINITY
    /// [-∞]: Numeric::NEG_INFINITY
    /// [`try_from`]: Self::try_from
    pub fn new(lo: Numeric, hi: Numeric) -> Self {
        match (lo.classify(), hi.classify()) {
            (Nan, _) | (_, Nan) => {
                panic!("Interval bounds should never be NaN")
            }
            (Infinite, Infinite) if lo.is_sign_positive() == hi.is_sign_positive() => {
                panic!("[+∞, +∞] and [-∞, -∞] are not valid intervals")
            }
            (_, _) => Self(lo, hi),
        }
    }

    /// Creates the interval [value, value].
    ///
    /// # Panics
    ///
    /// Panics if given [+∞], [-∞] or [`NaN`].
    ///
    /// [`try_from`] returns an error instead.
    ///
    /// [`NaN`]: Numeric::NAN
    /// [+∞]: Numeric::INFINITY
    /// [-∞]: Numeric::NEG_INFINITY
    /// [`try_from`]: Self::try_from
    pub fn new_singleton(value: Numeric) -> Self {
        assert!(
            value.is_finite(),
            "Cannot create [+∞, +∞], [-∞, -∞] or [NaN, NaN]"
        );
        Self(value, value)
    }

    /// The only interval that represents ℝ ([[-∞], [+∞]]).
    ///
    /// [+∞]: Numeric::INFINITY
    /// [-∞]: Numeric::NEG_INFINITY
    pub const R: Self = Self(Numeric::NEG_INFINITY, Numeric::INFINITY);

    /// One (of many) interval that represents ∅ ([[+∞], [-∞]]).
    ///
    /// [+∞]: Numeric::INFINITY
    /// [-∞]: Numeric::NEG_INFINITY
    pub const EMPTY: Self = Self(Numeric::INFINITY, Numeric::NEG_INFINITY);

    /// The singleton \[`0`, `0`\].
    pub const ZERO: Self = Self(0.0, 0.0);

    #[must_use]
    /// Checks if the interval is empty.
    ///
    /// True for all `[a, b]` when `a > b`.
    pub fn is_empty(&self) -> bool {
        self.0 > self.1
    }

    #[must_use]
    #[allow(clippy::float_cmp)]
    /// Checks if the interval is a singleton.
    ///
    /// True only for `[a, a]`.
    pub fn is_singleton(&self) -> bool {
        self.0 == self.1
    }

    #[must_use]
    /// Returns the lower bound.
    pub fn lo(&self) -> Numeric {
        self.0
    }

    #[must_use]
    /// Returns the higher bound.
    pub fn hi(&self) -> Numeric {
        self.1
    }

    #[must_use]
    /// Returns the length of the interval,
    /// usually the difference between the higher and lower bounds.
    pub fn get_length(&self) -> Numeric {
        if self.is_empty() {
            0.0 // TODO: Is this correct?
        } else if *self == Self::R {
            Numeric::INFINITY
        } else {
            self.1 - self.0
        }
    }

    /// Returns whether an interval is entirely positive, entirely negative or contains zero.
    pub fn get_strong_sign(&self) -> StrongSignClass {
        if self.0 > 0.0 {
            StrongSignClass::Positive
        } else if self.1 < 0.0 {
            StrongSignClass::Negative
        } else {
            StrongSignClass::ContainsZero
        }
    }

    /// Returns whether an interval is
    /// entirely non-positive, entirely non-negative or contains zero in its interior.
    pub fn get_weak_sign(&self) -> WeakSignClass {
        if self.0 >= 0.0 {
            WeakSignClass::NonNegative
        } else if self.1 <= 0.0 {
            WeakSignClass::NonPositive
        } else {
            WeakSignClass::StraddlesZero
        }
    }
}

impl Uncertain for Interval {}

impl Display for Interval {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        if self.is_singleton() {
            write!(f, "<{}>", self.0)
        } else if *self == Self::R {
            write!(f, "ℝ")
        } else if self.is_empty() {
            write!(f, "∅")
        } else {
            write!(f, "[{}, {}]", self.0, self.1)
        }
    }
}

impl TryFrom<(Numeric, Numeric)> for Interval {
    type Error = FromError;

    /// Tries to creates the interval [lo, hi].
    fn try_from((lo, hi): (Numeric, Numeric)) -> Result<Self, Self::Error> {
        match (lo.classify(), hi.classify()) {
            (Nan, _) | (_, Nan) => Err(FromError::NaNBound),
            (Infinite, Infinite) if lo.is_sign_positive() == hi.is_sign_positive() => {
                Err(FromError::InvalidInfinity)
            }
            (_, _) => Ok(Self(lo, hi)),
        }
    }
}

impl TryFrom<Numeric> for Interval {
    type Error = FromError;

    /// Tries to creates the interval [value, value].
    fn try_from(value: Numeric) -> Result<Self, Self::Error> {
        match value.classify() {
            Nan => Err(FromError::NaNBound),
            Infinite => Err(FromError::InvalidInfinity),
            _ => Ok(Self(value, value)),
        }
    }
}

impl From<&Affine> for Interval {
    fn from(value: &Affine) -> Self {
        match value {
            Affine::R => Self::R,
            Affine::Empty => Self::EMPTY,
            Affine::General(general) => {
                let radius = general.get_radius();
                let center = general.get_center();
                Self(center.floor_sub(radius), center.ciel_sub(radius))
            }
        }
    }
}

#[must_use]
/// Describes whether an [`Interval`] is entirely positive, entirely negative or contains zero.
pub enum StrongSignClass {
    Positive,
    Negative,
    ContainsZero,
}

#[must_use]
#[derive(PartialEq)]
/// Describes whether an [`Interval`] is
/// entirely non-positive, entirely non-negative or contains zero in its interior.
pub enum WeakSignClass {
    NonNegative,
    NonPositive,
    StraddlesZero,
}
