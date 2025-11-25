use crossbeam::channel::{Receiver, Sender, unbounded};
use std::collections::HashMap;
use std::thread::scope;

const PILL: &str = "STOP";

fn worker(in_q: Receiver<&str>, out_q: Sender<HashMap<char, usize>>) {
    while let Ok(text) = in_q.recv() {
        if text == PILL {
            break
        }

        let mut hm = HashMap::<char, usize>::new();

        for c in text.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_ascii_lowercase()) {
            hm.entry(c).and_modify(|v| *v += 1).or_insert(1);
        }

        out_q.send(hm).expect("Cannot send back the result.");
    }
}

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new()
    }
    
    let (in_tx, in_rx) = unbounded();
    let (out_tx, out_rx) = unbounded();
    let mut freqs = HashMap::new();

    scope(|s| {
        let mut unprocessed = input.len();

        for _ in 1..=worker_count {
            s.spawn(|| worker(in_rx.clone(), out_tx.clone()));
        }

        for text in input {
            in_tx.send(text).expect("Cannot send the work.");
        }

        for _ in 1..=worker_count {
            in_tx.send(PILL).expect("Cannot send the pill.");
        }

        while let Ok(hm) = out_rx.recv() {
            for (k, v) in hm {
                freqs.entry(k).and_modify(|o| *o += v).or_insert(v);
            }

            unprocessed -= 1;

            if unprocessed == 0 {
                break
            }
        }
    });

    freqs
}