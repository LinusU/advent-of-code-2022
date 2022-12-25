use std::{collections::BinaryHeap, marker::PhantomData};

pub trait PriorityQueueItem<C> {
    fn cost(&self, context: &C) -> usize;
}

struct PriorityQueueStorage<C, T: PriorityQueueItem<C>> {
    cost: usize,
    data: T,
    marker: PhantomData<C>,
}

impl<C, T: PriorityQueueItem<C>> Ord for PriorityQueueStorage<C, T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl<C, T: PriorityQueueItem<C>> PartialOrd for PriorityQueueStorage<C, T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl<C, T: PriorityQueueItem<C>> PartialEq for PriorityQueueStorage<C, T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost.eq(&other.cost)
    }
}

impl<C, T: PriorityQueueItem<C>> Eq for PriorityQueueStorage<C, T> {}

pub struct PriorityQueue<C, T: PriorityQueueItem<C>> {
    context: C,
    heap: BinaryHeap<PriorityQueueStorage<C, T>>,
}

impl<C, T: PriorityQueueItem<C>> PriorityQueue<C, T> {
    pub fn new(context: C) -> Self {
        PriorityQueue {
            context,
            heap: BinaryHeap::new(),
        }
    }

    pub fn push(&mut self, data: T) {
        let cost = data.cost(&self.context);

        self.heap.push(PriorityQueueStorage {
            cost,
            data,
            marker: PhantomData,
        })
    }

    pub fn pop(&mut self) -> Option<(usize, T)> {
        self.heap.pop().map(|item| (item.cost, item.data))
    }
}
