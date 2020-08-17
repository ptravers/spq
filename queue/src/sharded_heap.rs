use crate::error::Error;
use crate::storage::StorageType;
use log::debug;
use rocksdb::{ColumnFamily, IteratorMode, Options, DB};
use uuid::Uuid;

pub struct ShardedHeap {
    storage_type: StorageType,
    folder_path: String,
    options: Options,
}

impl ShardedHeap {
    pub fn new(maybe_folder_path: Option<String>) -> Result<ShardedHeap, Error> {
        let mut options = Options::default();
        options.create_missing_column_families(true);
        options.create_if_missing(true);

        let heap = match maybe_folder_path {
            Some(folder_path) => ShardedHeap {
                storage_type: StorageType::Durable,
                folder_path,
                options,
            },
            None => ShardedHeap {
                storage_type: StorageType::Memory,
                folder_path: format!("/tmp/spqr/{:?}", Uuid::new_v4()),
                options,
            },
        };

        //TODO: We should check to see if the table exists and if it doesn't create it
        match DB::open(&heap.options, heap.folder_path.clone()) {
            Ok(_) => debug!("Either created a fresh items table or found an empty table at {:?}", heap.folder_path),
            Err(_) => debug!("Encountered an error opening the items table. Assuming an existing items table was found at {:?}", heap.folder_path)
        }

        Ok(heap)
    }

    fn maybe_flush(&self, db: &DB, cf_handle: &ColumnFamily) -> Result<(), Error> {
        match self.storage_type {
            StorageType::Memory => Ok(()),
            StorageType::Durable => {
                db.flush_cf(cf_handle)?;

                Ok(())
            }
        }
    }

    pub fn push(&mut self, epoch: u64, key: u64, value: Vec<u8>) -> Result<(), Error> {
        let cfs = &DB::list_cf(&self.options, self.folder_path.clone())?;
        let db = &mut DB::open_cf(&self.options, self.folder_path.clone(), cfs)?;

        match db.cf_handle(&key.to_string()) {
            Some(cf_handle) => {
                db.put_cf(cf_handle, epoch.to_be_bytes(), value)?;
            }
            None => {
                db.create_cf(key.to_string(), &self.options)?;
                match db.cf_handle(&key.to_string()) {
                    Some(cf_handle) => {
                        db.put_cf(cf_handle, epoch.to_be_bytes(), value)?;

                        self.maybe_flush(db, cf_handle)?;
                    }
                    None => {
                        panic!("Cannot get created column family");
                    }
                }
            }
        }

        Ok(())
    }

    pub fn peek(&self, key: u64) -> Result<Option<Vec<u8>>, Error> {
        let cfs = &DB::list_cf(&self.options, self.folder_path.clone())?;
        let db = &mut DB::open_cf(&self.options, self.folder_path.clone(), cfs)?;

        let mut result: Option<Vec<u8>> = None;

        match db.cf_handle(&key.to_string()) {
            Some(cf_handle) => {
                if let Some((_, value)) = db.iterator_cf(cf_handle, IteratorMode::Start).next() {
                    result = Some(value.to_vec());
                }
            }
            None => {
                return Err(Error::new(format!("No shard for key {:?}", key)));
            }
        }

        Ok(result)
    }

    pub fn pop(&mut self, key: u64) -> Result<Option<Vec<u8>>, Error> {
        let cfs = &DB::list_cf(&self.options, self.folder_path.clone())?;
        let db = &mut DB::open_cf(&self.options, self.folder_path.clone(), cfs)?;

        let mut result: Option<Vec<u8>> = None;

        match db.cf_handle(&key.to_string()) {
            Some(cf_handle) => {
                if let Some((key, value)) = db.iterator_cf(cf_handle, IteratorMode::Start).next() {
                    result = Some(value.to_vec());
                    db.delete_cf(cf_handle, key)?;

                    self.maybe_flush(db, cf_handle)?;
                }
            }
            None => {
                return Err(Error::new(format!("No shard for key {:?}", key)));
            }
        }

        Ok(result)
    }
}

impl Drop for ShardedHeap {
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
