use std::{collections::HashMap, hash::Hash};

pub fn frequency<K: Eq + Hash>(iter: impl Iterator<Item=K>) -> HashMap<K, u64> {
    let mut counts = HashMap::new();

    for item in iter {
        *counts.entry(item).or_default() += 1;
    }

    counts
}