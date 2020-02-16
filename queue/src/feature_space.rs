use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
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

pub fn create_hash(features: &Vec<FeatureValue>, include_value: bool) -> u64 {
    let mut hasher = DefaultHasher::new();

    for feature in features {
        feature.hash(&mut hasher);
        if include_value {
            feature.value.hash(&mut hasher);
        }
    }

    return hasher.finish();
}

#[derive(Debug, Clone)]
struct FeatureNodeValue {
    value: usize,
    items_at_index: usize,
    child_index: u64,
    hash: u64,
}

impl FeatureNodeValue {
    pub fn new(
        name: &String,
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

#[derive(Debug, Clone)]
struct FeatureNode {
    name: String,
    values: Vec<FeatureNodeValue>,
    has_leaves: bool,
}

impl FeatureNode {
    fn find_next(
        &self,
        feature_value_to_step: &HashMap<u64, usize>,
        current_step: &usize,
    ) -> Option<(usize, u64)> {
        let mut next: Option<(usize, u64)> = None;
        let mut lowest_step: &usize = current_step;

        for (i, value) in self.values.iter().enumerate() {
            let value_last_used_step = feature_value_to_step.get(&value.hash).unwrap();

            if value_last_used_step <= lowest_step && value.items_at_index > 0 {
                next = Some((i, value.hash));
                lowest_step = value_last_used_step;
            }
        }

        return next;
    }

    pub fn peek(
        &self,
        feature_value_to_step: &HashMap<u64, usize>,
        current_step: &usize,
    ) -> Option<&FeatureNodeValue> {
        self.find_next(feature_value_to_step, current_step)
            .and_then(|(next_index, _)| self.values.get(next_index))
    }

    pub fn peek_and_update(
        &mut self,
        current_step: &usize,
        feature_value_to_step: &mut HashMap<u64, usize>,
    ) -> Option<FeatureNodeValue> {
        self.find_next(feature_value_to_step, current_step)
            .and_then(|(next_index, next_value_hash)| {
                self.values.get_mut(next_index).map(|next_node_value| {
                    feature_value_to_step.insert(next_value_hash, current_step.clone());
                    next_node_value.items_at_index -= 1;
                    next_node_value
                })
            })
            .map(|value| value.to_owned())
    }
}

pub struct FeatureSpace {
    total_items: usize,
    step: usize,
    root_index: u64,
    dimension: usize,
    feature_names_hash: u64,
    feature_tree: HashMap<u64, FeatureNode>,
    feature_value_to_step: HashMap<u64, usize>,
}

impl FeatureSpace {
    pub fn new(step: usize, features: Vec<String>) -> FeatureSpace {
        let mut hasher = DefaultHasher::new();

        for feature in features.clone() {
            feature.hash(&mut hasher);
        }

        let feature_names_hash = hasher.finish();

        FeatureSpace {
            total_items: 0,
            step,
            root_index: 0,
            dimension: features.len(),
            feature_names_hash,
            feature_tree: HashMap::new(),
            feature_value_to_step: HashMap::new(),
        }
    }

    pub fn step(&self) -> usize {
        self.step
    }

    pub fn dimension(&self) -> usize {
        self.dimension
    }

    pub fn feature_names_hash(&self) -> u64 {
        self.feature_names_hash
    }

    pub fn total_items(&self) -> usize {
        self.total_items
    }

    pub fn peek_next_leaf_feature(&self) -> Option<u64> {
        let mut next_node = self.root_index;

        for feature_space_layer in 0..self.dimension {
            match self.feature_tree.get(&next_node) {
                Some(ref mut feature_node) if feature_node.has_leaves => {
                    let next_feature_node_value =
                        feature_node.peek(&self.feature_value_to_step, &self.step);
                    return next_feature_node_value.map(|node_value| node_value.child_index);
                }
                Some(feature_node) => {
                    let next_feature_node_value =
                        feature_node.peek(&self.feature_value_to_step, &self.step);
                    match next_feature_node_value.map(|node_value| node_value.child_index) {
                        Some(next_index) => next_node = next_index,
                        None if feature_space_layer != 0 => panic!(
                            "Feature space has lost track of number of values for each feature. Found node that should contain values but contains none {:?}",
                            feature_node
                        ),
                        None => return None,
                    }
                }
                None => panic!("Index in feature space {:?} is missing", next_node),
            }
        }

        return None;
    }

    pub fn use_next_leaf_feature(&mut self) -> Option<u64> {
        let next_step = self.step + 1;
        let mut next_node = self.root_index;

        for feature_space_layer in 0..self.dimension {
            match self.feature_tree.get_mut(&next_node) {
                Some(ref mut feature_node) if feature_node.has_leaves => {
                    let next_feature_node_value =
                        feature_node.peek_and_update(&next_step, &mut self.feature_value_to_step);

                    if next_feature_node_value.is_some() {
                        self.step = next_step;
                        self.total_items -= 1;
                    }

                    return next_feature_node_value.map(|node_value| node_value.child_index);
                }
                Some(feature_node) => {
                    let next_feature_node_value =
                        feature_node.peek_and_update(&next_step, &mut self.feature_value_to_step);

                    match next_feature_node_value.map(|node_value| node_value.child_index) {
                        Some(next_index) => next_node = next_index,
                        None if feature_space_layer != 0 => panic!(
                            "Feature space has lost track of number of values for each feature. Found node that should contain values but contains none {:?}",
                            feature_node
                        ),
                        None => return None,
                    }
                }
                None => panic!("Index in feature space {:?} is missing", next_node),
            }
        }

        return None;
    }

    pub fn add_item(&mut self, features: &mut Vec<FeatureValue>, leaf_index: u64) {
        self.total_items += 1;
        self.step += 1;

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

            match self.feature_tree.get_mut(&current_node_index) {
                Some(FeatureNode {
                    name: _,
                    values,
                    has_leaves: _,
                }) => {
                    let maybe_value = values.iter_mut().find(|feature_node_value| {
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

                            values.push(value.clone());

                            if !self.feature_value_to_step.contains_key(&value.hash) {
                                self.feature_value_to_step.insert(value.hash, self.step);
                            }
                        }
                    }
                }
                None => {
                    let mut values = Vec::new();

                    let value = FeatureNodeValue::new(&feature.name, feature.value, 1, child_index);

                    values.push(value.clone());

                    if !self.feature_value_to_step.contains_key(&value.hash) {
                        self.feature_value_to_step.insert(value.hash, self.step);
                    }

                    self.feature_tree.insert(
                        current_node_index,
                        FeatureNode {
                            name: feature.name.clone(),
                            values: values,
                            has_leaves: i == 1,
                        },
                    );

                    if currently_empty && i == features.len() {
                        self.root_index = current_node_index;
                    }
                }
            }

            child_index = current_node_index;
            i += 1;
        }
    }
}
