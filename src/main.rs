mod caches;
mod test;
mod traits;
mod utils;

use test::{run_test, TestConfig};

fn main() {
    let config = TestConfig {
        num_samples: 1000,
        num_accesses: 10000,
        skew: 1.0
    };

    let mut rand_cache = caches::Rand::new();
    run_test(&mut rand_cache, &config);

    let mut lru_cache = caches::LRU::new();
    run_test(&mut lru_cache, &config);

    let mut fifo_cache = caches::Fifo::new();
    run_test(&mut fifo_cache, &config);
}
