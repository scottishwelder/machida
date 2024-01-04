use super::Interval;
use rand::distributions::{Distribution, Uniform};
use rand::thread_rng;

type NumberType = f64;

#[derive(Debug)]
pub struct Particles(Vec<NumberType>);

impl Particles {
    pub fn from_interval(interval: Interval, size: u32) -> Particles {
        let mut v = Vec::with_capacity(size as usize);
        let mut rng = thread_rng();
        let uni = Uniform::new_inclusive(interval.0, interval.1);
        for _ in 0..size {
            v.push(uni.sample(&mut rng));
        }
        Particles(v)
    }

    pub fn apply(&self, function: fn(NumberType) -> NumberType) -> Self {
        let size = self.0.len();
        let mut v = Vec::with_capacity(size);
        for i in 0..size {
            v.push(function(self.0[i]));
        }
        Self(v)
    }
}

impl Add for &Particles {
    type Output = Particles;

    fn add(self, other: Self) -> Particles {
        
    }
}
