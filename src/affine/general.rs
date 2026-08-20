mod noise_symbol;
mod ops;

use std::{
    collections::HashMap,
    fmt::{Display, Formatter, Write as _},
};

use crate::{uncertain::Numeric, Interval};
pub use noise_symbol::NoiseSymbol;

type NoiseSet = HashMap<NoiseSymbol, Numeric>;

#[derive(Debug, Clone, PartialEq)]
pub struct GeneralAffineForm {
    center: Numeric,
    noise_set: NoiseSet,
}

impl GeneralAffineForm {
    pub fn singleton(value: Numeric) -> Self {
        // TODO: Improve message
        assert!(value.is_finite(), "The general form cannot be created...");
        Self { center: value, noise_set: NoiseSet::new() }
    }

    pub fn get_radius(&self) -> Numeric {
        self.noise_set.values().map(|x| x.abs()).sum()
    }

    pub fn get_bounds(&self) -> (Numeric, Numeric) {
        let radius = self.get_radius();
        let center = self.center;
        (center - radius, center + radius)
    }

    pub fn get_center(&self) -> Numeric {
        self.center
    }

    pub fn degenerate_to_zero(&mut self) {
        self.center = 0.0;
        self.noise_set.clear();
    }

    pub fn is_singleton(&self) -> Option<Numeric> {
        self.noise_set.iter().all(|(_, coef)| *coef == 0.0).then_some(self.center)
    }

    pub fn is_zero(&self) -> bool {
        self.is_singleton() == Some(0.0)
    }

    pub fn contains_zero(&self) -> bool {
        let (lo, hi) = self.get_bounds();
        lo <= 0.0 && hi >= 0.0
    }

    pub fn is_non_positive(&self) -> bool {
        let (_, hi) = self.get_bounds();
        hi <= 0.0
    }
}

impl From<GeneralAffineForm> for super::Affine {
    fn from(value: GeneralAffineForm) -> Self {
        Self::General(value)
    }
}

impl From<Interval> for GeneralAffineForm {
    fn from(value: Interval) -> Self {
        // TODO: Improve message
        assert!(!value.is_empty() && value != Interval::R, "The general form cannot be created...");
        let length = value.get_length();
        let radius = length / 2.0;

        let noise_set =
            if radius == 0.0 { HashMap::new() } else { [(NoiseSymbol::new(), radius)].into() };
        GeneralAffineForm { center: value.lo() + radius, noise_set }
    }
}

impl Display for GeneralAffineForm {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        let mut s = format!("{}", self.center);
        for (symbol, coef) in &self.noise_set {
            let _ = write!(s, " {coef:+}{symbol}");
        }
        write!(f, "{s}")
    }
}

/// Combines the noise symbols from both sets using the provided function.
fn combine_noise<F>(set_1: &NoiseSet, set_2: &NoiseSet, mut combine: F) -> NoiseSet
where
    F: FnMut(Numeric, Numeric) -> Numeric,
{
    let a = set_1.iter().map(|(symbol, coef_1)| {
        let coef_2 = set_2.get(symbol).copied().unwrap_or_default();
        (*symbol, *coef_1, coef_2)
    });
    let b = set_2.iter().filter_map(|(symbol, coef_2)| {
        (!set_1.contains_key(symbol)).then_some((*symbol, 0.0, *coef_2))
    });
    a.chain(b)
        .filter_map(|(symbol, coef_1, coef_2)| {
            let c = combine(coef_1, coef_2);
            (c != 0.0).then_some((symbol, c))
        })
        .collect()
}

/// Combines the noise symbols from both sets using the provided function.
/// The result will be `set_1`
fn combine_noise_in_place<F>(set_1: &mut NoiseSet, set_2: &NoiseSet, mut combine: F)
where
    F: FnMut(Numeric, Numeric) -> Numeric,
{
    set_1.retain(|symbol, coef_1| {
        let coef_2 = set_2.get(symbol).copied().unwrap_or_default();
        *coef_1 = combine(*coef_1, coef_2);
        *coef_1 != 0.0
    });
    for (symbol, coef_2) in set_2 {
        if !set_1.contains_key(symbol) {
            let c = combine(0.0, *coef_2);
            if c != 0.0 {
                set_1.insert(*symbol, c);
            }
        }
    }
}
