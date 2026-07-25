/// Errors caused by the creation of an [`Interval`]
/// from ([`Bound`], [`Bound`]) or [`Bound`].
///
/// [`Interval`]: super::Interval
/// [`Bound`]: super::Bound
pub enum FromError {
    /// [`NaN`] was passed as one of the bounds.
    ///
    /// [`NaN`]: super::Bound::NAN
    NaNBound,
    /// Tried to create [[+∞], [+∞]] or [[-∞], [-∞]].
    ///
    /// [+∞]: super::Bound::INFINITY
    /// [-∞]: super::Bound::NEG_INFINITY
    InvalidInfinity,
}
