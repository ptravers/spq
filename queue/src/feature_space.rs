use crate::error::Error;
use crate::prefix_storage::PrefixStorage;
use crate::storage::Storage;
use std::collections::hash_map::DefaultHasher;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
pub struct FeatureValue {
    feature_name: String,
    value: usize,
}

#[allow(dead_code)]
impl FeatureValue {
    pub fn new(feature_name: String, value: usize) -> FeatureValue {
        FeatureValue {
            feature_name,
            value,
        }
    }

    pub fn get_name(&self) -> &String {
        &self.feature_name
    }

    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();

        self.hash(&mut hasher);

        hasher.finish()
    }
}

impl Hash for FeatureValue {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
        self.feature_name.hash(state);
    }
}

pub fn create_hash<H: Hash>(features: &[H]) -> u64 {
    let mut hasher = DefaultHasher::new();

    for feature in features {
        feature.hash(&mut hasher);
    }

    hasher.finish()
}

const TOTAL_ITEMS_KEY: u64 = 0;

const EPOCH_STEP_KEY: u64 = 1;

const ROOT_INDEX_KEY: u64 = 2;

const DIMENSION_KEY: u64 = 3;

const FEATURE_NAMES_KEY: u64 = 4;

pub struct FeatureSpace {
    metadata: Storage<u64>,
    feature_node_has_leaves: Storage<bool>,
    feature_node_value_items_at_index: PrefixStorage,
    feature_node_value_child_index: PrefixStorage,
    feature_value_to_epoch_step: Storage<u64>,
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

        metadata_storage.put_if_absent(&FEATURE_NAMES_KEY, feature_names_hash)?;

        metadata_storage.put_if_absent(&EPOCH_STEP_KEY, 0)?;

        metadata_storage.put_if_absent(&TOTAL_ITEMS_KEY, 0)?;

        metadata_storage.put_if_absent(&DIMENSION_KEY, features.len() as u64)?;

        let feature_space = FeatureSpace {
            metadata: metadata_storage,
            feature_node_has_leaves: Storage::<bool>::new_bool(
                maybe_folder_path
                    .clone()
                    .map(|folder_path| folder_path + "/node_has_leaves"),
            ),
            feature_node_value_items_at_index: PrefixStorage::new_integer(
                maybe_folder_path
                    .clone()
                    .map(|folder_path| folder_path + "/node_value_items_at_index"),
            ),
            feature_node_value_child_index: PrefixStorage::new_integer(
                maybe_folder_path
                    .clone()
                    .map(|folder_path| folder_path + "/node_value_child_index"),
            ),
            feature_value_to_epoch_step: Storage::<u64>::new_integer(
                maybe_folder_path.map(|folder_path| folder_path + "/value_to_epoch"),
            ),
        };

