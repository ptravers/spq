#[allow(dead_code)]
struct SortingPriorityQueue<T: Clone> {
    items: Vec<T>,
}

impl <T: Clone> SortingPriorityQueue<T> {
    pub fn new() -> SortingPriorityQueue<T> {
        SortingPriorityQueue { items: Vec::new() }
    }

    pub fn add(& mut self, item: T) {
        self.items.push(item);
    }

    pub fn size(&self) -> usize {
        return self.items.len();
    }

    pub fn peek(&self) -> Option<T> {
        return self.items.first().map(|item| item.clone());
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
}
