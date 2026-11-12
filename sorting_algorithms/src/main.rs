use sorting_algorithms::sorting::{bucket_sort::bucket_sort, counting_sort::counting_sort};

fn main() {
    let test_list = vec![10, 7, 5, 3, 9, 10, 2, 9, 5, 1, 6, 5, 6, 4, 4, 0];
    let ordered_list = counting_sort(&test_list);
    println!("{:?}", ordered_list);

    let new_ordered_list = bucket_sort(&test_list);
    println!("{:?}", new_ordered_list);
}
