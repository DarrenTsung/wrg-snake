use random;
use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

pub struct PsuedoRandom {
    seed: usize,
    incr: RefCell<u64>,
}

impl PsuedoRandom {
    pub fn new() -> PsuedoRandom {
        PsuedoRandom {
            seed: (random() * (usize::max_value() as f64)) as usize,
            incr: RefCell::new(0),
        }
    }

    pub fn random(&self) -> f64 {
        let mut hasher = DefaultHasher::new();
        ((*self.incr.borrow() as usize) ^ self.seed).hash(&mut hasher);
        *self.incr.borrow_mut() += 1;
        hasher.finish() as f64 / u64::max_value() as f64
    }

    pub fn random_in_range(&self, low: usize, high: usize) -> usize {
        assert!(low <= high);
        let diff = high - low;
        if diff == 0 {
            return low;
        }

        low + ((self.random() * (diff as f64)) as usize % diff)
    }

    pub fn hash_value(&self, value: usize) -> u64 {
        let mut hasher = DefaultHasher::new();
        (value ^ self.seed).hash(&mut hasher);
        hasher.finish()
    }
}
