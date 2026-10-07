struct Dictionary {
    node: Option<Box<Node>>,
}

impl Dictionary {
    fn new() -> Self {
        Self {
            node: None,
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let mut current = &mut self.node;

        loop {
            if current.is_none() {
                *current = Some(Box::new(Node::new(key, val)));
                return;
            }

            let node = current.as_mut().unwrap();

            if key < node.key {
                current = &mut node.left;
            } else if key > node.key {
                current = &mut node.right;
            } else {
                node.value = val;
                return;
            }
        }
    }
}


struct Node {
    key: String,
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {

    fn new(key: String, value: i32) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }
}


fn main() {
    println!("Hello, world!");
}
