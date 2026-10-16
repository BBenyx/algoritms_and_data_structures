use std::vec;

fn main() {
    let mut test_list1 = vec![5,2,8,1,6,32,65,2,6,8,3,2,5,7];
    let mut test_list2 = vec!['b','g','l','q','a','m','v','y','i','l'];

    println!("Original: {:?}", &test_list1);
    bubble_sort(&mut test_list1);
    println!("Sorted: {:?}", &test_list1);
    println!("Original: {:?}", &test_list2);
    bubble_sort(&mut test_list2);
    println!("Sorted: {:?}", &test_list2);
}

fn bubble_sort<T: PartialOrd>(list: &mut [T]) {
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
    }
}