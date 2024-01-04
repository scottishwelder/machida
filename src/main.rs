use std::collections::HashMap;

use a::{Affine, NoiseSymbol, Particles};

fn main() {
    let a = Affine {
        center: 6.0,
        noise: HashMap::from([(NoiseSymbol::new(), 0.5)]),
    };
    let b = Affine {
        center: 4.0,
        noise: HashMap::from([(NoiseSymbol::new(), 0.5)]),
    };
    let r = &a + &b;
    println!("Hello, world\nThe answer is {:?} | {}", &r, r.interval());

    let p = Particles::from_interval(r.interval(), 10);
    println!("{:?}", p);
    println!("{:?}", p.apply(|x|2.0*x));
}
