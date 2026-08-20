//! Defines the base trait used in this crate

use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

/// The base numeric type for all representations in this crate.
pub type Numeric = f64;

// TODO: Should take into account operations with references?
/// Common trait between all uncertain types
pub trait Uncertain:
    Sized
    + Debug
    + Display
    + Add<Output = Self>
    + Add<Numeric, Output = Self>
    + AddAssign
    + AddAssign<Numeric>
    + Sub<Output = Self>
    + Sub<Numeric, Output = Self>
    + SubAssign
    + SubAssign<Numeric>
    + Mul<Output = Self>
    + Mul<Numeric, Output = Self>
    + MulAssign
    + MulAssign<Numeric>
    + Div<Output = Self>
    + Div<Numeric, Output = Self>
    + DivAssign
    + DivAssign<Numeric>
    + Neg
    + ExtraOps
{
}

pub trait ExtraOps {
    type SqrtResult;
    type ReciprocalResult;

    fn sqrt(self) -> Self::SqrtResult;
    fn reciprocal(self) -> Self::ReciprocalResult;
}
