mod rdll;

fn main() {
    int_test();
    println!("\n");
    string_test();
}

// Test functions

fn int_test() {
    let mut rdll1 = crate::rdll::RDLL::new();

    println!("\nThe doubly linked list is {}", if rdll1.empty() { "empty!" } else {"not empty!"});

    rdll1.push_front(1);
    rdll1.push_front(22);
    rdll1.push_front(333);
    rdll1.push_back(4444);

    println!("\nSize: {}", rdll1.size());
    println!("\n{}", rdll1);


    println!("\nPop_front: {}", rdll1.pop_front().unwrap());
    println!("Pop_back: {}", rdll1.pop_back().unwrap());

    println!("\nSize: {}", rdll1.size());
    println!("\n{}", rdll1);
    println!("\nThe doubly linked list is {}", if rdll1.empty() { "empty!" } else {"not empty!"});
}

fn string_test() {
    let mut rdll1 = crate::rdll::RDLL::new();

    println!("\nThe doubly linked list is {}", if rdll1.empty() { "empty!" } else {"not empty!"});

    rdll1.push_front("world");
    rdll1.push_front(",");
    rdll1.push_front("Hello");
    rdll1.push_back("!");

    println!("\nSize: {}", rdll1.size());
    println!("\n{}", rdll1);


    println!("\nPop_front: {}", rdll1.pop_front().unwrap());
    println!("Pop_back: {}", rdll1.pop_back().unwrap());

    println!("\nSize: {}", rdll1.size());
    println!("\n{}", rdll1);
    println!("\nThe doubly linked list is {}", if rdll1.empty() { "empty!" } else {"not empty!"});
}