        Ok(feature_space)
    }

    pub fn epoch_step(&self) -> Result<u64, Error> {
        self.metadata.get(&EPOCH_STEP_KEY)
    }

    fn increment_epoch_step(&mut self) -> Result<u64, Error> {
        self.metadata
            .update(&EPOCH_STEP_KEY, |epoch_step| epoch_step + 1)
    }

    pub fn dimension(&self) -> Result<u64, Error> {
        self.metadata.get(&DIMENSION_KEY)
    }

    pub fn feature_names_hash(&self) -> Result<u64, Error> {
        self.metadata.get(&FEATURE_NAMES_KEY)
    }

    pub fn total_items(&self) -> Result<u64, Error> {
        match self.root_index() {
            Ok(root_index) => {
                let items_at_each_root_value = self
                    .feature_node_value_items_at_index
                    .get_at_prefix(&root_index)?;

                Ok(items_at_each_root_value.iter().sum())
            }
            Err(Error::Empty { .. }) => Ok(0),
            Err(e) => Err(e),
        }
    }

    pub fn increment_total_items(&mut self) -> Result<u64, Error> {
        self.metadata
            .update(&TOTAL_ITEMS_KEY, |total_items| total_items + 1)
    }

    pub fn decrement_total_items(&mut self) -> Result<u64, Error> {
        self.metadata.update(&TOTAL_ITEMS_KEY, |total_items| total_items - 1)
    }

    fn set_root_index(&mut self, index: u64) -> Result<(), Error> {
        let was_put = self.metadata.put_if_absent(&ROOT_INDEX_KEY, index)?;

        if was_put {
            Ok(())
        } else {
            Err(Error::new(format!(
                "Queue already initialized with root node at {:?}",
                ROOT_INDEX_KEY
            )))
        }
    }

    fn root_index(&self) -> Result<u64, Error> {
        self.metadata.get(&ROOT_INDEX_KEY)
    }

    pub fn peek_next_leaf_feature(&self) -> Result<Option<u64>, Error> {
        let mut current_node = self.root_index()?;

        for feature_space_layer in 0..self.dimension()? {
            let keys_greater_than_zero: Vec<u64> = self
                .feature_node_value_items_at_index
                .filter_keys_by_prefix(&current_node, |count| count > 0)?;

            let mut maybe_next_key: Option<&u64> = None;
            let mut lowest_epoch_step: u64 = self.epoch_step()?;

            for key in keys_greater_than_zero.iter() {
                let value_last_used_epoch_step = self.feature_value_to_epoch_step.get(key)?;

                if value_last_used_epoch_step < lowest_epoch_step {
                    maybe_next_key = Some(key);
                    lowest_epoch_step = value_last_used_epoch_step;
                }
            }

            match maybe_next_key {
                Some(next_key) if self.feature_node_has_leaves.get(&current_node)? => {
                    let leaf = self.feature_node_value_child_index.get(&current_node, &next_key)?;
                    return Ok(Some(leaf));
                },
                Some(next_key) => {
                    current_node = self.feature_node_value_child_index.get(&current_node, &next_key)?;
                },
                None if feature_space_layer != 0 => panic!(
                    "Feature space has lost track of number of values for each feature. Found node that should contain values but contains none {:?}",
                    current_node
                ),
                None => return Ok(None),
            }
        }

        Ok(None)
    }

    pub fn use_next_leaf_feature(&mut self) -> Result<Option<u64>, Error> {
        let next_epoch_step = self.epoch_step()? + 1;
        let mut current_node = self.root_index()?;
        let mut leaf_index: Option<u64> = None;

        for feature_space_layer in 0..self.dimension()? {
            let keys_greater_than_zero: Vec<u64> = self
                .feature_node_value_items_at_index
                .filter_keys_by_prefix(&current_node, |count| count > 0)?;

            let mut maybe_next_key: Option<&u64> = None;
            let mut lowest_epoch_step: u64 = next_epoch_step;

            for key in keys_greater_than_zero.iter() {
                let value_last_used_epoch_step = self.feature_value_to_epoch_step.get(key)?;

                if value_last_used_epoch_step < lowest_epoch_step {
                    maybe_next_key = Some(key);
                    lowest_epoch_step = value_last_used_epoch_step;
                }
            }

            match maybe_next_key {
                Some(next_key) if self.feature_node_has_leaves.get(&current_node)? => {
                    self.feature_value_to_epoch_step.put(next_key, next_epoch_step)?;
                    self.feature_node_value_items_at_index.update(&current_node, &next_key, |count| count-1)?;
                    leaf_index = Some(self.feature_node_value_child_index.get(&current_node, &next_key)?);
                    //FIXME: When implementing concurrent access we need to check
                    //that this incremented epoch step matches the one we are using
                    //during this use action.
                    self.increment_epoch_step()?;
                },
                Some(next_key) => {
                    self.feature_value_to_epoch_step.put(next_key, next_epoch_step)?;
                    self.feature_node_value_items_at_index.update(&current_node, &next_key, |count| count-1)?;
                    current_node = self.feature_node_value_child_index.get(&current_node, &next_key)?;
                },
                None if feature_space_layer != 0 => panic!(
                    "Feature space has lost track of number of values for each feature. Found node that should contain values but contains none {:?}",
                    current_node
                ),
                None => break,
            }
        }

        Ok(leaf_index)
    }

    pub fn add_item(
        &mut self,
        feature_values: Vec<FeatureValue>,
        leaf_index: u64,
    ) -> Result<(), Error> {
        let mut child_index = leaf_index;
        let mut height = 1;
        let currently_empty = self.epoch_step()? == 0;

        let all_feature_values_natural_order = feature_values.clone();

        let mut current_node_index;

        let root_index: u64;
        if currently_empty {
            let feature_names: Vec<&String> = feature_values
                .iter()
                .map(|feature| feature.get_name())
                .collect();
            root_index = create_hash(&feature_names);
            self.set_root_index(root_index)?;
        } else {
            root_index = self.root_index()?;
        }

        // Insert in to the graph in reverse so that we create the child before
        // the parent. We need to have created the child before the parent to have
        // a child id to add as one of the parents children.
        feature_values.reverse();

        for feature_value in feature_values.clone().iter() {
            let mut next_feature_values = all_feature_values_natural_order.clone();
            next_feature_values.truncate(height);

            // Use root index for final node
            if height != feature_values.len() {
                current_node_index = create_hash(&next_feature_values);
            } else {
                current_node_index = root_index;
            }

            let has_node = self
                .feature_node_value_child_index
                .has_prefix(&current_node_index)?;

            let value_hash = feature_value.get_hash();

            let current_index = self
                .feature_node_value_child_index
                .get(&current_node_index, &value_hash);

            match current_index {
                Ok(_) => {
                    self.feature_node_value_items_at_index.update(
                        &current_node_index,
                        &value_hash,
                        |count| count + 1,
                    )?
                }
                Err(Error::Empty { .. }) if has_node => {
                    self.feature_node_value_items_at_index.put(
                        &current_node_index,
                        &value_hash,
                        1,
                    )?;
                    self.feature_value_to_epoch_step
                        .put_if_absent(&value_hash, 0)?;
                    self.feature_node_value_child_index.put(
                        &current_node_index,
                        &value_hash,
                        child_index,
                    )?;
                }
                Err(Error::Empty { .. }) => {
                    self.feature_node_value_items_at_index.put(
                        &current_node_index,
                        &value_hash,
                        1,
                    )?;
                    self.feature_value_to_epoch_step
                        .put_if_absent(&value_hash, 0)?;

                    self.feature_node_value_child_index.put(
                        &current_node_index,
                        &value_hash,
                        child_index,
                    )?;

                    self.feature_node_has_leaves
                        .put(&current_node_index, height == 1)?;
                }
                Err(e) => return Err(e),
            }

            child_index = current_node_index;
            height += 1;
        }

        self.increment_epoch_step()?;

        Ok(())
    }
}
