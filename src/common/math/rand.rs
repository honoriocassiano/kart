use rand::distributions::uniform::SampleUniform;
use rand::prelude::*;

use std::cmp::PartialOrd;

pub fn rand() -> f64 {
    let mut rng = rand::thread_rng();

    rng.gen_range(0.0..=1.0)
}

pub fn rand_between<T: SampleUniform + PartialOrd>(min: T, max: T) -> T {
    let mut rng = rand::thread_rng();

    rng.gen_range(min..=max)
}
