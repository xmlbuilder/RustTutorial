## RandomNumberGenerator Documentation

- A Rust utility providing a user-friendly random number generator API, inspired by C# and Python,  
  but built on top of the rand and rand_distr crates.
  This library wraps common random generation tasks into intuitive methods.

### Overview

- The RandomNumberGenerator struct offers:
  - Easy integer and floating-point random generation within ranges.
  - Multiple values at once (batch generation).
  - Shuffling collections.
  - Gaussian (normal), exponential, and beta distribution sampling.
  - Seeded reproducibility.
- Internally, it uses Box<dyn RngCore> to allow both ThreadRng (system entropy) and StdRng (seeded) generators.

### API Reference
- Constructors
  - new() -> Self
    - Creates a generator seeded from system entropy (ThreadRng).
  - from_seed(seed: u64) -> Self
    - Creates a reproducible generator seeded with a fixed value (StdRng).

- Uniform Random Values
  - next_int(min: i32, max: i32) -> i32
    - Returns a random integer in [min, max].
  - next_ints(min: i32, max: i32, count: usize) -> Vec<i32>
    - Returns count random integers in [min, max].
  
  - next_uint(min: u32, max: u32) -> u32
    - Returns a random unsigned integer in [min, max].
  - next_double01() -> f64
    - Returns a random double in [0.0, 1.0].
  - next_double(min: f64, max: f64) -> f64
    - Returns a random double in [min, max].
  - next_doubles(min: f64, max: f64, count: usize) -> Vec<f64>
    - Returns count random doubles in [min, max].

- Collection Operations
  - shuffle<T>(slice: &mut [T])
    - Randomly permutes elements in a slice.

- Gaussian Distribution
  - two_gaussians() -> (f64, f64)
    - Returns two independent samples from the standard normal distribution (mean 0, stddev 1).
  - gaussian(mean: f64, stddev: f64) -> f64
    - Returns a sample from a normal distribution with given mean and standard deviation.

  - Formula:

$$
X \sim \mathcal{N}(\mu, \sigma^2) \quad \Rightarrow \quad X = \mu + \sigma Z, \quad Z \sim \mathcal{N}(0,1)
$$

- Exponential Distribution

  - exponential(lambda: f64) -> f64
  - Returns a sample from an exponential distribution with rate parameter λ.
  - Formula:

$$
X \sim \text{Exp}(\lambda) \quad \Rightarrow \quad f(x) = \lambda e^{-\lambda x}, \quad x \geq 0
$$

- Beta Distribution
  - beta(alpha: f64, beta: f64) -> f64
  - Returns a sample from a beta distribution with shape parameters α and β.
  - Formula:

$$
X \sim \text{Beta}(\alpha, \beta) \quad \Rightarrow \quad f(x) = \frac{x^{\alpha-1}(1-x)^{\beta-1}}{B(\alpha,\beta)}, \quad 0 \leq x \leq 1 ]
$$

## Examples
```rust
fn main() {
    let mut rng = RandomNumberGenerator::new();

    // Uniform
    println!("Random int: {}", rng.next_int(-5, 5));
    println!("Random doubles: {:?}", rng.next_doubles(0.0, 1.0, 3));

    // Gaussian
    println!("Gaussian sample: {}", rng.gaussian(0.0, 1.0));

    // Exponential
    println!("Exponential sample: {}", rng.exponential(1.5));

    // Beta
    println!("Beta sample: {}", rng.beta(2.0, 5.0));
}
```
- Testing
  - Unit tests verify:
    - Range correctness for uniform integers and doubles.
    - Batch generation length and bounds.
    - Shuffle preserves elements.
    - Gaussian, exponential, and beta samples are finite and within expected domains.
    - Seed reproducibility.
   
---

## Sources
```rust
use rand::prelude::*;
use rand::distributions::{Uniform};
use rand::rngs::{StdRng, ThreadRng};
use rand_distr::{Beta, Exp, StandardNormal};

pub struct RandomNumberGenerator {
    rng: Box<dyn RngCore>,
}

impl RandomNumberGenerator {
    /// Create new RNG seeded from system entropy (ThreadRng)
    pub fn new() -> Self {
        Self { rng: Box::new(rand::thread_rng()) }
    }

    /// Create RNG from a fixed seed (StdRng)
    pub fn from_seed(seed: u64) -> Self {
        Self { rng: Box::new(StdRng::seed_from_u64(seed)) }
    }

    /// Random integer in [min, max] inclusive
    pub fn next_int(&mut self, min: i32, max: i32) -> i32 {
        let dist = Uniform::new_inclusive(min, max);
        dist.sample(&mut self.rng)
    }

    /// Multiple random integers in [min, max] inclusive
    pub fn next_ints(&mut self, min: i32, max: i32, count: usize) -> Vec<i32> {
        let dist = Uniform::new_inclusive(min, max);
        (0..count).map(|_| dist.sample(&mut self.rng)).collect()
    }


    /// Random unsigned integer in [min, max] inclusive
    pub fn next_uint(&mut self, min: u32, max: u32) -> u32 {
        let dist = Uniform::new_inclusive(min, max);
        dist.sample(&mut self.rng)
    }

    /// Random unsigned integer in [min, max] inclusive
    pub fn next_uints(&mut self, min: u32, max: u32, count: usize) -> Vec<u32> {
        let dist = Uniform::new_inclusive(min, max);
        (0..count).map(|_| dist.sample(&mut self.rng)).collect()
    }

    /// Random double in [0.0, 1.0]
    pub fn next_double01(&mut self) -> f64 {
        self.rng.r#gen::<f64>()
    }

    /// Random double in [min, max]
    pub fn next_double(&mut self, min: f64, max: f64) -> f64 {
        let dist = Uniform::new_inclusive(min, max);
        dist.sample(&mut self.rng)
    }

    /// Multiple random doubles in [min, max]
    pub fn next_doubles(&mut self, min: f64, max: f64, count: usize) -> Vec<f64> {
        let dist = Uniform::new_inclusive(min, max);
        (0..count).map(|_| dist.sample(&mut self.rng)).collect()
    }

    /// Shuffle a mutable slice
    pub fn shuffle<T>(&mut self, slice: &mut [T]) {
        slice.shuffle(&mut self.rng);
    }

    /// Two independent Gaussian samples (mean 0, stddev 1)
    pub fn two_gaussians(&mut self) -> (f64, f64) {
        let n1: f64 = StandardNormal.sample(&mut self.rng);
        let n2: f64 = StandardNormal.sample(&mut self.rng);
        (n1, n2)
    }


    /// Single Gaussian sample (mean, stddev)
    pub fn gaussian(&mut self, mean: f64, stddev: f64) -> f64 {
        let n: f64 = StandardNormal.sample(&mut self.rng);
        mean + stddev * n
    }


    /// Exponential distribution sample (lambda > 0)
    pub fn exponential(&mut self, lambda: f64) -> f64 {
        let dist = Exp::new(lambda).unwrap();
        dist.sample(&mut self.rng)
    }

    /// Beta distribution sample (alpha, beta > 0)
    pub fn beta(&mut self, alpha: f64, beta: f64) -> f64 {
        let dist = Beta::new(alpha, beta).unwrap();
        dist.sample(&mut self.rng)
    }
}
```

