use rocksdb::DB;
use sp_error::Error;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::fs;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use uuid::Uuid;

enum StorageType {
    Memory,
    Durable,
}

type DeserializeFn<V> = fn(value: V) -> Result<Vec<u8>, Error>;
type SerializeFn<V> = fn(value: Vec<u8>) -> Result<V, Error>;

pub struct Storage<V> {
    storage_type: StorageType,
    size: AtomicUsize,
    folder_path: String,
    to_bytes: DeserializeFn<V>,
    from_bytes: SerializeFn<V>,
}

impl<V> Storage<V> {
    pub fn new_integer(maybe_folder_path: Option<String>) -> Storage<u64> {
        let to_bytes: DeserializeFn<u64> = |integer| Ok(integer.to_be_bytes().to_vec());
        let from_bytes: SerializeFn<u64> = |bytes| {
            let mut sized_bytes: [u8; 8] = Default::default();
            sized_bytes.copy_from_slice(&bytes[0..8]);
            Ok(u64::from_be_bytes(sized_bytes))
        };
        match maybe_folder_path {
            Some(folder_path) => Storage {
                storage_type: StorageType::Durable,
                size: AtomicUsize::new(0),
                folder_path,
                to_bytes,
                from_bytes,
            },
            None => Storage {
                storage_type: StorageType::Memory,
                size: AtomicUsize::new(0),
                folder_path: format!("/tmp/spqr/{:?}", Uuid::new_v4()),
                to_bytes,
                from_bytes,
            },
        }
    }

    pub fn new(
        maybe_durable: Option<String>,
        to_bytes: DeserializeFn<V>,
        from_bytes: SerializeFn<V>,
    ) -> Storage<V> {
        match maybe_durable {
            Some(folder_path) => Storage {
                storage_type: StorageType::Durable,
                size: AtomicUsize::new(0),
                folder_path,
                to_bytes,
                from_bytes,
            },
            None => Storage {
                storage_type: StorageType::Memory,
                size: AtomicUsize::new(0),
                folder_path: format!("/tmp/spqr/{:?}", Uuid::new_v4()),
                to_bytes,
                from_bytes,
            },
        }
    }

    fn _put(&mut self, db: &DB, key: u64, value: V) -> Result<(), Error> {
        self.size.fetch_add(1, Relaxed);

        let bytes = (self.to_bytes)(value)?;

        db.put(&key.to_be_bytes(), bytes)?;

        Ok(())
    }

    pub fn put(&mut self, key: u64, value: V) -> Result<(), Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        match self.storage_type {
            StorageType::Memory => {
                self._put(db, key, value)?;

                Ok(())
            }
            StorageType::Durable => {
                self._put(db, key, value)?;

                db.flush()?;

                Ok(())
            }
        }
    }

    pub fn put_if_absent(&mut self, key: u64, value: V) -> Result<bool, Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        match self.storage_type {
            StorageType::Memory => {
                match self._get(db, key) {
                    Err(Error::Empty { .. }) => (),
                    Err(e) => return Err(e),
                    Ok(_) => return Ok(false),
                }

                self._put(db, key, value)?;

                Ok(true)
            }
            StorageType::Durable => {
                match self._get(db, key) {
                    Err(Error::Empty { .. }) => (),
                    Err(e) => return Err(e),
                    Ok(_) => return Ok(false),
                }

                self._put(db, key, value)?;

                db.flush()?;

                Ok(true)
            }
        }
    }

    fn _get(&self, db: &DB, key: u64) -> Result<V, Error> {
        let maybe_bytes = db.get(&key.to_be_bytes())?;

        let bytes = maybe_bytes.ok_or_else(|| Error::Empty {
            message: "No element present".to_string(),
        })?;

        (self.from_bytes)(bytes)
    }

    pub fn get(&self, key: u64) -> Result<V, Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        self._get(db, key)
    }

    pub fn update(&mut self, key: u64, f: fn(value: V) -> V) -> Result<(), Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        match self.storage_type {
            StorageType::Memory => {
                let value = self._get(db, key)?;

                let new_value = (f)(value);

                self._put(db, key, new_value)?;

                Ok(())
            }
            StorageType::Durable => {
                let value = self._get(db, key)?;

                let new_value = (f)(value);

                self._put(db, key, new_value)?;

                db.flush()?;

                Ok(())
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size.load(Relaxed) == 0
    }
}

impl<V> Drop for Storage<V> {
    fn drop(&mut self) {
        match self.storage_type {
            StorageType::Memory => match fs::remove_dir_all(self.folder_path.clone()) {
                Ok(_) => (),
                Err(e) => println!("failed to delete storage dir: {:?}", e),
            },
            StorageType::Durable => (),
        }
    }
}

#[derive(Eq, Clone)]
pub struct Item {
    pub data: Vec<u8>,
    epoch_created: u64,
}

impl Item {
    pub fn new(data: Vec<u8>, epoch_created: u64) -> Item {
        Item {
            data,
            epoch_created,
        }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.epoch_created.cmp(&self.epoch_created)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.epoch_created == other.epoch_created
    }
}

#[derive(Default)]
pub struct ShardedHeap {
    heaps: HashMap<u64, BinaryHeap<Item>>,
}

impl ShardedHeap {
    pub fn new() -> ShardedHeap {
        ShardedHeap::default()
    }

    pub fn push(&mut self, key: u64, value: Item) -> Result<(), Error> {
        self.heaps
            .entry(key)
            .or_insert_with(BinaryHeap::<Item>::new)
            .push(value);

        Ok(())
    }

    pub fn peek(&self, key: u64) -> Result<Option<Item>, Error> {
        let maybe_value = self
            .heaps
            .get(&key)
            .and_then(|leaf_items| leaf_items.peek());

        match maybe_value {
            Some(value) => Ok(Some(value.clone())),
            None => Ok(None),
        }
    }

    pub fn pop(&mut self, key: u64) -> Result<Option<Item>, Error> {
        let mut next_item: Option<Item> = None;

        self.heaps
            .entry(key)
            .and_modify(|leaf_items| next_item = leaf_items.pop());

        Ok(next_item)
    }
}
