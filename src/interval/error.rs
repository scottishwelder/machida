/// Errors caused by the creation of an [`Interval`]
/// from ([`Numeric`], [`Numeric`]) or [`Numeric`].
///
/// [`Interval`]: super::Interval
/// [`Numeric`]: crate::uncertain::Numeric
pub enum FromError {
    /// [`NaN`] was passed as one of the bounds.
    ///
    /// [`NaN`]: crate::uncertain::Numeric::NAN
    NaNBound,
    /// Tried to create [[+∞], [+∞]] or [[-∞], [-∞]].
    ///
    /// [+∞]: crate::uncertain::Numeric::INFINITY
    /// [-∞]: crate::uncertain::Numeric::NEG_INFINITY
    InvalidInfinity,
}
