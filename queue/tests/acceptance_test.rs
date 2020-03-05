use sp_queue::feature_space::FeatureValue;
use sp_queue::SortingPriorityQueue;

#[macro_use]
extern crate lazy_static;

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
fn must_contain_enqueueed_item() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
    let expected_element: Option<&i32> = Some(&1);

    queue.enqueue(1, DEFAULT_FEATURES.clone()).unwrap();

    assert_eq!(queue.peek(), expected_element);
}

#[test]
fn peek_must_not_alter_contents() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
    let expected_element: Option<&i32> = Some(&2);

    queue.enqueue(1, DEFAULT_FEATURES.clone()).unwrap();
    queue.enqueue(2, DEFAULT_FEATURES.clone()).unwrap();

    assert_eq!(queue.peek(), expected_element);
    assert_eq!(queue.peek(), expected_element);
    assert_eq!(queue.size(), 2);
}

#[test]
fn must_decrease_size_when_items_are_removed() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());

    queue.enqueue(1, DEFAULT_FEATURES.clone()).unwrap();
    queue.enqueue(1, DEFAULT_FEATURES.clone()).unwrap();
    queue.dequeue();
    queue.dequeue();

    assert_eq!(queue.size(), 0);
}

#[test]
fn must_increase_size_when_items_are_enqueueed() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());

    queue.enqueue(1, DEFAULT_FEATURES.clone()).unwrap();
    queue.enqueue(1, DEFAULT_FEATURES.clone()).unwrap();

    assert_eq!(queue.size(), 2);
}

#[test]
fn must_return_dequeue_item() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
    let dequeue_item = 1;

    queue
        .enqueue(dequeue_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    assert_eq!(queue.dequeue(), (Some(dequeue_item), 2));
}

#[test]
fn must_remove_dequeue_item_after_returning() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
    let dequeue_item = 1;

    queue
        .enqueue(dequeue_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    assert_eq!(queue.dequeue(), (Some(dequeue_item), 2));

    assert_eq!(queue.dequeue(), (None, 2));
}

#[test]
fn must_return_items_in_order() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());
    let dequeue_item = 2;
    let not_dequeue_item = 1;

    queue
        .enqueue(dequeue_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    queue
        .enqueue(not_dequeue_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    assert_eq!(queue.dequeue(), (Some(dequeue_item), 3));
}

#[test]
fn must_balance_selection_by_leaf_feature() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());

    let first_item = 2;
    let unseen_item = 1;
    let fairest_item = 3;

    queue
        .enqueue(first_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();
    queue
        .enqueue(unseen_item, DEFAULT_FEATURES.clone())
        .unwrap();
    queue
        .enqueue(
            fairest_item,
            vec![FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 2)],
        )
        .unwrap();

    assert_eq!(queue.dequeue(), (Some(first_item), 4));

    assert_eq!(queue.dequeue(), (Some(fairest_item), 5));
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
        .enqueue(
            first_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            unseen_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            fairest_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue(), (Some(first_item), 4));

    assert_eq!(queue.dequeue(), (Some(fairest_item), 5));
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
        .enqueue(
            first_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            second_last_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            last_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            fairest_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue(), (Some(first_item), 5));

    assert_eq!(queue.dequeue(), (Some(fairest_item), 6));

    assert_eq!(queue.dequeue(), (Some(second_last_item), 7));

    assert_eq!(queue.dequeue(), (Some(last_item), 8));

    assert_eq!(queue.dequeue(), (None, 8));
}

#[test]
fn must_validate_features_size() {
    let mut queue = SortingPriorityQueue::<i32>::new(vec![]);

    let result = queue.enqueue(1, DEFAULT_FEATURES.clone());

    assert_eq!(result.is_err(), true);
}

#[test]
fn must_validate_features_exist_in_space() {
    let mut queue = SortingPriorityQueue::<i32>::new(vec!["Different Name".to_string()]);

    let result = queue.enqueue(1, DEFAULT_FEATURES.clone());

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
        .enqueue(
            first_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            last_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            fairest_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 2),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue(), (Some(first_item), 4));

    assert_eq!(queue.dequeue(), (Some(fairest_item), 5));

    assert_eq!(queue.dequeue(), (Some(last_item), 6));
}

#[test]
fn after_being_drained_must_accept_and_return_new_items() {
    let feature_names: Vec<String> =
        vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

    let mut queue = SortingPriorityQueue::<i32>::new(feature_names);

    let first_item = 4;
    let last_item = 2;

    queue
        .enqueue(
            first_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue(), (Some(first_item), 2));

    assert_eq!(queue.dequeue(), (None, 2));

    queue
        .enqueue(
            last_item,
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue(), (Some(last_item), 4));

    assert_eq!(queue.dequeue(), (None, 4));
}

#[test]
fn must_increment_step_for_each_enqueue() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());

    let enqueue_result = queue.enqueue(1, DEFAULT_FEATURES.clone());

    assert_eq!(enqueue_result, Result::Ok(1));

    let enqueue_result = queue.enqueue(1, DEFAULT_FEATURES.clone());

    assert_eq!(enqueue_result, Result::Ok(2));
}

#[test]
fn must_increment_step_for_each_dequeue() {
    let mut queue = SortingPriorityQueue::<i32>::new(DEFAULT_FEATURE_NAMES.to_vec());

    let item: i32 = 1;

    let enqueue_result = queue.enqueue(item, DEFAULT_FEATURES.clone());

    assert_eq!(enqueue_result, Result::Ok(1));

    assert_eq!(queue.dequeue(), (Some(item), 2));
}
