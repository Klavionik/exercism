use std::borrow::Borrow;
use std::iter::Cycle;
use std::ops::Range;

#[derive(Clone)]
pub struct Xorcism<'a> {
    key: &'a [u8],
    cycle: Cycle<Range<usize>>,
}

impl<'a> Xorcism<'a> {
    pub fn new<Key>(key: &'a Key) -> Xorcism<'a>
    where
        Key: ?Sized + AsRef<[u8]>,
    {
        let key = key.as_ref();
        let cycle = (0..key.len()).cycle();
        Self { key, cycle }
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for (i, j) in std::iter::zip(0..data.len(), &mut self.cycle) {
            data[i] ^= self.key[j]
        }
    }

    pub fn munge<V, Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        V: Borrow<u8>,
        Data: IntoIterator<Item = V>,
    {
        let mut it = data.into_iter();

        std::iter::from_fn(move || {
            it.next().map(|byte| {
                let next_key_idx = self.cycle.next().unwrap();
                let next_key = self.key[next_key_idx];
                byte.borrow() ^ next_key
            })
        })
    }
}
