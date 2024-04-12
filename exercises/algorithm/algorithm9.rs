/*
    heap
    This question requires you to implement a binary heap function
*/
// I AM NOT DON

use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        // 先加入尾部，然后再整理大小顺序
        // 在这里整理大小顺序，和父节点比较，如果comparator比较时value获胜，那么需要调换
        self.items.push(value);
        self.count += 1;
        let (mut val_index, mut target_index) = (self.len(), self.parent_idx(self.len()));
        while target_index >= 1
            && val_index >= 2
            && (self.comparator)(&self.items[val_index], &self.items[target_index])
        {
            // 进行调换
            self.items.swap(val_index, target_index);
            // 更新索引
            val_index = target_index;
            target_index = self.parent_idx(val_index);
        }
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        //TODO
        if self.children_present(idx) && self.right_child_idx(idx) <= self.count {
            if (self.comparator)(
                &self.items[self.left_child_idx(idx)],
                &self.items[self.right_child_idx(idx)],
            ) {
                self.left_child_idx(idx)
            } else {
                self.right_child_idx(idx)
            }
        } else if self.children_present(idx) {
            self.left_child_idx(idx)
        }else {
            0
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        // 先把堆顶和末尾交换
        let mut bottom = self.len();
        if bottom == 0 {
            return None;
        }
        self.items.swap(1usize, bottom);
        // 弹出
        let result = self.items.pop();
        self.count -= 1;
        // 进行排序整理，从根开始，把compare赢的交换
        let (mut unsort_idx, mut min_child) = (1usize, self.smallest_child_idx(1usize));
        // 如果比较出来unsort的输了，说明没排序
        while min_child != 0
            && unsort_idx < self.len()
            && (self.comparator)(&self.items[min_child], &self.items[unsort_idx])
        {
            // 交换
            self.items.swap(unsort_idx, min_child);
            // 更新索引
            unsort_idx = min_child;
            min_child = self.smallest_child_idx(unsort_idx);
        }
        result
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);

        // assert_eq!(heap.items[4] ,10);

        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
