pub fn shaker_sort<T: PartialOrd>(list: &mut [T]) {
    if list.is_empty() {return}

    let mut swap_flag = true;

    while swap_flag {
        swap_flag = false;
        for i in 0..list.len() - 1 {
            if list[i] > list[i + 1] {
                list.swap(i, i+1);
                swap_flag = true;
            }
        }
        for i in (0..list.len() - 1).rev() {
            if list[i] > list[i + 1] {
                list.swap(i, i+1);
                swap_flag = true;
            }
        }
    }
}