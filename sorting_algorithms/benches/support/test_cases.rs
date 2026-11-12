pub fn ordered(n: usize) -> Vec<usize> {
    let mut list = Vec::new();

    for i in 0..n {
        list.push(i as usize);
    }
    list
}

pub fn reverse_ordered(n: usize) -> Vec<usize> {
    let mut list = Vec::new();

    for i in (0..n).rev() {
        list.push(i as usize);
    }
    list
}

pub fn randoms(n: usize) -> Vec<usize> {
    use rand::RngExt;

    let mut list = Vec::new();

    for _ in 0..n {
        let random = rand::rng().random_range(0..300);
        list.push(random);
    }
    list
}
