#[allow(dead_code)]
struct SortingPriorityQueue<T: Clone> {
    items: Vec<T>,
}

#[allow(dead_code)]
impl<T: Clone> SortingPriorityQueue<T> {
    pub fn new() -> SortingPriorityQueue<T> {
        SortingPriorityQueue { items: Vec::new() }
    }

    pub fn add(&mut self, item: T) {
        self.items.push(item);
    }

    pub fn size(&self) -> usize {
        return self.items.len();
    }

    pub fn peek(&self) -> Option<T> {
        return self.items.first().map(|item| item.clone());
    }

    pub fn next(&mut self) -> Option<T> {
        return self.items.pop();
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn must_be_empty_at_creation() {
        let queue: SortingPriorityQueue<i32> = SortingPriorityQueue::new();
        assert_eq!(queue.size(), 0);
    }

    #[test]
    fn must_contain_added_item() {
        let mut queue: SortingPriorityQueue<i32> = SortingPriorityQueue::new();

        queue.add(1);

        assert_eq!(queue.peek(), Some(1));
    }

    #[test]
    fn must_increase_size_when_items_are_added() {
        let mut queue: SortingPriorityQueue<i32> = SortingPriorityQueue::new();

        queue.add(1);
        queue.add(1);

        assert_eq!(queue.size(), 2);
    }

    #[test]
    fn must_return_next_item() {
        let mut queue: SortingPriorityQueue<String> = SortingPriorityQueue::new();
        let next_item = String::from("I'm next!");

        queue.add(next_item.clone());

        assert_eq!(queue.next(), Some(next_item));
    }

    #[test]
    fn must_remove_next_item_after_returning() {
        let mut queue: SortingPriorityQueue<String> = SortingPriorityQueue::new();
        let next_item = String::from("I'm next!");

        queue.add(next_item.clone());

        queue.next();

        assert_eq!(queue.next(), None);
    }
}
