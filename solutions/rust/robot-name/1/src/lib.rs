use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};
use std::time::{SystemTime, UNIX_EPOCH};

const M: u64 = 4294967296;
const A: u64 = 1664525;
const C: u64 = 1013904223;

struct RNG {
    last_state: u32,
}

impl RNG {
    pub fn new(seed: u128) -> Self {
        Self {
            last_state: (seed % M as u128) as u32,
        }
    }

    pub fn next_u64(&mut self) -> u32 {
        self.last_state = ((A * (self.last_state as u64) + C) % M) as u32;
        self.last_state
    }

    pub fn range(&mut self, min: u32, max: u32) -> u32 {
        let range = max - min + 1;
        min + self.next_u64() % range
    }
}

impl Default for RNG {
    fn default() -> Self {
        let curr_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();
        Self::new(curr_time)
    }
}

static GENERATOR: OnceLock<Mutex<RNG>> = OnceLock::new();
static USED_NAMES: OnceLock<Mutex<HashSet<String>>> = OnceLock::new();

fn generate_name() -> String {
    let rng_mutex = GENERATOR.get_or_init(|| Mutex::new(RNG::default()));
    let used_names_mutex = USED_NAMES.get_or_init(|| Mutex::new(HashSet::new()));
    let mut rng = rng_mutex.lock().unwrap();
    let mut used_names = used_names_mutex.lock().unwrap();

    let mut new_name = String::new();
    let alphabet = ('A'..'Z').collect::<Vec<char>>();
    let mut go = true;

    while go {
        new_name.push(alphabet[rng.range(0, (alphabet.len() - 1) as u32) as usize]);
        new_name.push(alphabet[rng.range(0, (alphabet.len() - 1) as u32) as usize]);
        new_name.push_str(&format!(
            "{}{}{}",
            rng.range(0, 9),
            rng.range(0, 9),
            rng.range(0, 9)
        ));

        if used_names.contains(&new_name) {
            new_name.clear();
        } else {
            used_names.insert(new_name.clone());
            go = false;
        }
    }

    new_name
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