//! Defines the base trait used in this crate

use std::{
    fmt::{Debug, Display},
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

// TODO: Centralize base numeric type?
type NumberType = f64;

pub trait Uncertain:
    Sized
    + Debug
    + Display
    + Add<Output = Self>
    + Add<NumberType, Output = Self>
    + AddAssign
    + AddAssign<NumberType>
    + Sub<Output = Self>
    + Sub<NumberType, Output = Self>
    + SubAssign
    + SubAssign<NumberType>
    + Mul<Output = Self>
    + Mul<NumberType, Output = Self>
    + MulAssign
    + MulAssign<NumberType>
    + Div<Output = Self>
    + Div<NumberType, Output = Self>
    + DivAssign
    + DivAssign<NumberType>
    + Neg
{
}
