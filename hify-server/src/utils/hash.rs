use std::hash::{DefaultHasher, Hash, Hasher};

#[macro_export]
macro_rules! stable_hash {
    ($($value:expr),+) => {{
        use ::std::hash::{DefaultHasher, Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        $( Hash::hash(&$value, &mut hasher); )+
        <DefaultHasher as Hasher>::finish(&hasher)
    }};
}

pub fn iter_stable_hash<T: Hash>(iter: impl ExactSizeIterator<Item = T>) -> u64 {
    let len = iter.len();

    let (xor, sum) = iter
        .into_iter()
        .map(|item| {
            let mut h = DefaultHasher::new();
            item.hash(&mut h);
            h.finish()
        })
        .fold((0_u64, 0_u64), |(xor, sum), h| {
            (xor ^ h, sum.wrapping_add(h))
        });

    // Combine with length for extra safety
    xor.wrapping_add(sum.rotate_left(5))
        .wrapping_add(u64::try_from(len).unwrap())
}
