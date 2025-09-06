use rand::{prelude::StdRng, prelude::Distribution, Rng, SeedableRng};
use rand::distr::StandardUniform;
use rand::distr::uniform::SampleUniform;
use std::ops::Range;
use std::cmp::PartialOrd;

pub struct RandomNumberGenerator {
    rng: StdRng,
}

impl RandomNumberGenerator {
    pub fn new() -> Self {
        Self {
            rng: StdRng::from_os_rng(),
        }
    }

    pub fn seeded(seed: u64) -> Self {
        Self {
            rng: StdRng::seed_from_u64(seed)
        }
    }

    pub fn random_range<T>(&mut self, range: Range<T>) -> T where T: PartialOrd + SampleUniform {
        self.rng.random_range(range)
    }

    pub fn random_next<T>(&mut self) -> T where StandardUniform: Distribution<T> {
        self.rng.random()
    }
}

impl Default for RandomNumberGenerator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_bounds() {
        let mut rng = RandomNumberGenerator::new();

        for _ in 0..1000 {
            let n = rng.random_range(1..10);
            assert!(n >= 1);
            assert!(n < 10);
        }
    }

    #[test]
    fn test_reproduceability() {
        let mut rng = (
            RandomNumberGenerator::seeded(1),
            RandomNumberGenerator::seeded(1),
        );

        (0..1000).for_each(|_| {
            assert_eq!(
                rng.0.random_range(u32::MIN..u32::MAX),
                rng.1.random_range(u32::MIN..u32::MAX),
            );
        });
    }

    #[test]
    fn test_next_types() {
        let mut rng = RandomNumberGenerator::new();
        let _: i32 = rng.random_next();
        let _ = rng.random_next::<f32>();
    }

    #[test]
    fn test_float() {
        let mut rng = RandomNumberGenerator::new();
        for _ in 0..1000 {
            let n = rng.random_range(-5000.0f32..5000.0f32);

            assert!(n.is_finite());
            assert!(n >=-5000.0);
            assert!(n < 5000.0);
        }
    }
}
