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
    let queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();
    assert_eq!(queue.size().unwrap(), 0);
}

#[test]
fn must_contain_enqueued_item() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();
    let expected_element: Option<Vec<u8>> = Some(vec![1]);

    queue.enqueue(vec![1], DEFAULT_FEATURES.clone()).unwrap();

    assert_eq!(queue.peek().unwrap(), expected_element);
}

#[test]
fn must_increment_the_epoch_for_each_state_change() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();

    queue.enqueue(vec![1], DEFAULT_FEATURES.clone()).unwrap();

    assert_eq!(queue.get_epoch().unwrap(), 1);

    queue.dequeue().unwrap();

    assert_eq!(queue.get_epoch().unwrap(), 2);
}

#[test]
fn peek_must_not_alter_contents() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();
    let expected_element: Option<Vec<u8>> = Some(vec![1]);

    queue.enqueue(vec![1], DEFAULT_FEATURES.clone()).unwrap();
    queue.enqueue(vec![2], DEFAULT_FEATURES.clone()).unwrap();

    assert_eq!(queue.peek().unwrap(), expected_element);
    assert_eq!(queue.peek().unwrap(), expected_element);
    assert_eq!(queue.size().unwrap(), 2);
}

#[test]
fn must_decrease_size_when_items_are_removed() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();

    queue.enqueue(vec![1], DEFAULT_FEATURES.clone()).unwrap();
    queue.enqueue(vec![1], DEFAULT_FEATURES.clone()).unwrap();
    queue.dequeue().unwrap();
    queue.dequeue().unwrap();

    assert_eq!(queue.size().unwrap(), 0);
}

#[test]
fn must_increase_size_when_items_are_enqueueed() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();

    queue.enqueue(vec![1], DEFAULT_FEATURES.clone()).unwrap();
    queue.enqueue(vec![1], DEFAULT_FEATURES.clone()).unwrap();

    assert_eq!(queue.size().unwrap(), 2);
}

#[test]
fn must_return_dequeue_item() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();
    let dequeue_item = vec![1];

    queue
        .enqueue(dequeue_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(dequeue_item), 2));
}

#[test]
fn must_remove_dequeue_item_after_returning() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();
    let dequeue_item = vec![1];

    queue
        .enqueue(dequeue_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(dequeue_item), 2));

    assert_eq!(queue.dequeue().unwrap(), (None, 2));
}

#[test]
fn must_return_items_in_order() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();
    let dequeue_item = vec![2];
    let not_dequeue_item = vec![1];

    queue
        .enqueue(dequeue_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    queue
        .enqueue(not_dequeue_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(dequeue_item), 3));
}

#[test]
fn must_balance_selection_by_leaf_feature() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();

    let first_item: Vec<u8> = vec![2];
    let unseen_item: Vec<u8> = vec![1];
    let fairest_item: Vec<u8> = vec![3];

    queue
        .enqueue(first_item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();
    queue
        .enqueue(unseen_item, DEFAULT_FEATURES.clone())
        .unwrap();
    queue
        .enqueue(
            fairest_item.clone(),
            vec![FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 2)],
        )
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(first_item), 4));

    assert_eq!(queue.dequeue().unwrap(), (Some(fairest_item), 5));
}

#[test]
fn must_balance_selection_by_feature_heirarchy() {
    let feature_names: Vec<String> =
        vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

    let mut queue = SortingPriorityQueue::new(feature_names).unwrap();

    let first_item: Vec<u8> = vec![2];
    let unseen_item: Vec<u8> = vec![1];
    let fairest_item: Vec<u8> = vec![3];

    queue
        .enqueue(
            first_item.clone(),
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
            fairest_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(first_item), 4));

    assert_eq!(queue.dequeue().unwrap(), (Some(fairest_item), 5));
}

#[test]
fn should_be_drained_by_feature_heirarchy() {
    let feature_names: Vec<String> =
        vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

    let mut queue = SortingPriorityQueue::new(feature_names).unwrap();

    let first_item: Vec<u8> = vec![4];
    let second_last_item: Vec<u8> = vec![3];
    let last_item: Vec<u8> = vec![2];
    let fairest_item: Vec<u8> = vec![1];

    queue
        .enqueue(
            first_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            second_last_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            last_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            fairest_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(first_item), 5));

    assert_eq!(queue.dequeue().unwrap(), (Some(fairest_item), 6));

    assert_eq!(queue.dequeue().unwrap(), (Some(second_last_item), 7));

    assert_eq!(queue.dequeue().unwrap(), (Some(last_item), 8));

    assert_eq!(queue.dequeue().unwrap(), (None, 8));
}

#[test]
fn must_validate_features_size() {
    let mut queue = SortingPriorityQueue::new(vec![]).unwrap();

    let result = queue.enqueue(vec![1], DEFAULT_FEATURES.clone());

    assert_eq!(result.is_err(), true);
}

#[test]
fn must_validate_features_exist_in_space() {
    let mut queue = SortingPriorityQueue::new(vec!["Different Name".to_string()]).unwrap();

    let result = queue.enqueue(vec![1], DEFAULT_FEATURES.clone());

    assert_eq!(result.is_err(), true);
}

