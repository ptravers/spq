use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{Hash, Hasher};

#[derive(Debug, Clone)]
struct Feature {
    name: String,
    value: usize,
}

#[allow(dead_code)]
impl Feature {
    pub fn new(name: String, value: usize) -> Feature {
        Feature { name, value }
    }
}

impl Hash for Feature {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn create_hash(features: &Vec<Feature>, include_value: bool) -> u64 {
    let mut hasher = DefaultHasher::new();

    for feature in features {
        feature.hash(&mut hasher);
        if include_value {
            feature.value.hash(&mut hasher);
        }
    }

    return hasher.finish();
}

#[derive(Eq, Debug, Clone)]
struct FeatureNodeFeatureSpace {
    feature_value: usize,
    index: u64,
    last_used_step: usize,
}

impl Ord for FeatureNodeFeatureSpace {
    fn cmp(&self, other: &Self) -> Ordering {
        other.last_used_step.cmp(&self.last_used_step)
    }
}

impl PartialOrd for FeatureNodeFeatureSpace {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for FeatureNodeFeatureSpace {
    fn eq(&self, other: &Self) -> bool {
        self.last_used_step == other.last_used_step
    }
}
#[derive(Debug)]
struct FeatureNode {
    value_stats: BinaryHeap<FeatureNodeFeatureSpace>,
    has_leaves: bool,
}

impl FeatureNode {
    pub fn peek(&self) -> Option<&FeatureNodeFeatureSpace> {
        self.value_stats.peek()
    }

    pub fn peek_and_update(&mut self, step: usize) -> Option<FeatureNodeFeatureSpace> {
        let next_feature_node_stats = self.value_stats.pop();
        match next_feature_node_stats.clone() {
            Some(feature_node_stats) => {
                let mut updated_feature_node_stats = feature_node_stats;
                updated_feature_node_stats.last_used_step = step;
                self.value_stats.push(updated_feature_node_stats);
            }
            None => {}
        }
        next_feature_node_stats
    }
}

struct FeatureSpace {
    total_items: usize,
    step: usize,
    root_index: u64,
    feature_space_dimension: usize,
    feature_tree: HashMap<u64, FeatureNode>,
}

impl FeatureSpace {
    pub fn new(step: usize, feature_space_dimension: usize) -> FeatureSpace {
        FeatureSpace {
            total_items: 0,
            step,
            root_index: 0,
            feature_space_dimension,
            feature_tree: HashMap::new(),
        }
    }

    pub fn peek_next_leaf_feature(&self) -> Option<u64> {
        let mut next_leaf: Option<u64> = None;

        fn get_leaf(
            feature_tree: &HashMap<u64, FeatureNode>,
            feature_node: &FeatureNode,
        ) -> Option<u64> {
            match feature_node.peek() {
                Some(feature_node_stats) => {
                    if feature_node.has_leaves {
                        Some(feature_node_stats.index)
                    } else {
                        feature_tree
                            .get(&feature_node_stats.index)
                            .and_then(move |node| get_leaf(feature_tree, node))
                    }
                }
                None => None,
            }
        }

        match self.feature_tree.get(&self.root_index) {
            Some(feature_node) => next_leaf = get_leaf(&self.feature_tree, feature_node),
            None => {}
        }

        return next_leaf.map(|leaf| leaf.to_owned());
    }

    pub fn use_next_leaf_feature(&mut self) -> Option<u64> {
        let mut maybe_next_leaf_feature: Option<u64> = None;

        self.step += 1;
        let mut next_node = self.root_index;

        for _ in 0..self.feature_space_dimension {
            match self.feature_tree.get_mut(&next_node) {
                Some(ref mut feature_node) if feature_node.has_leaves => {
                    let next_feature_node_stats = feature_node.peek_and_update(self.step);
                    maybe_next_leaf_feature =
                        next_feature_node_stats.map(|node_stats| node_stats.index);
                }
                Some(feature_node) => {
                    let next_feature_node_stats = feature_node.value_stats.pop();
                    next_node = next_feature_node_stats.unwrap().index;
                }
                None => {}
            }
        }

        return maybe_next_leaf_feature;
    }

