use std::collections::hash_map::Entry;
use std::collections::HashMap;

#[derive(Default)]
pub struct Storage<V> {
    hash_map: HashMap<u64, V>,
}

impl<V> Storage<V> {
    pub fn new() -> Storage<V> {
        Storage {
            hash_map: HashMap::<u64, V>::new(),
        }
    }

    pub fn insert(&mut self, key: u64, value: V) -> Option<V> {
        self.hash_map.insert(key, value)
    }

    pub fn entry(&mut self, key: u64) -> Entry<u64, V> {
        self.hash_map.entry(key)
    }

    pub fn get(&self, key: u64) -> Option<&V> {
        self.hash_map.get(&key)
    }

    pub fn get_mut(&mut self, key: u64) -> Option<&mut V> {
        self.hash_map.get_mut(&key)
    }

    pub fn update(&mut self, key: u64, value: V) -> Option<V> {
        self.get(key).map(|_| value)
    }

    pub fn is_empty(&self) -> bool {
        self.hash_map.is_empty()
    }
}
