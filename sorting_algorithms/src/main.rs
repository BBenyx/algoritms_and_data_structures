use sorting_algorithms::sorting::{
    quick_sort::quick_sort,
    merge_sort::merge_sort,
};

fn main() {
    let mut test_list = vec![10,7,5,3,9,23,76,12,65,1,22,5,6,534,74,12];
    let ordered_list = merge_sort(&test_list);
    println!("{:?}", &ordered_list);

    quick_sort(&mut test_list);
    println!("{:?}", &test_list);

}
