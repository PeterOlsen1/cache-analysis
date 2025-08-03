use crate::traits::SimpleCache;
use crate::utils::clean_files;
use rand::{Rng, distributions::Alphanumeric};
use rand_distr::{Distribution, Zipf};
use std::time::Instant;

pub struct TestConfig {
    pub num_samples: usize,
    pub num_accesses: usize,
    pub skew: f64,
}

pub fn run_test<T>(cache: &mut T, config: &TestConfig)
where
    T: SimpleCache,
{
    let _ = clean_files();

    let mut all_samples: Vec<(String, String)> = Vec::new();
    for _ in 0..config.num_samples {
        let id = generate_id();
        let text = generate_text();

        cache.put(&id, &text);
        all_samples.push((id, text));
    }
    let pattern = generate_access_pattern(config);

    let start = Instant::now();
    for idx in pattern {
        let (id, _) = &all_samples[idx];
        cache.get(id);
    }

    let diff = start.elapsed();
    println!("Testing: {}", cache.name());
    println!("Time for {} accesses: {:.3?}", config.num_accesses, diff);

    let _ = clean_files();
    ()
}

///
/// Simulate a sequence in which we will access
/// different data entries. Follows the zipf distribution
/// to most closely simulate regular human access.
///
fn generate_access_pattern(config: &TestConfig) -> Vec<usize> {
    let mut out = Vec::new();
    let zipf = Zipf::new(config.num_samples as u64, config.skew).unwrap();
    let mut rng = rand::thread_rng();
    for _ in 0..config.num_accesses {
        let idx = zipf.sample(&mut rng) as usize;
        out.push(idx % config.num_samples);
    }
    out
}

///
/// Generate random 6 character ID
///
fn generate_id() -> String {
    let mut rng = rand::thread_rng();
    let id: String = (0..6).map(|_| rng.sample(Alphanumeric) as char).collect();
    id
}

///
/// Generate 16 random characters, text to be used in a file
///
fn generate_text() -> String {
    let mut rng = rand::thread_rng();
    let text: String = (0..16).map(|_| rng.sample(Alphanumeric) as char).collect();
    text
}
