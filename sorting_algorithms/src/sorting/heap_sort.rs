pub fn heap_sort<T: Ord>(list: &mut [T]) {
    for i in (0..(list.len() / 2)).rev() {
        max_heapify(list, i, list.len());
    }
    for i in (0..(list.len() / 2)).rev() {
        list.swap(0, i);
        max_heapify(list, 0, i);
    }
    
}

fn max_heapify<T: Ord>(list: &mut [T], current_idx: usize, len: usize) {
    let l = { current_idx*2 + 1 };
    let r = { current_idx*2 + 2 };
    let mut largest = current_idx;

    if l < len && list[l] > list[largest] {
        largest = l;
    }
    if r < len && list[r] > list[largest] {
        largest = r;
    }

    if largest != current_idx {
        list.swap(current_idx, largest);

        max_heapify(list, largest, len);
    }
}