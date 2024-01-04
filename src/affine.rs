mod noise_symbol;

use super::Interval;
pub use noise_symbol::NoiseSymbol;
use std::collections::HashMap;
use std::ops::{Add, Sub};

type NumberType = f64;

#[derive(Debug, Clone)]
pub struct Affine {
    pub center: NumberType,
    pub noise: HashMap<NoiseSymbol, NumberType>,
}

impl Affine {
    // pub fn new() -> Self {
    //     Affine {
    //         center: 0.0,
    //         noise: HashMap::from([(noise_symbol::NoiseSymbol::new(), 1.0)]),
    //     }
    // }
    pub fn interval(&self) -> Interval {
        let radius: f64 = self.noise.values().sum();
        Interval::new(self.center - radius, self.center + radius)
    }
}

impl Add for &Affine {
    type Output = Affine;

    fn add(self, other: Self) -> Affine {
        let mut new_hash = HashMap::new();
        for (noise_symbol, first_coefficient) in &self.noise {
            let second_coefficient = other.noise.get(noise_symbol).unwrap_or(&0.0);
            let sum = first_coefficient + second_coefficient;
            if sum != 0.0 {
                new_hash.insert(*noise_symbol, sum);
            }
        }
        for (noise_symbol, second_coefficient) in &other.noise {
            if !self.noise.contains_key(noise_symbol) {
                new_hash.insert(*noise_symbol, *second_coefficient);
            };
        }
        Affine {
            center: self.center + other.center,
            noise: new_hash,
        }
    }
}

impl Sub for &Affine {
    type Output = Affine;

    fn sub(self, other: Self) -> Affine {
        let mut new_hash = HashMap::new();
        for (noise_symbol, first_coefficient) in &self.noise {
            let second_coefficient = other.noise.get(noise_symbol).unwrap_or(&0.0);
            let difference = first_coefficient - second_coefficient;
            if difference != 0.0 {
                new_hash.insert(*noise_symbol, difference);
            }
        }
        for (noise_symbol, second_coefficient) in &other.noise {
            if !self.noise.contains_key(noise_symbol) {
                new_hash.insert(*noise_symbol, -second_coefficient);
            };
        }
        Affine {
            center: self.center - other.center,
            noise: new_hash,
        }
    }
}
