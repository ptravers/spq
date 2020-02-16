use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::result::Result;
use std::result::Result::{Err, Ok};
pub mod feature_space;
use feature_space::{create_hash, FeatureSpace, FeatureValue};

#[allow(dead_code)]
pub struct SortingPriorityQueue<T: Clone + Ord> {
    epoch_step: usize,
    feature_space: FeatureSpace,
    items: HashMap<u64, BinaryHeap<T>>,
}

#[allow(dead_code)]
impl<T: Clone + Ord> SortingPriorityQueue<T> {
    pub fn new(feature_space: Vec<String>) -> SortingPriorityQueue<T> {
        SortingPriorityQueue {
            epoch_step: 0,
            feature_space: FeatureSpace::new(0, feature_space),
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T, features: Vec<FeatureValue>) -> Result<usize, &str> {
        if features.len() != self.feature_space.dimension() {
            return Err("Invalid feature vector must have same size as feature space");
        } else {
            let feature_names_hash = create_hash(&features, false);

            if feature_names_hash != self.feature_space.feature_names_hash() {
                return Err(
                    "Invalid feature vector must have same feature names as initialization",
                );
            }

            let hash = create_hash(&features, true);

            let mut features_copy = features.clone();

            return Ok({
                self.items
                    .entry(hash)
                    .or_insert({
                        self.feature_space.add_item(&mut features_copy, hash);
                        BinaryHeap::<T>::new()
                    })
                    .push(item);

                self.feature_space.epoch_step()
            });
        }
    }

    pub fn size(&self) -> usize {
        return self.feature_space.total_items();
    }

    pub fn peek(&self) -> Option<&T> {
        return self
            .feature_space
            .peek_next_leaf_feature()
            .and_then(|next| {
                self.items
                    .get(&next)
                    .and_then(|leaf_items| leaf_items.peek())
            });
    }

    pub fn next(&mut self) -> (Option<T>, usize) {
        let mut next_item: Option<T> = None;

        match self.feature_space.use_next_leaf_feature() {
            Some(next) => {
                self.items
                    .entry(next)
                    .and_modify(|leaf_items| next_item = leaf_items.pop());
            }
            None => {}
        }

        return (next_item, self.feature_space.epoch_step());
    }
}
