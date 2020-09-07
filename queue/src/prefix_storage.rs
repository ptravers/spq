use crate::error::Error;
use crate::storage::{StorageType, INTEGER_FROM_BYTES};
use rocksdb::{Options, DB};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use uuid::Uuid;

fn create_composite_key(prefix: &u64, key: &u64) -> [u8; 16] {
    let mut composite_key: [u8; 16] = [0; 16];
    let prefix_bytes = prefix.to_be_bytes();
    let key_bytes = key.to_be_bytes();

    composite_key[..8].clone_from_slice(&prefix_bytes[..8]);
    composite_key[8..(8 + 8)].clone_from_slice(&key_bytes[..8]);

    composite_key
}

pub struct PrefixStorage {
    storage_type: StorageType,
    size: AtomicUsize,
    folder_path: String,
}

impl PrefixStorage {
    pub fn new_integer(maybe_folder_path: Option<String>) -> PrefixStorage {
        match maybe_folder_path {
            Some(folder_path) => PrefixStorage {
                storage_type: StorageType::Durable,
                size: AtomicUsize::new(0),
                folder_path,
            },
            None => PrefixStorage {
                storage_type: StorageType::Memory,
                size: AtomicUsize::new(0),
                folder_path: format!("/tmp/spqr/{:?}", Uuid::new_v4()),
            },
        }
    }

    fn _get(&self, db: &DB, key: [u8; 16]) -> Result<u64, Error> {
        let maybe_bytes = db.get(&key)?;

        let bytes = maybe_bytes.ok_or_else(|| Error::Empty {
            message: "No element present".to_string(),
        })?;

        (INTEGER_FROM_BYTES)(bytes)
    }

    pub fn get(&self, prefix: &u64, key: &u64) -> Result<u64, Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        let maybe_bytes = db.get(&create_composite_key(prefix, key))?;

        let bytes = maybe_bytes.ok_or_else(|| Error::Empty {
            message: "No element present".to_string(),
        })?;

        (INTEGER_FROM_BYTES)(bytes)
    }

    fn maybe_flush(&self, db: &DB) -> Result<(), Error> {
        match self.storage_type {
            StorageType::Memory => Ok(()),
            StorageType::Durable => {
                db.flush()?;

                Ok(())
            }
        }
    }

    fn _put(&mut self, db: &DB, key: [u8; 16], value: u64) -> Result<(), Error> {
        self.size.fetch_add(1, Relaxed);

        db.put(&key, value.to_be_bytes())?;

        Ok(())
    }

    pub fn put(&mut self, prefix: &u64, key: &u64, value: u64) -> Result<(), Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        self.size.fetch_add(1, Relaxed);

        let bytes = value.to_be_bytes();

        db.put(&create_composite_key(prefix, key), bytes)?;

        Ok(())
    }

    pub fn update(
        &mut self,
        prefix: &u64,
        key: &u64,
        f: fn(value: u64) -> u64,
    ) -> Result<(), Error> {
        let db = &DB::open_default(self.folder_path.clone())?;
        let composite_key = create_composite_key(prefix, key);
        let value = self._get(db, composite_key)?;

        let new_value = (f)(value);

        self._put(db, composite_key, new_value)?;

        self.maybe_flush(db)
    }

    pub fn has_prefix(&self, prefix: &u64) -> Result<bool, Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        let mut has_prefix = false;

        for (key, _) in db.prefix_iterator(prefix.to_be_bytes()) {
            let integer_prefix = (INTEGER_FROM_BYTES)(key[0..8].to_vec())?;

            //FIXME: Checking the prefix like this is an ugly hack as prefix
            // iterator does not seem to actually filter the prefix as one would expect.
            if &integer_prefix == prefix {
                has_prefix = true;

                break;
            }
        }

        Ok(has_prefix)
    }

    pub fn filter_keys_by_prefix(
        &self,
        prefix: &u64,
        check: fn(value: u64) -> bool,
    ) -> Result<Vec<u64>, Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        let mut keys: Vec<u64> = vec![];

        for (key, value) in db.prefix_iterator(prefix.to_be_bytes()) {
            let integer_value = (INTEGER_FROM_BYTES)(value.to_vec())?;
            let integer_key = (INTEGER_FROM_BYTES)(key[8..16].to_vec())?;
            let integer_prefix = (INTEGER_FROM_BYTES)(key[0..8].to_vec())?;

            //FIXME: Checking the prefix like this is an ugly hack as prefix
            // iterator does not seem to actually filter the prefix as one would expect.
            if (check)(integer_value) && &integer_prefix == prefix {
                keys.push(integer_key);
            }
        }

        Ok(keys)
    }

    pub fn get_at_prefix(&self, prefix: &u64) -> Result<Vec<u64>, Error> {
        let db = &DB::open_default(self.folder_path.clone())?;

        let mut values: Vec<u64> = vec![];

        for (key, value) in db.prefix_iterator(prefix.to_be_bytes()) {
            let integer_value = (INTEGER_FROM_BYTES)(value.to_vec())?;
            let integer_prefix = (INTEGER_FROM_BYTES)(key[0..8].to_vec())?;

            //FIXME: Checking the prefix like this is an ugly hack as prefix
            // iterator does not seem to actually filter the prefix as one would expect.
            if &integer_prefix == prefix {
                values.push(integer_value);
            }
        }

        Ok(values)
    }

    pub fn is_empty(&self) -> bool {
        self.size.load(Relaxed) == 0
    }
}

impl Drop for PrefixStorage {
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
