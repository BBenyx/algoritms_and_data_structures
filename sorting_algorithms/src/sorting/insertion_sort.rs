pub fn insertion_sort<T: PartialOrd>(list: &mut [T]) {
    if list.is_empty() { return; }

    for key in 1..list.len() {
        let mut left_next = key - 1;

        while list[left_next] > list[left_next + 1] {
            list.swap(left_next + 1, left_next);

            if left_next == 0 {
                break;      
            } else {
                left_next -= 1;
            } 
        }
    }
}