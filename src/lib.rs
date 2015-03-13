use std::collections::hash_state::{DefaultState, HashState};
use std::collections::{HashMap, HashSet};
use std::default::Default;
use std::hash::Hash;

pub mod fnv;

use fnv::FnvHasher;

/// A deterministic hashstate for HashMap
///
/// # Example
///
/// ```
///  let map = std::collections::HashMap::with_hash_state(FnvHashState::new());
/// ```
pub struct FnvHashState(DefaultState<FnvHasher>);

impl FnvHashState {
    fn new() -> FnvHashState {
        FnvHashState(Default::default())
    }
}

impl HashState for FnvHashState {
    type Hasher = FnvHasher;

    fn hasher(&self) -> FnvHasher {
        Default::default()
    }
}

pub type FnvHashMap<K, V> = HashMap<K, V, DefaultState<FnvHasher>>;
pub type FnvHashSet<V> = HashSet<V, DefaultState<FnvHasher>>;

pub fn new_fnvhashmap<K: Hash + Eq, V>() -> FnvHashMap<K, V> {
        Default::default()
}
pub fn new_fnvhashset<V: Hash + Eq>() -> FnvHashSet<V> {
        Default::default()
}

#[test]
fn test1() {
    let mut map1 = new_fnvhashmap();
    let mut map2 = HashMap::with_hash_state(FnvHashState::new());
    map1.insert(1, 2);
    map2.insert(1, 2);
}

