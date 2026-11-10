use rand::RngExt;

pub fn quick_sort<T: PartialOrd>(list: &mut [T]) {
    let pivot_idx = rand::rng().random_range(0..list.len());
    list.swap(pivot_idx, list.len() - 1);

    let pivot_idx = list.len() - 1;
    let mut border = 0;

    for i in 0..pivot_idx {
        if list[i] < list[pivot_idx] {
            list.swap(i, border);
            border += 1;
        }
    }
    list.swap(pivot_idx, border);


    if border > 1 {
        quick_sort(&mut list[..border]);
    }
    if border < list.len() - 2 {
        quick_sort(&mut list[border..])
    }
}