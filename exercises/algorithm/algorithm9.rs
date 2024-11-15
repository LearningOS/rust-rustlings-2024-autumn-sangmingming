/*
	heap
	This question requires you to implement a binary heap function
*/

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
    T: Default + std::clone::Clone,
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
        self.count += 1;
        self.items.push(value);
        let i = self.count;
        self.bubble_up(i);
    }

    fn bubble_up(&mut self, idx: usize) {
        if idx > 1 {
            let parent = self.parent_idx(idx);
            if !(self.comparator)(&self.items[parent], &self.items[idx]) {
                self.items.swap(parent, idx);
                self.bubble_up(parent);
            }
        } 
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }
        let min = self.items[1].clone();
        let last = self.items.pop().unwrap();
        self.count -= 1;
        if !self.items.is_empty() {
            self.items[1] = last;
            self.bubble_down(1);
        }
        Some(min)
    } 

    fn bubble_down(&mut self, idx: usize) {
        let left = self.left_child_idx(idx);
        let right = self.right_child_idx(idx);
        let mut mid_idx = idx;
        if left < self.count && (self.comparator)(&self.items[left], &self.items[mid_idx]) {
            mid_idx = left;
        }
        if right < self.count && (self.comparator)(&self.items[right], &self.items[mid_idx]) {
            mid_idx = right;
        }
        if mid_idx != idx {
            self.items.swap(mid_idx, idx);
            self.bubble_down(mid_idx);
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

    fn smallest_child_idx(&self, _idx: usize) -> usize {
        //TODO
		1
    }
}

impl<T> Heap<T>
where
    T: Default + Ord + Copy + std::clone::Clone,
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
    T: Default + std::clone::Clone,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        self.pop()
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord + std::clone::Clone,
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