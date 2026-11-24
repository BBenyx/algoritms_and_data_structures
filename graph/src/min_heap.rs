use crate::graph::VertexId;

pub type WeightToV = isize;

#[derive(Debug)]
pub struct GrayHeap {
    heap: Vec<(VertexId, WeightToV)>,
}

impl GrayHeap {
    pub fn new() -> Self {
        Self { heap: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.heap.len()
    }

    pub fn is_empty(&self) -> bool {
        self.heap.is_empty()
    }

    pub fn push(&mut self, vertex_id: VertexId, weigth_to_it: WeightToV) {
        self.heap.push((vertex_id, weigth_to_it));

        let mut current = self.heap.len() - 1;
        while current != 0 && self.heap[parent(current)].1 < self.heap[current].1 {
            self.heap.swap(current, parent(current));
            current = parent(current);
        }
    }

    pub fn pop_min(&mut self) -> Option<(VertexId, WeightToV)> {
        if self.heap.len() < 2 {
            return self.heap.pop();
        }
        let min = self.heap.swap_remove(0);

        self.heapify(0);

        Some(min)
    }

    pub fn change_weight(&mut self, vertex_id: VertexId, new_weight: WeightToV) {
        for v in &mut self.heap {
            if v.0 == vertex_id {
                v.1 = new_weight;
                break;
            }
        }
    }

    fn heapify(&mut self, idx: usize) {
        let l = left(idx);
        let r = right(idx);

        let mut smallest_idx = idx;
        let mut smallest_val = self.heap[idx].1;

        if l < self.heap.len() && self.heap[l].1 < smallest_val {
            smallest_idx = l;
            smallest_val = self.heap[l].1;
        }
        if r < self.heap.len() && self.heap[r].1 < smallest_val {
            smallest_idx = r;
        }

        if idx != smallest_idx {
            self.heapify(smallest_idx);
        }
    }
}

impl Default for GrayHeap {
    fn default() -> Self {
        Self::new()
    }
}

fn parent(idx: usize) -> usize {
    (idx - 1) / 2
}
fn left(n: usize) -> usize {
    n * 2 + 1
}
fn right(n: usize) -> usize {
    n * 2 + 2
}
