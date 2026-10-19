fn main() {
    let mut test_list1 = vec![5,2,8,1,6,32,65,2,6,8,3,2,5,7];
    let mut test_list2 = vec!['b','g','l','q','a','m','v','y','i','l'];
    let mut test_list3 = vec![5,2,8,1,6,32,65,2,6,8,3,2,5,7];
    let mut test_list4 = vec!['b','g','l','q','a','m','v','y','i','l'];

    println!("Original: {:?}", &test_list1);
    bubble_sort(&mut test_list1);
    println!("Sorted: {:?}", &test_list1);

    println!("Original: {:?}", &test_list2);
    shaker_sort(&mut test_list2);
    println!("Sorted: {:?}", &test_list2);

    println!("Original: {:?}", &test_list3);
    insertion_sort(&mut test_list3);
    println!("Sorted: {:?}", &test_list3);

    println!("Original: {:?}", &test_list4);
    selection_sort(&mut test_list4);
    println!("Sorted: {:?}", &test_list4);
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

fn shaker_sort<T: PartialOrd>(list: &mut [T]) {
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

fn insertion_sort<T: PartialOrd>(list: &mut [T]) {
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

fn selection_sort<T: PartialOrd>(list: &mut [T]) {
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