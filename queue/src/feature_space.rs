use serde::{Deserialize, Serialize};
use sp_error::Error;
use sp_storage::Storage;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct FeatureValue {
    name: String,
    value: usize,
}

#[allow(dead_code)]
impl FeatureValue {
    pub fn new(name: String, value: usize) -> FeatureValue {
        FeatureValue { name, value }
    }
}

impl Hash for FeatureValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub fn create_hash(features: &[FeatureValue], include_value: bool) -> u64 {
    let mut hasher = DefaultHasher::new();

    for feature in features {
        feature.hash(&mut hasher);
        if include_value {
            feature.value.hash(&mut hasher);
        }
    }

    hasher.finish()
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct FeatureNodeValue {
    value: usize,
    items_at_index: usize,
    child_index: u64,
    hash: u64,
}

impl FeatureNodeValue {
    pub fn new(
        name: &str,
        value: usize,
        items_at_index: usize,
        child_index: u64,
    ) -> FeatureNodeValue {
        let mut hasher = DefaultHasher::new();

        value.hash(&mut hasher);
        name.hash(&mut hasher);

        FeatureNodeValue {
            value,
            items_at_index,
            child_index,
            hash: hasher.finish(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
struct FeatureNode {
    name: String,
    values: Vec<FeatureNodeValue>,
    has_leaves: bool,
}

impl FeatureNode {
    fn serialize(feature_node: FeatureNode) -> Result<Vec<u8>, Error> {
        let bytes = bincode::serialize(&feature_node)?;

        Ok(bytes)
    }

    fn deserialize(bytes: Vec<u8>) -> Result<FeatureNode, Error> {
        let feature_node = bincode::deserialize(&bytes)?;

        Ok(feature_node)
    }

    fn find_next(
        &self,
        feature_value_to_epoch_step: &Storage<u64>,
        current_epoch_step: u64,
    ) -> Result<Option<(usize, u64)>, Error> {
        let mut next: Option<(usize, u64)> = None;
        let mut lowest_epoch_step: u64 = current_epoch_step;

        for (i, value) in self.values.iter().enumerate() {
            let value_last_used_epoch_step = feature_value_to_epoch_step.get(value.hash)?;

            if value_last_used_epoch_step < lowest_epoch_step && value.items_at_index > 0 {
                next = Some((i, value.hash));
                lowest_epoch_step = value_last_used_epoch_step;
            }
        }

        Ok(next)
    }

    pub fn peek(
        &self,
        feature_value_to_epoch_step: &Storage<u64>,
        current_epoch_step: u64,
    ) -> Result<Option<&FeatureNodeValue>, Error> {
        let maybe_next = self.find_next(feature_value_to_epoch_step, current_epoch_step)?;

        Ok(maybe_next.and_then(|(next_index, _)| self.values.get(next_index)))
    }

    pub fn peek_and_update(
        &mut self,
        current_epoch_step: u64,
        feature_value_to_epoch_step: &mut Storage<u64>,
    ) -> Result<Option<FeatureNodeValue>, Error> {
        let maybe_next = self.find_next(feature_value_to_epoch_step, current_epoch_step)?;

        let mut maybe_next_node_value = None;

        if let Some((next_index, next_value_hash)) = maybe_next {
            feature_value_to_epoch_step.put(next_value_hash, current_epoch_step)?;

            maybe_next_node_value = self.values.get_mut(next_index).map(|next_node_value| {
                next_node_value.items_at_index -= 1;
                next_node_value.clone()
            });
        }

        Ok(maybe_next_node_value)
    }
}

const TOTAL_ITEMS_HASH: u64 = 0;

const EPOCH_STEP_HASH: u64 = 1;

const ROOT_INDEX_HASH: u64 = 2;

const DIMENSION_HASH: u64 = 3;

const FEATURE_NAMES_HASH: u64 = 4;

pub struct FeatureSpace {
    feature_tree: Storage<FeatureNode>,
    feature_value_to_epoch_step: Storage<u64>,
    metadata: Storage<u64>,
}

impl FeatureSpace {
    pub fn new(
        features: Vec<String>,
        maybe_folder_path: Option<String>,
    ) -> Result<FeatureSpace, Error> {
        let mut hasher = DefaultHasher::new();

        for feature in features.clone() {
            feature.hash(&mut hasher);
        }

        let feature_names_hash = hasher.finish();

        let mut metadata_storage = Storage::<u64>::new_integer(
            maybe_folder_path
                .clone()
                .map(|folder_path| folder_path + "/metadata"),
        );

        metadata_storage.put_if_absent(FEATURE_NAMES_HASH, feature_names_hash)?;

        metadata_storage.put_if_absent(EPOCH_STEP_HASH, 0)?;

        metadata_storage.put_if_absent(TOTAL_ITEMS_HASH, 0)?;

        metadata_storage.put_if_absent(DIMENSION_HASH, features.len() as u64)?;

        Ok(FeatureSpace {
            feature_tree: Storage::new(
                maybe_folder_path
                    .clone()
                    .map(|folder_path| folder_path + "/feature_tree"),
                FeatureNode::serialize,
                FeatureNode::deserialize,
            ),
            feature_value_to_epoch_step: Storage::<u64>::new_integer(
                maybe_folder_path.map(|folder_path| folder_path + "/value_to_epoch"),
            ),
            metadata: metadata_storage,
        })
    }

    pub fn epoch_step(&self) -> Result<u64, Error> {
        self.metadata.get(EPOCH_STEP_HASH)
    }

    fn increment_epoch_step(&mut self) -> Result<(), Error> {
        self.metadata
            .update(EPOCH_STEP_HASH, |epoch_step| epoch_step + 1)
    }

    pub fn dimension(&self) -> Result<u64, Error> {
        self.metadata.get(DIMENSION_HASH)
    }

    pub fn feature_names_hash(&self) -> Result<u64, Error> {
        self.metadata.get(FEATURE_NAMES_HASH)
    }

    pub fn total_items(&self) -> Result<u64, Error> {
        self.metadata.get(TOTAL_ITEMS_HASH)
    }

    fn increment_total_items(&mut self) -> Result<(), Error> {
        self.metadata
            .update(TOTAL_ITEMS_HASH, |total_items| total_items + 1)
    }

    fn decrement_total_items(&mut self) -> Result<(), Error> {
        self.metadata
            .update(TOTAL_ITEMS_HASH, |total_items| total_items - 1)
    }

    fn set_root_index(&mut self, index: u64) -> Result<(), Error> {
        let was_put = self.metadata.put_if_absent(ROOT_INDEX_HASH, index)?;

        if was_put {
            Ok(())
        } else {
            Err(Error::new(format!(
                "Queue already initialized with root node at {:?}",
                ROOT_INDEX_HASH
            )))
        }
    }

    fn root_index(&self) -> Result<u64, Error> {
        self.metadata.get(ROOT_INDEX_HASH)
    }

    pub fn peek_next_leaf_feature(&self) -> Result<Option<u64>, Error> {
        let mut next_node = self.root_index()?;

        for feature_space_layer in 0..self.dimension()? {
            let feature_node = self.feature_tree.get(next_node)?;
            let next_feature_node_value =
                feature_node.peek(&self.feature_value_to_epoch_step, self.epoch_step()?)?;

            if feature_node.has_leaves {
                return Ok(next_feature_node_value.map(|node_value| node_value.child_index));
            } else {
                match next_feature_node_value.map(|node_value| node_value.child_index) {
                    Some(next_index) => next_node = next_index,
                    None if feature_space_layer != 0 => panic!(
                        "Feature space has lost track of number of values for each feature. Found node that should contain values but contains none {:?}",
                        feature_node
                    ),
                    None => return Ok(None),
                }
            }
        }

        Ok(None)
    }

    pub fn use_next_leaf_feature(&mut self) -> Result<Option<u64>, Error> {
        let next_epoch_step = self.epoch_step()? + 1;
        let mut next_node = self.root_index()?;

        for feature_space_layer in 0..self.dimension()? {
            let mut feature_node = self.feature_tree.get(next_node)?;

            let next_feature_node_value = feature_node
                .peek_and_update(next_epoch_step, &mut self.feature_value_to_epoch_step)?;

            self.feature_tree.put(next_node, feature_node.clone())?;

            if feature_node.has_leaves {
                if next_feature_node_value.is_some() {
                    self.increment_epoch_step()?;
                    self.decrement_total_items()?;
                }

                return Ok(next_feature_node_value.map(|node_value| node_value.child_index));
            } else {
                match next_feature_node_value.map(|node_value| node_value.child_index) {
                    Some(next_index) => next_node = next_index,
                    None if feature_space_layer != 0 => panic!(
                        "Feature space has lost track of number of values for each feature. Found node that should contain values but contains none {:?}",
                        feature_node
                    ),
                    None => return Ok(None),
                }
            }
        }

        Ok(None)
    }

    fn put_first_node(
        &mut self,
        feature: &FeatureValue,
        child_index: u64,
        current_node_index: u64,
        has_leaves: bool,
    ) -> Result<(), Error> {
        let mut values = Vec::new();

        let value = FeatureNodeValue::new(&feature.name, feature.value, 1, child_index);

        values.push(value.clone());

        self.feature_value_to_epoch_step
            .put_if_absent(value.hash, 0)?;

        self.feature_tree.put(
            current_node_index,
            FeatureNode {
                name: feature.name.clone(),
                values,
                has_leaves,
            },
        )?;

        Ok(())
    }

    pub fn add_item(
        &mut self,
        features: &mut Vec<FeatureValue>,
        leaf_index: u64,
    ) -> Result<(), Error> {
        self.increment_total_items()?;
        self.increment_epoch_step()?;

        let mut child_index = leaf_index;
        let mut i = 1;
        let currently_empty = self.feature_tree.is_empty();

        let mut current_node_index;

        features.reverse();

        for feature in features.clone().iter() {
            let mut next_features = features.clone();
            next_features.reverse();
            next_features.truncate(i);
            current_node_index = create_hash(&next_features, i != features.len());

            match self.feature_tree.get(current_node_index) {
                Ok(mut feature_node) => {
                    let maybe_value = feature_node.values.iter_mut().find(|feature_node_value| {
                        feature_node_value.value == feature.value
                            && feature_node_value.child_index == child_index
                    });

                    match maybe_value {
                        Some(value) => {
                            value.items_at_index += 1;
                        }
                        None => {
                            let value =
                                FeatureNodeValue::new(&feature.name, feature.value, 1, child_index);

                            feature_node.values.push(value.clone());

                            self.feature_value_to_epoch_step
                                .put_if_absent(value.hash, 0)?;
                        }
                    }

                    self.feature_tree.put(current_node_index, feature_node)?;
                }
                Err(Error::Empty { .. }) => {
                    self.put_first_node(feature, child_index, current_node_index, i == 1)?;

                    if currently_empty && i == features.len() {
                        self.set_root_index(current_node_index)?;
                    }
                }
                Err(e) => return Err(e),
            }

            child_index = current_node_index;
            i += 1;
        }

        Ok(())
    }
}