#[test]
fn must_guarantee_fair_retrieval_by_feature_value_regardless_of_path() {
    let feature_names: Vec<String> =
        vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

    let mut queue = SortingPriorityQueue::new(feature_names).unwrap();

    let first_item = vec![3];
    let last_item = vec![2];
    let fairest_item = vec![1];

    queue
        .enqueue(
            first_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            last_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            fairest_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 2),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(first_item), 4));

    assert_eq!(queue.dequeue().unwrap(), (Some(fairest_item), 5));

    assert_eq!(queue.dequeue().unwrap(), (Some(last_item), 6));
}

#[test]
fn must_guarantee_fair_retrieval_after_items_have_been_removed() {
    let feature_names: Vec<String> =
        vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

    let mut queue = SortingPriorityQueue::new(feature_names).unwrap();

    let first_item = vec![3];
    let last_item = vec![2];
    let fairest_item = vec![1];

    queue
        .enqueue(
            first_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    assert_eq!(queue.dequeue().unwrap(), (Some(first_item), 2));

    queue
        .enqueue(
            last_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            fairest_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 2),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(fairest_item), 5));
    assert_eq!(queue.dequeue().unwrap(), (Some(last_item), 6));
}

#[test]
fn after_being_drained_must_accept_and_return_new_items() {
    let feature_names: Vec<String> =
        vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

    let mut queue = SortingPriorityQueue::new(feature_names).unwrap();

    let first_item = vec![4];
    let last_item = vec![2];

    queue
        .enqueue(
            first_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(first_item), 2));

    assert_eq!(queue.dequeue().unwrap(), (None, 2));

    queue
        .enqueue(
            last_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(last_item), 4));

    assert_eq!(queue.dequeue().unwrap(), (None, 4));
}

#[test]
fn must_increment_step_for_each_enqueue() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();

    let enqueue_result = queue.enqueue(vec![1], DEFAULT_FEATURES.clone());

    assert_eq!(enqueue_result, Result::Ok(1));

    let enqueue_result = queue.enqueue(vec![1], DEFAULT_FEATURES.clone());

    assert_eq!(enqueue_result, Result::Ok(2));
}

#[test]
fn must_increment_step_for_each_dequeue() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();

    let item: Vec<u8> = vec![1];

    let enqueue_result = queue.enqueue(item.clone(), DEFAULT_FEATURES.clone());

    assert_eq!(enqueue_result, Result::Ok(1));

    assert_eq!(queue.dequeue().unwrap(), (Some(item), 2));
}

#[test]
fn must_maintain_epoch_between_instances_when_durable() {
    let directory = "/tmp/durable".to_string();
    std::fs::create_dir_all(directory.clone()).unwrap();

    let mut queue =
        SortingPriorityQueue::new_durable(DEFAULT_FEATURE_NAMES.to_vec(), directory.clone())
            .unwrap();

    let item: Vec<u8> = vec![1];

    queue
        .enqueue(item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    drop(queue);

    let queue =
        SortingPriorityQueue::new_durable(DEFAULT_FEATURE_NAMES.to_vec(), directory.clone())
            .unwrap();

    let epoch = queue.get_epoch().unwrap();

    match std::fs::remove_dir_all(directory.clone()) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }

    assert_eq!(epoch, 1);
}

#[test]
fn must_not_maintain_epoch_between_instances_when_not_durable() {
    let mut queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();

    let item: Vec<u8> = vec![1];

    queue
        .enqueue(item.clone(), DEFAULT_FEATURES.clone())
        .unwrap();

    drop(queue);

    let queue = SortingPriorityQueue::new(DEFAULT_FEATURE_NAMES.to_vec()).unwrap();

    assert_eq!(queue.get_epoch(), Ok(0));
}

#[test]
fn must_maintain_feature_space_between_instances_when_durable_and_queue_empty() {
    let directory = "/tmp/durable2".to_string();

    match std::fs::remove_dir_all(directory.clone()) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
    std::fs::create_dir_all(directory.clone()).unwrap();

    let feature_names: Vec<String> =
        vec![ROOT_FEATURE_NAME.to_string(), LEAF_FEATURE_NAME.to_string()];

    let mut queue =
        SortingPriorityQueue::new_durable(feature_names.clone(), directory.clone()).unwrap();

    let first_item: Vec<u8> = vec![4];
    let last_item: Vec<u8> = vec![2];
    let fairest_item: Vec<u8> = vec![1];

    queue
        .enqueue(
            first_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();

    assert_eq!(queue.dequeue().unwrap(), (Some(first_item), 2));

    drop(queue);

    let mut queue = SortingPriorityQueue::new_durable(feature_names, directory.clone()).unwrap();

    queue
        .enqueue(
            last_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 1),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    queue
        .enqueue(
            fairest_item.clone(),
            vec![
                FeatureValue::new(ROOT_FEATURE_NAME.to_string(), 2),
                FeatureValue::new(LEAF_FEATURE_NAME.to_string(), 1),
            ],
        )
        .unwrap();
    assert_eq!(queue.dequeue().unwrap(), (Some(fairest_item), 5));

    assert_eq!(queue.dequeue().unwrap(), (Some(last_item), 6));

    assert_eq!(queue.dequeue().unwrap(), (None, 6));

    match std::fs::remove_dir_all(directory.clone()) {
        Ok(_) => (),
        Err(e) => println!("{:?}", e),
    }
}
