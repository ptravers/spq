use std::result::Result;
use std::result::Result::{Err, Ok};
pub mod feature_space;
use feature_space::{create_hash, FeatureSpace, FeatureValue};
pub mod sharded_heap;
use sharded_heap::ShardedHeap;
pub mod error;
pub mod prefix_storage;
pub mod storage;
use error::Error;

#[allow(dead_code)]
pub struct SortingPriorityQueue {
    feature_space: FeatureSpace,
    items: ShardedHeap,
}

#[allow(dead_code)]
impl SortingPriorityQueue {
    pub fn new(features: Vec<String>) -> Result<SortingPriorityQueue, Error> {
        Ok(SortingPriorityQueue {
            feature_space: FeatureSpace::new(features, None)?,
            items: ShardedHeap::new(None)?,
        })
    }

    pub fn new_durable(
        features: Vec<String>,
        folder_path: String,
    ) -> Result<SortingPriorityQueue, Error> {
        Ok(SortingPriorityQueue {
            feature_space: FeatureSpace::new(features, Some(folder_path.clone()))?,
            items: ShardedHeap::new(Some(folder_path))?,
        })
    }

    pub fn enqueue(&mut self, data: Vec<u8>, features: Vec<FeatureValue>) -> Result<u64, Error> {
        if features.len() as u64 != self.feature_space.dimension()? {
            Err(Error::new(
                "Invalid feature vector must have same size as feature space".to_string(),
            ))
        } else {
            let feature_names: Vec<&String> =
                features.iter().map(|feature| feature.get_name()).collect();
            let feature_names_hash = create_hash(&feature_names);

            if feature_names_hash != self.feature_space.feature_names_hash()? {
                return Err(Error::new(
                    "Invalid feature vector must have same feature names as initialization"
                        .to_string(),
                ));
            }

            let hash = create_hash(&features);

            let mut features_copy = features;

            self.feature_space.add_item(&mut features_copy, hash)?;

            let current_epoch_step = self.feature_space.epoch_step()?;

            self.items.push(current_epoch_step, hash, data)?;
            self.feature_space.increment_total_items()?;

            Ok(current_epoch_step)
        }
    }

    pub fn size(&self) -> Result<u64, Error> {
        self.feature_space.total_items()
    }

    pub fn peek(&self) -> Result<Option<Vec<u8>>, Error> {
        let maybe_next_leaf_feature = self.feature_space.peek_next_leaf_feature()?;

        let mut maybe_item = None;

        if let Some(next_leaf_feature) = maybe_next_leaf_feature {
            maybe_item = self.items.peek(next_leaf_feature)?;
        }

        Ok(maybe_item)
    }

    pub fn dequeue(&mut self) -> Result<(Option<Vec<u8>>, u64), Error> {
        let mut next_item: Option<Vec<u8>> = None;

        if let Some(next) = self.feature_space.use_next_leaf_feature()? {
            let maybe_next_item = self.items.pop(next)?;
            next_item = maybe_next_item;

            self.feature_space.decrement_total_items()?;
        }

        let epoch_step = self.feature_space.epoch_step()?;

        Ok((next_item, epoch_step))
    }

    pub fn get_epoch(&self) -> Result<u64, Error> {
        self.feature_space.epoch_step()
    }
}
