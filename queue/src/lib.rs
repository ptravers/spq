use std::collections::hash_map::DefaultHasher;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use std::result::Result;
use std::result::Result::{Err, Ok};

#[derive(Debug, Clone)]
struct FeatureValue {
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

fn create_hash(features: &Vec<FeatureValue>, include_value: bool) -> u64 {
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
    index: u64,
    hash: u64,
}

impl FeatureNodeValue {
    pub fn new(name: &String, value: usize, items_at_index: usize, index: u64) -> FeatureNodeValue {
        let mut hasher = DefaultHasher::new();

        value.hash(&mut hasher);
        name.hash(&mut hasher);

        FeatureNodeValue {
            value,
            items_at_index,
            index,
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

    pub fn peek(&self, feature_value_to_step: &HashMap<u64, usize>, current_step: &usize) -> Option<&FeatureNodeValue> {
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

struct FeatureSpace {
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

    pub fn peek_next_leaf_feature(&self) -> Option<u64> {
        let mut next_node = self.root_index;

        for _ in 0..self.dimension {
            match self.feature_tree.get(&next_node) {
                Some(ref mut feature_node) if feature_node.has_leaves => {
                    let next_feature_node_value = feature_node.peek(&self.feature_value_to_step, &self.step);
                    return next_feature_node_value.map(|node_value| node_value.index);
                }
                Some(feature_node) => {
                    let next_feature_node_value = feature_node.peek(&self.feature_value_to_step, &self.step);
                    match next_feature_node_value.map(|node_value| node_value.index) {
                        Some(next_index) => next_node = next_index,
                        None => return None,
                    }
                }
                None => {}
            }
        }

        return None;
    }

    pub fn use_next_leaf_feature(&mut self) -> Option<u64> {
        self.step += 1;
        let mut next_node = self.root_index;

        for _ in 0..self.dimension {
            match self.feature_tree.get_mut(&next_node) {
                Some(ref mut feature_node) if feature_node.has_leaves => {
                    let next_feature_node_value =
                        feature_node.peek_and_update(&self.step, &mut self.feature_value_to_step);
                    return next_feature_node_value.map(|node_value| node_value.index);
                }
                Some(feature_node) => {
                    let next_feature_node_value =
                        feature_node.peek_and_update(&self.step, &mut self.feature_value_to_step);

                    match next_feature_node_value.map(|node_value| node_value.index) {
                        Some(next_index) => next_node = next_index,
                        None => return None,
                    }
                }
                None => {}
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
                Some(FeatureNode{
                    name: _,
                    values,
                    has_leaves: _,
                }) => {
                    let maybe_value = values.iter_mut().find(|feature_node_value| {
                        feature_node_value.value == feature.value
                            && feature_node_value.index == child_index
                    });

                    match maybe_value {
                        Some(value) => {
                            value.items_at_index += 1;
                        }
                        None => {
                            let value = FeatureNodeValue::new(
                                &feature.name,
                                feature.value,
                                1,
                                child_index,
                            );

                            values.push(value.clone());

                            if !self.feature_value_to_step.contains_key(&value.hash) {
                                self.feature_value_to_step.insert(value.hash, self.step);
                            }
                        }
                    }
                }
                None => {
                    let mut values = Vec::new();

                    let value =
                        FeatureNodeValue::new(&feature.name, feature.value, 1, child_index);

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

#[allow(dead_code)]
struct SortingPriorityQueue<T: Clone + Debug + Copy + Ord> {
    step: usize,
    feature_space: FeatureSpace,
    items: HashMap<u64, BinaryHeap<T>>,
}

#[allow(dead_code)]
impl<T: Clone + Debug + Copy + Ord> SortingPriorityQueue<T> {
    pub fn new(feature_space: Vec<String>) -> SortingPriorityQueue<T> {
        SortingPriorityQueue {
            step: 0,
            feature_space: FeatureSpace::new(0, feature_space),
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T, features: Vec<FeatureValue>) -> Result<(), &str> {
        if features.len() != self.feature_space.dimension {
            return Err("Invalid feature vector must have same size as feature space");
        } else {
            let feature_names_hash = create_hash(&features, false);

            if feature_names_hash != self.feature_space.feature_names_hash {
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
            });
        }
    }

    pub fn size(&self) -> usize {
        return self.feature_space.total_items;
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

    pub fn next(&mut self) -> Option<T> {
        let mut next_item: Option<T> = None;

        match self.feature_space.use_next_leaf_feature() {
            Some(next) => {
                self.items
                    .entry(next)
                    .and_modify(|leaf_items| next_item = leaf_items.pop());
            }
            None => {}
        }

        return next_item;
    }
}

#[macro_use]
extern crate lazy_static;
#[cfg(test)]
mod tests {
    use super::*;

    static LEAF_FEATURE_NAME: &str = "leaf";
    static ROOT_FEATURE_NAME: &str = "root";
    lazy_static! {
        static ref DEFAULT_FEATURES: Vec<FeatureValue> =
            vec![FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1)];
        static ref DEFAULT_FEATURE_NAMES: Vec<String> = vec![LEAF_FEATURE_NAME.to_string()];
    }

    #[test]
    fn must_be_empty_at_creation() {
        let queue = SortingPriorityQueue::<i32>::new(vec![]);
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn must_contain_added_item() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
        let expected_element: Option<&i32> = Some(&1);

        queue.add(1, DEFAULT_FEATURES.clone()).unwrap();

        assert_eq!(queue.peek(), expected_element);
    }

    #[test]
    fn peek_must_not_alter_contents() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
        let expected_element: Option<&i32> = Some(&2);

        queue.add(1, DEFAULT_FEATURES.clone()).unwrap();
        queue.add(2, DEFAULT_FEATURES.clone()).unwrap();

        assert_eq!(queue.peek(), expected_element);
        assert_eq!(queue.peek(), expected_element);
    }

    #[test]
    fn must_increase_size_when_items_are_added() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());

        queue.add(1, DEFAULT_FEATURES.clone()).unwrap();
        queue.add(1, DEFAULT_FEATURES.clone()).unwrap();

        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn must_return_next_item() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
        let next_item = 1;

        queue
            .add(next_item.clone(), DEFAULT_FEATURES.clone())
            .unwrap();

        assert_eq!(queue.next(), Some(next_item));
    }

    #[test]
    fn must_remove_next_item_after_returning() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
        let next_item = 1;

        queue
            .add(next_item.clone(), DEFAULT_FEATURES.clone())
            .unwrap();

        assert_eq!(queue.next(), Some(next_item));

        assert_eq!(queue.next(), None);
    }

    #[test]
    fn must_return_items_in_order() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
        let next_item = 2;
        let not_next_item = 1;

        queue
            .add(next_item.clone(), DEFAULT_FEATURES.clone())
            .unwrap();

        queue
            .add(not_next_item.clone(), DEFAULT_FEATURES.clone())
            .unwrap();

        assert_eq!(queue.next(), Some(next_item));
    }

    #[test]
    fn must_balance_selection_by_leaf_feature() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());

        let first_item = 2;
        let unseen_item = 1;
        let fairest_item = 3;

        queue
            .add(first_item.clone(), DEFAULT_FEATURES.clone())
            .unwrap();
        queue.add(unseen_item, DEFAULT_FEATURES.clone()).unwrap();
        queue
            .add(
                fairest_item,
                vec![FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 2)],
            )
            .unwrap();

        assert_eq!(queue.next(), Some(first_item));

        assert_eq!(queue.next(), Some(fairest_item));
    }

    #[test]
    fn must_balance_selection_by_feature_heirarchy() {
        let feature_names: Vec<String> =
            vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

        let mut queue = SortingPriorityQueue::<i32>::new(feature_names);

        let first_item = 3;
        let unseen_item = 2;
        let fairest_item = 1;

        queue
            .add(
                first_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();
        queue
            .add(
                unseen_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();
        queue
            .add(
                fairest_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();

        assert_eq!(queue.next(), Some(first_item));

        assert_eq!(queue.next(), Some(fairest_item));
    }

    #[test]
    fn should_be_drained_by_feature_heirarchy() {
        let feature_names: Vec<String> =
            vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

        let mut queue = SortingPriorityQueue::<i32>::new(feature_names);

        let first_item = 4;
        let second_last_item = 3;
        let last_item = 2;
        let fairest_item = 1;

        queue
            .add(
                first_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();
        queue
            .add(
                second_last_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();
        queue
            .add(
                last_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();
        queue
            .add(
                fairest_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();

        assert_eq!(queue.next(), Some(first_item));

        assert_eq!(queue.next(), Some(fairest_item));

        assert_eq!(queue.next(), Some(second_last_item));

        assert_eq!(queue.next(), Some(last_item));

        assert_eq!(queue.next(), None);
    }

    #[test]
    fn must_validate_features_size() {
        let mut queue = SortingPriorityQueue::<i32>::new(vec![]);

        let result = queue.add(1, DEFAULT_FEATURES.clone());

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn must_validate_features_exist_in_space() {
        let mut queue = SortingPriorityQueue::<i32>::new(vec!["Different Name".to_string()]);

        let result = queue.add(1, DEFAULT_FEATURES.clone());

        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn must_guarantee_fair_retrieval_by_feature_value_regardless_of_path() {
        let feature_names: Vec<String> =
            vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

        let mut queue = SortingPriorityQueue::<i32>::new(feature_names);

        let first_item = 3;
        let last_item = 2;
        let fairest_item = 1;

        queue
            .add(
                first_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();
        queue
            .add(
                last_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();
        queue
            .add(
                fairest_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 2),
                ],
            )
            .unwrap();

        assert_eq!(queue.next(), Some(first_item));

        assert_eq!(queue.next(), Some(fairest_item));

        assert_eq!(queue.next(), Some(last_item));
    }

    #[test]
    fn after_being_drained_must_accept_and_return_new_items() {
        let feature_names: Vec<String> =
            vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

        let mut queue = SortingPriorityQueue::<i32>::new(feature_names);

        let first_item = 4;
        let last_item = 2;

        queue
            .add(
                first_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();

        assert_eq!(queue.next(), Some(first_item));

        assert_eq!(queue.next(), None);

        queue
            .add(
                last_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();

        assert_eq!(queue.next(), Some(last_item));

        assert_eq!(queue.next(), None);
    }
}
