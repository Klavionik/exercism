use std::collections::HashSet;
use std::iter::Iterator;
use std::sync::{Mutex, OnceLock};

const M: u64 = 4294967296;
const A: u64 = 1664525;
const C: u64 = 1013904223;

/// A very simple pseudo-random number generator
/// based on the Linear Congruential Generator algorithm.
struct Rng {
    last_state: u64,
}

impl Rng {
    pub fn new(seed: u64) -> Self {
        Self {
            last_state: seed % M,
        }
    }

    pub fn next_int(&mut self) -> u64 {
        self.last_state = (A * self.last_state + C) % M;
        self.last_state
    }

    pub fn range(&mut self, min: u64, max: u64) -> u64 {
        let range = max - min + 1;
        min + self.next_int() % range
    }
}

static GENERATOR: OnceLock<Mutex<Rng>> = OnceLock::new();
static USED_NAMES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn compose_name(rng: &mut Rng) -> String {
    let mut new_name = String::with_capacity(5);
    let alphabet = ('A'..='Z').collect::<Vec<char>>();
    
    new_name.push(alphabet[rng.range(0, 25) as usize]);
    new_name.push(alphabet[rng.range(0, 25) as usize]);
    new_name.push_str(&format!(
        "{}{}{}",
        rng.range(0, 9),
        rng.range(0, 9),
        rng.range(0, 9)
    ));
    
    new_name
}

fn generate_name() -> String {
    let rng_mutex = GENERATOR.get_or_init(|| Mutex::new(Rng::new(42)));
    let used_names_mutex = USED_NAMES.get_or_init(|| Mutex::new(HashSet::new()));

    let mut rng = rng_mutex.lock().unwrap();
    let mut used_names = used_names_mutex.lock().unwrap();

    loop {
        let new_name = compose_name(&mut rng);

        if !used_names.contains(&new_name) {
            used_names.insert(new_name.clone());
            break new_name
        }
    }
}
pub struct Robot(String);

impl Robot {
    pub fn new() -> Self {
        Self(generate_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = generate_name()
    }
}

impl Drop for Robot {
    fn drop(&mut self) {
        if let Some(used_names) = USED_NAMES.get() {
            used_names.lock().unwrap().remove(self.name());
        }
    }
}
