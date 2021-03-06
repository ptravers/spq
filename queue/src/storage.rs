use crate::error::Error;
use rocksdb::{Options, DB};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use uuid::Uuid;

pub enum StorageType {
    Memory,
    Durable,
}

pub type DeserializeFn<V> = fn(value: V) -> Result<Vec<u8>, Error>;
pub type SerializeFn<V> = fn(value: Vec<u8>) -> Result<V, Error>;

pub const INTEGER_TO_BYTES: DeserializeFn<u64> = |integer| Ok(integer.to_be_bytes().to_vec());
pub const INTEGER_FROM_BYTES: SerializeFn<u64> = |bytes| {
    let mut sized_bytes: [u8; 8] = Default::default();
    sized_bytes.copy_from_slice(&bytes[0..8]);
    Ok(u64::from_be_bytes(sized_bytes))
};

pub struct Storage<V: Copy> {
    storage_type: StorageType,
    size: AtomicUsize,
    folder_path: String,
    to_bytes: DeserializeFn<V>,
    from_bytes: SerializeFn<V>,
}

impl<V> Storage<V>
where
    V: Copy,
{
    pub fn new_integer(maybe_folder_path: Option<String>) -> Storage<u64> {
        match maybe_folder_path {
            Some(folder_path) => Storage {
                storage_type: StorageType::Durable,
                size: AtomicUsize::new(0),
                folder_path,
                to_bytes: INTEGER_TO_BYTES,
                from_bytes: INTEGER_FROM_BYTES,
            },
            None => Storage {
                storage_type: StorageType::Memory,
                size: AtomicUsize::new(0),
                folder_path: format!("/tmp/spqr/{:?}", Uuid::new_v4()),
                to_bytes: INTEGER_TO_BYTES,
                from_bytes: INTEGER_FROM_BYTES,
            },
        }
    }

    pub fn new_bool(maybe_folder_path: Option<String>) -> Storage<bool> {
        let to_bytes: DeserializeFn<bool> = |boolean| {
            if boolean {
                Ok(vec![1])
            } else {
                Ok(vec![0])
            }
        };
        let from_bytes: SerializeFn<bool> = |bytes| {
            if bytes == vec![1] {
                Ok(true)
            } else if bytes == vec![0] {
                Ok(false)
            } else {
                Err(Error::new("Item not a valid boolean".to_string()))
            }
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

    fn _put(&mut self, db: &DB, key: &u64, value: V) -> Result<(), Error> {
        self.size.fetch_add(1, Relaxed);

        let bytes = (self.to_bytes)(value)?;

        db.put(&key.to_be_bytes(), bytes)?;

        Ok(())
    }

    pub fn put(&mut self, key: &u64, value: V) -> Result<(), Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        self._put(db, key, value)?;

        match self.storage_type {
            StorageType::Memory => Ok(()),
            StorageType::Durable => {
                db.flush()?;

                Ok(())
            }
        }
    }

    pub fn put_if_absent(&mut self, key: &u64, value: V) -> Result<bool, Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        match self._get(db, key) {
            Err(Error::Empty { .. }) => (),
            Err(e) => return Err(e),
            Ok(_) => return Ok(false),
        }

        self._put(db, key, value)?;

        match self.storage_type {
            StorageType::Memory => Ok(true),
            StorageType::Durable => {
                db.flush()?;

                Ok(true)
            }
        }
    }

    fn _get(&self, db: &DB, key: &u64) -> Result<V, Error> {
        let maybe_bytes = db.get(&key.to_be_bytes())?;

        let bytes = maybe_bytes.ok_or_else(|| Error::Empty {
            message: "No element present".to_string(),
        })?;

        (self.from_bytes)(bytes)
    }

    pub fn get(&self, key: &u64) -> Result<V, Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        self._get(db, key)
    }

    pub fn update(&mut self, key: &u64, f: fn(value: V) -> V) -> Result<V, Error> {
        let db = &DB::open_default(self.folder_path.clone())?;
        let value = self._get(db, key)?;

        let new_value = (f)(value);

        self._put(db, key, new_value)?;

        match self.storage_type {
            StorageType::Memory => Ok(new_value),
            StorageType::Durable => {
                db.flush()?;

                Ok(new_value)
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.size.load(Relaxed) == 0
    }
}

impl<V> Drop for Storage<V>
where
    V: Copy,
{
    fn drop(&mut self) {
        match self.storage_type {
            StorageType::Memory => match DB::destroy(&Options::default(), self.folder_path.clone())
            {
                Ok(_) => (),
                Err(e) => println!("failed to delete storage dir: {:?}", e),
            },
            StorageType::Durable => (),
        }
    }
}
