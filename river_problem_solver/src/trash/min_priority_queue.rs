use crate::State;

#[derive(Default)]
pub struct MinPriorityQueue {
    pub heap: Vec<(usize, State)>,
    next_idx: usize
}

impl MinPriorityQueue {
    pub fn push(&mut self, s: State) {
        self.heap.push((self.next_idx, s));
        self.next_idx += 1;
        let mut current_idx = self.heap.len() - 1;
        if current_idx == 0 {return}

        let mut parent = parent_of(current_idx);
        while current_idx != 0 && self.heap[current_idx].0 < self.heap[parent].0 {
            self.heap.swap(current_idx, parent);

            current_idx = parent;
            parent = parent_of(current_idx);
        }
    }

    pub fn pop_min(&mut self) -> Option<State> {
        if self.heap.is_empty() {
            return None;
        }

        let min = self.heap.swap_remove(0);

        if !self.heap.is_empty() {
            self.min_heapify(0);
        }

        Some(min.1)
    }
    
    pub fn len(&self) -> usize {
        self.heap.len()
    }

    fn min_heapify(&mut self, current_idx: usize) {
        let left = left_child(current_idx);
        let right = right_child(current_idx);

        let mut smallest = current_idx;
        if left < self.heap.len() && self.heap[left].0 < self.heap[smallest].0 {
            smallest = left;
        }
        if right < self.heap.len() && self.heap[right].0 < self.heap[smallest].0 {
            smallest = right;
        }

        if smallest != current_idx {
            self.heap.swap(current_idx, smallest);
            self.min_heapify(smallest);
        }
    } 
}

fn parent_of(n: usize) -> usize { (n - 1) / 2 }
fn left_child(n: usize) -> usize { n*2 + 1 }
fn right_child(n: usize) -> usize { n*2 + 1 }