    pub fn add_item(&mut self, features: &mut Vec<Feature>, leaf_index: u64) {
        self.total_items += 1;
        self.step += 1;

        let mut previous_hash = leaf_index;
        let mut i = 1;
        let currently_empty = self.feature_tree.is_empty();

        let mut hash_to_check;

        features.reverse();

        for feature in features.clone().iter() {
            let mut next_features = features.clone();
            next_features.truncate(i);
            hash_to_check = create_hash(&next_features, false);

            match self.feature_tree.get_mut(&hash_to_check) {
                Some(FeatureNode {
                    value_stats,
                    has_leaves: _,
                }) => {
                    let value_not_present = value_stats
                        .iter()
                        .find(|&feature_stats| {
                            feature_stats.feature_value == feature.value
                                && feature_stats.index == previous_hash
                        })
                        .is_none();

                    if value_not_present {
                        value_stats.push(FeatureNodeFeatureSpace {
                            feature_value: feature.value,
                            index: previous_hash,
                            last_used_step: self.step,
                        });

                        self.step += 1;
                    }
                }
                None => {
                    let mut heap = BinaryHeap::new();

                    heap.push(FeatureNodeFeatureSpace {
                        feature_value: feature.value,
                        index: previous_hash,
                        last_used_step: self.step,
                    });

                    self.feature_tree.insert(
                        hash_to_check,
                        FeatureNode {
                            value_stats: heap,
                            has_leaves: i == 1,
                        },
                    );

                    if currently_empty && i == features.len() {
                        self.root_index = hash_to_check;
                    }

                    self.step += 1;
                }
            }

            previous_hash = hash_to_check;
            i += 1;
        }
    }
}

#[allow(dead_code)]
struct SortingPriorityQueue<T: Clone + Debug + Copy + Ord> {
    step: usize,
    stats: FeatureSpace,
    items: HashMap<u64, BinaryHeap<T>>,
}

#[allow(dead_code)]
impl<T: Clone + Debug + Copy + Ord> SortingPriorityQueue<T> {
    pub fn new(feature_space_dimension: usize) -> SortingPriorityQueue<T> {
        SortingPriorityQueue {
            step: 0,
            stats: FeatureSpace::new(0, feature_space_dimension),
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T, features: Vec<Feature>) {
        if features.len() < 1 {
            panic!("must have greater than zero features");
        }

        let hash = create_hash(&features, true);

        let mut features_copy = features.clone();

        self.items
            .entry(hash)
            .or_insert({
                self.stats.add_item(&mut features_copy, hash);
                BinaryHeap::<T>::new()
            })
            .push(item);
    }

    pub fn size(&self) -> usize {
        return self.stats.total_items;
    }

    pub fn peek(&self) -> Option<&T> {
        return self.stats.peek_next_leaf_feature().and_then(|next| {
            self.items
                .get(&next)
                .and_then(|leaf_items| leaf_items.peek())
        });
    }

    pub fn next(&mut self) -> Option<T> {
        let mut next_item: Option<T> = None;

        match self.stats.use_next_leaf_feature() {
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
    lazy_static! {
        static ref DEFAULT_FEATURES: Vec<Feature> =
            vec![Feature::new(LEAF_FEATURE_NAME.to_string(), 1)];
    }

    #[test]
    fn must_be_empty_at_creation() {
        let queue = SortingPriorityQueue::<i32>::new(0);
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn must_contain_added_item() {
        let mut queue = SortingPriorityQueue::<i32>::new(1);
        let expected_element: Option<&i32> = Some(&1);

        queue.add(1, DEFAULT_FEATURES.clone());

        assert_eq!(queue.peek(), expected_element);
    }

    #[test]
    fn peek_must_not_alter_contents() {
        let mut queue = SortingPriorityQueue::<i32>::new(1);
        let expected_element: Option<&i32> = Some(&2);

        queue.add(1, DEFAULT_FEATURES.clone());
        queue.add(2, DEFAULT_FEATURES.clone());

        assert_eq!(queue.peek(), expected_element);
        assert_eq!(queue.peek(), expected_element);
    }

    #[test]
    fn must_increase_size_when_items_are_added() {
        let mut queue = SortingPriorityQueue::<i32>::new(1);

        queue.add(1, DEFAULT_FEATURES.clone());
        queue.add(1, DEFAULT_FEATURES.clone());

        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn must_return_next_item() {
        let mut queue = SortingPriorityQueue::<i32>::new(1);
        let next_item = 1;

        queue.add(next_item.clone(), DEFAULT_FEATURES.clone());

        assert_eq!(queue.next(), Some(next_item));
    }

    #[test]
    fn must_remove_next_item_after_returning() {
        let mut queue = SortingPriorityQueue::<i32>::new(1);
        let next_item = 1;

        queue.add(next_item.clone(), DEFAULT_FEATURES.clone());

        assert_eq!(queue.next(), Some(next_item));

        assert_eq!(queue.next(), None);
    }

    #[test]
    fn must_return_items_in_order() {
        let mut queue = SortingPriorityQueue::<i32>::new(1);
        let next_item = 2;
        let not_next_item = 1;

        queue.add(next_item.clone(), DEFAULT_FEATURES.clone());

        queue.add(not_next_item.clone(), DEFAULT_FEATURES.clone());

        assert_eq!(queue.next(), Some(next_item));
    }

    #[test]
    fn must_balance_selection_by_leaf_feature() {
        let mut queue = SortingPriorityQueue::<i32>::new(1);

        let first_item = 2;
        let unseen_item = 1;
        let fairest_item = 3;

        queue.add(first_item.clone(), DEFAULT_FEATURES.clone());
        queue.add(unseen_item, DEFAULT_FEATURES.clone());
        queue.add(
            fairest_item,
            vec![Feature::new(LEAF_FEATURE_NAME.to_string(), 2)],
        );

        assert_eq!(queue.next(), Some(first_item));

        assert_eq!(queue.next(), Some(fairest_item));
    }

    #[test]
    fn must_balance_selection_by_feature_heirarchy() {
        let mut queue = SortingPriorityQueue::<i32>::new(2);

        let first_item = 3;
        let unseen_item = 2;
        let fairest_item = 1;

        let root_feature_name: String = "root".to_string();

        queue.add(
            first_item,
            vec![
                Feature::new(root_feature_name.clone(), 1),
                Feature::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        );
        queue.add(
            unseen_item,
            vec![
                Feature::new(root_feature_name.clone(), 1),
                Feature::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        );
        queue.add(
            fairest_item,
            vec![
                Feature::new(root_feature_name.to_string(), 2),
                Feature::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        );

        assert_eq!(queue.next(), Some(first_item));

        assert_eq!(queue.next(), Some(fairest_item));
    }
}