### Test codes
```rust
#[cfg(test)]
mod tests {
    use nurbslib::core::random_number_generator::RandomNumberGenerator;
    use super::*;

    #[test]
    fn test_next_int() {
        let mut rng = RandomNumberGenerator::new();
        let v = rng.next_int(-5, 5);
        assert!(v >= -5 && v <= 5);
    }

    #[test]
    fn test_next_ints() {
        let mut rng = RandomNumberGenerator::new();
        let vs = rng.next_ints(-10, 10, 5);
        assert_eq!(vs.len(), 5);
        assert!(vs.iter().all(|&x| x >= -10 && x <= 10));
    }

    #[test]
    fn test_next_uint() {
        let mut rng = RandomNumberGenerator::new();
        let v = rng.next_uint(3, 7);
        assert!(v >= 3 && v <= 7);
    }

    #[test]
    fn test_next_double01() {
        let mut rng = RandomNumberGenerator::new();
        let d = rng.next_double01();
        assert!(d >= 0.0 && d <= 1.0);
    }

    #[test]
    fn test_next_double() {
        let mut rng = RandomNumberGenerator::new();
        let d = rng.next_double(1.0, 5.0);
        assert!(d >= 1.0 && d <= 5.0);
    }

    #[test]
    fn test_next_doubles() {
        let mut rng = RandomNumberGenerator::new();
        let ds = rng.next_doubles(0.0, 1.0, 10);
        assert_eq!(ds.len(), 10);
        assert!(ds.iter().all(|&x| x >= 0.0 && x <= 1.0));
    }

    #[test]
    fn test_shuffle() {
        let mut rng = RandomNumberGenerator::new();
        let mut arr = [1, 2, 3, 4, 5];
        rng.shuffle(&mut arr);
        // 배열 길이는 그대로, 값은 동일 집합
        assert_eq!(arr.len(), 5);
        assert!(arr.iter().all(|&x| (1..=5).contains(&x)));
    }

    #[test]
    fn test_two_gaussians() {
        let mut rng = RandomNumberGenerator::new();
        let (g1, g2) = rng.two_gaussians();
        // 평균 0, 표준편차 1인 정규분포 샘플이므로 값은 제한 없음
        assert!(g1.is_finite());
        assert!(g2.is_finite());
    }

    #[test]
    fn test_from_seed_reproducibility() {
        let mut rng1 = RandomNumberGenerator::from_seed(12345);
        let mut rng2 = RandomNumberGenerator::from_seed(12345);

        let v1 = rng1.next_int(0, 100);
        let v2 = rng2.next_int(0, 100);
        assert_eq!(v1, v2); // 같은 시드면 같은 결과
    }
}


#[cfg(test)]
mod tests_case2 {
    use nurbslib::core::random_number_generator::RandomNumberGenerator;
    use super::*;

    #[test]
    fn test_gaussian() {
        let mut rng = RandomNumberGenerator::new();
        let g = rng.gaussian(0.0, 1.0);
        assert!(g.is_finite());
    }

    #[test]
    fn test_exponential() {
        let mut rng = RandomNumberGenerator::new();
        let e = rng.exponential(1.5);
        assert!(e >= 0.0); // exponential distribution is non-negative
    }

    #[test]
    fn test_beta() {
        let mut rng = RandomNumberGenerator::new();
        let b = rng.beta(2.0, 5.0);
        assert!(b >= 0.0 && b <= 1.0); // beta distribution is in [0,1]
    }

    #[test]
    fn test_next_ints() {
        let mut rng = RandomNumberGenerator::new();
        let vs = rng.next_ints(-10, 10, 5);
        assert_eq!(vs.len(), 5);
        assert!(vs.iter().all(|&x| x >= -10 && x <= 10));
    }
}
```
---


