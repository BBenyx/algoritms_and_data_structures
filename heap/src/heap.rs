pub struct MinHeap<T> {
    pub data: Vec<T>,
}

fn parent(idx: usize) -> usize { (idx - 1) / 2 }
fn left(idx: usize) -> usize { idx*2 + 1 }
fn right(idx: usize) -> usize { idx*2 + 2 }

impl<T: Ord> MinHeap<T> {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn push(&mut self, data: T) {
        let mut new_idx = self.data.len();

        self.data.push(data);

        while new_idx != 0 && self.data[new_idx] < self.data[parent(new_idx)] {
            self.data.swap(new_idx, parent(new_idx));
            new_idx = parent(new_idx);
        }
    }

    pub fn pop_min(&mut self) -> Option<T> {
        if self.data.len() <= 1 { return self.data.pop() }

        let last_idx = self.data.len() - 1;
        self.data.swap(0, last_idx);
        let min = self.data.pop();

        self.heapify(0);

        min
    }

    pub fn heapify(&mut self, current_idx: usize) {
        let l = left(current_idx);
        let r = right(current_idx);
        let mut smallest = current_idx;

        if l < self.data.len() && self.data[l] < self.data[smallest] {
            smallest = l;
        }
        if r < self.data.len() && self.data[r] < self.data[smallest] {
            smallest = r;
        }

        if smallest != current_idx {
            self.data.swap(current_idx, smallest);

            self.heapify( smallest);
        }
    }
}