use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;
use std::result::Result;
use std::result::Result::{Err, Ok};
mod feature_space;
use feature_space::{create_hash, FeatureSpace, FeatureValue};

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

                self.feature_space.step()
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

        return (next_item, self.feature_space.step());
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

        assert_eq!(queue.next(), (Some(next_item), 2));
    }

    #[test]
    fn must_remove_next_item_after_returning() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
        let next_item = 1;

        queue
            .add(next_item.clone(), DEFAULT_FEATURES.clone())
            .unwrap();

        assert_eq!(queue.next(), (Some(next_item), 2));

        assert_eq!(queue.next(), (None, 2));
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

        assert_eq!(queue.next(), (Some(next_item), 3));
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

        assert_eq!(queue.next(), (Some(first_item), 4));

        assert_eq!(queue.next(), (Some(fairest_item), 5));
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

        assert_eq!(queue.next(), (Some(first_item), 4));

        assert_eq!(queue.next(), (Some(fairest_item), 5));
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

        assert_eq!(queue.next(), (Some(first_item), 5));

        assert_eq!(queue.next(), (Some(fairest_item), 6));

        assert_eq!(queue.next(), (Some(second_last_item), 7));

        assert_eq!(queue.next(), (Some(last_item), 8));

        assert_eq!(queue.next(), (None, 8));
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

        assert_eq!(queue.next(), (Some(first_item), 4));

        assert_eq!(queue.next(), (Some(fairest_item), 5));

        assert_eq!(queue.next(), (Some(last_item), 6));
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

        assert_eq!(queue.next(), (Some(first_item), 2));

        assert_eq!(queue.next(), (None, 2));

        queue
            .add(
                last_item,
                vec![
                    FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                    FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
                ],
            )
            .unwrap();

        assert_eq!(queue.next(), (Some(last_item), 4));

        assert_eq!(queue.next(), (None, 4));
    }

    #[test]
    fn must_increment_step_for_each_add() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());

        let add_result = queue.add(1, DEFAULT_FEATURES.clone());

        assert_eq!(add_result, Result::Ok(1));

        let add_result = queue.add(1, DEFAULT_FEATURES.clone());

        assert_eq!(add_result, Result::Ok(2));
    }

    #[test]
    fn must_increment_step_for_each_next() {
        let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());

        let item: i32 = 1;

        let add_result = queue.add(item, DEFAULT_FEATURES.clone());

        assert_eq!(add_result, Result::Ok(1));

        assert_eq!(queue.next(), (Some(item), 2));
    }
}
