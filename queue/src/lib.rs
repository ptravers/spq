use std::collections::BinaryHeap;
use std::collections::HashMap;

struct Stats {
    : HashMap

#[allow(dead_code)]
struct SortingPriorityQueue<T: Clone + Ord> {
    stats: Stats
    items: Vec<BinaryHeap<T>>,
}

#[allow(dead_code)]
impl<T: Clone + Ord> SortingPriorityQueue<T> {
    pub fn new() -> SortingPriorityQueue<T> {
        SortingPriorityQueue { items: vec![] }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn size(&self) -> usize {
        return self.items.len();
    }

    pub fn peek(&self) -> Option<T> {
        return self.items.peek().map(|item| item.clone());
    }

    pub fn next(&mut self) -> Option<T> {
        return self.items.pop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn must_be_empty_at_creation() {
        let queue = SortingPriorityQueue::<i32>::new();
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn must_contain_added_item() {
        let mut queue = SortingPriorityQueue::<i32>::new();

        queue.add(1);

        assert_eq!(queue.peek(), Some(1));
    }

    #[test]
    fn must_increase_size_when_items_are_added() {
        let mut queue = SortingPriorityQueue::<i32>::new();

        queue.add(1);
        queue.add(1);

        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn must_return_next_item() {
        let mut queue = SortingPriorityQueue::<i32>::new();
        let next_item = 1;

        queue.add(next_item.clone());

        assert_eq!(queue.next(), Some(next_item));
    }

    #[test]
    fn must_remove_next_item_after_returning() {
        let mut queue = SortingPriorityQueue::<i32>::new();
        let next_item = 1;

        queue.add(next_item.clone());

        queue.next();

        assert_eq!(queue.next(), None);
    }

    #[test]
    fn must_return_items_in_order() {
        let mut queue = SortingPriorityQueue::<i32>::new();
        let next_item = 2;
        let not_next_item = 1;

        queue.add(next_item.clone());

        queue.add(not_next_item.clone());

        assert_eq!(queue.next(), Some(next_item));
    }

    #[test]
    fn must_update_sort_based_on_stats_changes() {
        let mut queue = SortingPriorityQueue::<i32>::new();

        let fairest_item = 3;

        assert_eq!(queue.next(), Some(fairest_item));
    }
}
