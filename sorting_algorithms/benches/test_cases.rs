pub fn ordered(n: usize) -> Vec<i32> {
    let mut list = Vec::new();

    for i in 0..n {
        list.push(i as i32);
    }
    list
}

pub fn reverse_ordered(n: usize) -> Vec<i32> {
    let mut list = Vec::new();

    for i in (0..n).rev() {
        list.push(i as i32);
    }
    list
}

pub fn randoms(n: usize) -> Vec<i32> {
    use rand::RngExt;

    let mut list = Vec::new();

    for i in 0..n {
        let random = rand::rng().random_range(0..3141);
        list.push(random);
    }
    list
}

/*pub fn binary_alternating(n: usize) -> Vec<i32> {
    let mut list = Vec::new();
    
    for i in 0..n {
        list.push((i % 2) as i32);
    }
    list
}*/
