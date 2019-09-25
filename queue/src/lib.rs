use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::fmt::Debug;

struct FeatureStats {
    value_to_last_returned: HashMap<usize, usize>,
}

#[derive(Debug, Clone)]
struct Feature {
    value: usize,
}

struct Stats {
    total_items: usize,
    features: FeatureStats,
    step: usize,
}

impl Stats {
    pub fn new(step: usize, features: Vec<Feature>) -> Stats {
        let mut feature_stats: FeatureStats =
                    FeatureStats {
                        value_to_last_returned: HashMap::<usize, usize>::new()
                    };
        for feature in features {
            feature_stats
                .value_to_last_returned
                .insert(feature.value, step);
        }
        Stats { total_items: 0, features: feature_stats, step }
    }

    pub fn peek_next_leaf_feature(&self) -> Option<usize> {
        let mut oldest_step: &usize = &0;
        let mut next_leaf: Option<&usize> = None;

        for (feature_value, last_used_step) in self.features.value_to_last_returned.iter() {
            println!("feature_value {:?} last_used_step {:?}", feature_value, last_used_step);
            if *oldest_step == 0 ||*last_used_step < *oldest_step {
                next_leaf = Some(feature_value);
                oldest_step = last_used_step;
            }
        }

        return next_leaf.map(|leaf| leaf.to_owned());
    }

    pub fn use_next_leaf_feature(&mut self) -> Option<usize> {
        let maybe_next_leaf_feature = self.peek_next_leaf_feature();

        match maybe_next_leaf_feature {
            Some(next_leaf_feature) => {
                self.step += 1;
                self.features
                    .value_to_last_returned
                    .insert(next_leaf_feature, self.step);
            }
            None => {}
        }

        return maybe_next_leaf_feature;
    }


    pub fn add_item(&mut self, features: Vec<Feature>) {
        self.total_items += 1;
        self.step += 1;
        for feature in features {
            self.features
                .value_to_last_returned
                .insert(feature.value, self.step);
        }
    }
}

#[allow(dead_code)]
struct SortingPriorityQueue<T: Clone + Debug + Copy + Ord> {
    step: usize,
    stats: Stats,
    items: HashMap<usize, BinaryHeap<T>>,
}

#[allow(dead_code)]
impl<T: Clone + Debug + Copy + Ord> SortingPriorityQueue<T> {
    pub fn new() -> SortingPriorityQueue<T> {
        SortingPriorityQueue {
            step: 0,
            stats: Stats::new(0, vec![]),
            items: HashMap::new(),
        }
    }

    pub fn add(&mut self, item: T, features: Vec<Feature>) {
        self.items
            .entry(features.first().unwrap().value)
            .or_insert(BinaryHeap::<T>::new())
            .push(item);
        self.stats.add_item(features);
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
            },
            None => {}
        }

        return next_item
    }
}

#[macro_use] extern crate lazy_static;
#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref DEFAULT_FEATURES: Vec<Feature> = vec![Feature { value: 1 }];
    }

    #[test]
    fn must_be_empty_at_creation() {
        let queue = SortingPriorityQueue::<i32>::new();
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn must_contain_added_item() {
        let mut queue = SortingPriorityQueue::<i32>::new();
        let expected_element: Option<&i32> = Some(&1);

        queue.add(1, DEFAULT_FEATURES.clone());

        assert_eq!(queue.peek(), expected_element);
    }

    #[test]
    fn must_increase_size_when_items_are_added() {
        let mut queue = SortingPriorityQueue::<i32>::new();

        queue.add(1, DEFAULT_FEATURES.clone());
        queue.add(1, DEFAULT_FEATURES.clone());

        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn must_return_next_item() {
        let mut queue = SortingPriorityQueue::<i32>::new();
        let next_item = 1;
        ;

        queue.add(next_item.clone(), DEFAULT_FEATURES.clone());

        assert_eq!(queue.next(), Some(next_item));
    }

    #[test]
    fn must_remove_next_item_after_returning() {
        let mut queue = SortingPriorityQueue::<i32>::new();
        let next_item = 1;

        queue.add(next_item.clone(), DEFAULT_FEATURES.clone());

        assert_eq!(queue.next(), Some(next_item));

        assert_eq!(queue.next(), None);
    }

    #[test]
    fn must_return_items_in_order() {
        let mut queue = SortingPriorityQueue::<i32>::new();
        let next_item = 2;
        let not_next_item = 1;

        queue.add(next_item.clone(), DEFAULT_FEATURES.clone());

        queue.add(not_next_item.clone(), DEFAULT_FEATURES.clone());

        assert_eq!(queue.next(), Some(next_item));
    }

    #[test]
    fn must_balance_selection_by_leaf_feature() {
        let mut queue = SortingPriorityQueue::<i32>::new();

        let first_item = 2;
        let unseen_item = 1;
        let fairest_item = 3;

        queue.add(first_item.clone(), DEFAULT_FEATURES.clone());
        queue.add(unseen_item, DEFAULT_FEATURES.clone());
        queue.add(fairest_item, vec![Feature { value: 2 }]);

        assert_eq!(queue.next(), Some(first_item));

        assert_eq!(queue.next(), Some(fairest_item));
    }
}
