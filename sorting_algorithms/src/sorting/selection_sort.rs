pub fn selection_sort<T: PartialOrd>(list: &mut [T]) {
    if list.is_empty() { return; }

    for i in 0..list.len() {
        let mut smallest = i;
        
        for n in i+1..list.len() {
            if list[n] < list[smallest] {
                smallest = n;
            }
        }
        for n in (i+1..=smallest).rev() {
            list.swap(n-1, n);
        }
    }
}