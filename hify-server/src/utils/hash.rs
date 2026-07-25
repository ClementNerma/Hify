use std::hash::{Hash, Hasher};

/// A simple FNV‑1a hasher with a fixed initial state.
pub struct FnvHasher(u64);

impl FnvHasher {
    pub fn new() -> Self {
        // FNV offset basis for 64‑bit
        FnvHasher(0xcbf2_9ce4_8422_2325)
    }
}

impl Hasher for FnvHasher {
    fn finish(&self) -> u64 {
        self.0
    }

    fn write(&mut self, bytes: &[u8]) {
        let mut hash = self.0;
        for &b in bytes {
            hash ^= u64::from(b);
            hash = hash.wrapping_mul(0x0100_0000_01b3); // FNV prime
        }
        self.0 = hash;
    }
}

#[macro_export]
macro_rules! stable_hash {
    ($($value: expr),+) => {{
        use ::std::hash::{Hash, Hasher};

        let mut hasher = $crate::utils::FnvHasher::new();
        $( Hash::hash(&$value, &mut hasher); )+
        hasher.finish()
    }};
}

pub fn unordered_iter_stable_hash<T: Hash>(iter: impl ExactSizeIterator<Item = T>) -> u64 {
    let len = iter.len();

    let hash = iter
        .into_iter()
        .map(|item| {
            let mut h = FnvHasher::new();
            item.hash(&mut h);
            h.finish()
        })
        .fold(0_u64, u64::wrapping_add);

    // Combine with length for extra safety
    hash.wrapping_add(u64::try_from(len).unwrap())
}
