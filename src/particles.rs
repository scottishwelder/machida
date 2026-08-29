//! A collection of real numbers.
//!
//! The collection should represent a sampling of some sort of probability distribution.

mod ops;

use std::{any::type_name, fmt::Display, ops::Deref};

use rand::{
    distr::{Distribution as _, Uniform},
    rng,
    seq::IndexedRandom as _,
};

use crate::{
    uncertain::{Numeric, Uncertain},
    Interval,
};

#[must_use]
#[derive(Debug)]
/// A collection of real numbers.
///
/// Can be created with [`new_empty`], [`from_interval`],
/// [`From<Vec<Numeric>>`] and [`FromIterator<Numeric>`].
///
/// See the [module-level documentation](self) for more details.
///
/// [`new_empty`]: Self::new_empty
/// [`from_interval`]: Self::from_interval
/// [`From<Vec<Numeric>>`]: Self::from
/// [`FromIterator<Numeric>`]: Self::from_iter
pub struct Particles(Box<[Numeric]>);

impl Particles {
    /// Creates an empty collection
    pub fn new_empty() -> Self {
        Box::<[f64]>::default().into()
    }

    /// Creates a new collection with `size` particles uniformly sampled from the interval.
    ///
    /// # Panics
    ///
    /// Panics if the length of the interval cannot be represented by a finite [`Numeric`].
    pub fn from_interval(interval: Interval, size: usize) -> Self {
        if interval.is_empty() {
            return Self::new_empty();
        }
        let rng = rng();
        let uni = Uniform::new_inclusive(interval.lo(), interval.hi()).unwrap();
        let v: Vec<_> = uni.sample_iter(rng).take(size).collect();
        v.into()
    }

    /// Creates a new collection by mapping each element.
    pub fn map<F: FnMut(&Numeric) -> Numeric>(&self, f: F) -> Self {
        self.iter().map(f).collect()
    }

    /// Maps each element in place.
    pub fn map_in_place<F: FnMut(&mut Numeric)>(&mut self, f: F) {
        self.0.iter_mut().for_each(f);
    }

    /// Creates a new collection by mapping two collections.
    /// The first one will be traversed in order and the second one will be sampled randomly.
    /// The result will have the same size as the first one, unless the second one is empty,
    /// in which case the result will also be empty.
    pub fn sample_map<F>(&self, other: &Particles, f: F) -> Self
    where
        F: FnMut((&Numeric, &Numeric)) -> Numeric,
    {
        let mut rng = rng();
        let Some(iter_b) = other.choose_iter(&mut rng) else { return Self::new_empty() };
        self.iter().zip(iter_b).map(f).collect()
    }

    /// The same as [`sample_map`](Self::sample_map),
    /// but no allocation is done and the result is stored in the first argument.
    pub fn sample_map_in_place<F>(&mut self, other: &Particles, f: F)
    where
        F: FnMut((&mut Numeric, &Numeric)),
    {
        match other.0.choose_iter(&mut rng()) {
            Some(iter_2) => self.0.iter_mut().zip(iter_2).for_each(f),
            None => *self = Particles::new_empty(),
        }
    }
}

impl Uncertain for Particles {}

impl From<Box<[Numeric]>> for Particles {
    fn from(value: Box<[Numeric]>) -> Self {
        Self(value)
    }
}

impl From<Vec<Numeric>> for Particles {
    fn from(value: Vec<Numeric>) -> Self {
        value.into_boxed_slice().into()
    }
}

impl FromIterator<Numeric> for Particles {
    fn from_iter<T: IntoIterator<Item = Numeric>>(iter: T) -> Self {
        Box::from_iter(iter).into()
    }
}

impl Display for Particles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Collection of {} {}", self.len(), type_name::<Numeric>())
    }
}

impl Deref for Particles {
    type Target = [Numeric];

    fn deref(&self) -> &Self::Target {
        self.0.deref()
    }
}

impl AsRef<[Numeric]> for Particles {
    fn as_ref(&self) -> &[Numeric] {
        self.0.as_ref()
    }
}
