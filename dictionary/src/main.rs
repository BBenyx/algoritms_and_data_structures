struct Dictionary {
    node: Option<Box<Node>>,
    size: usize,
}

impl Dictionary {
    fn new() -> Self {
        Self {
            node: None,
            size: 0,
        }
    }

    fn insert(&mut self, key: String, val: i32) {
        let mut current = &mut self.node;

        loop {
            if current.is_none() {
                *current = Some(Box::new(Node::new(key, val)));
                self.size += 1;
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

    fn remove(&mut self, key: String) -> Option<i32> {
        if self.node.is_none() { return None; }

        let mut current: &mut Option<Box<Node>> = &mut self.node;
        loop {
            let node = current.as_ref()?;

            if node.key == key {
                if node.left.is_none() && node.right.is_none() {
                    // No child branch
                    self.size -= 1;
                    return Some(current.take().unwrap().value);

                } else if node.left.is_some() && node.right.is_some() {
                    // Two child branch
                    self.size -= 1;
                    return Self::two_child_node_remove(current);

                } else {
                    // One child branch
                    self.size -= 1;
                    return Self::one_child_node_remove(current);
                }
            } else if node.key > key {
                current = &mut current.as_mut().unwrap().left;
            } else {
                current = &mut current.as_mut().unwrap().right;
            }
        }
    }

    fn two_child_node_remove(from: &mut Option<Box<Node>>) -> Option<i32> {
        if from.is_none() { return None; }
        let current = from;

        let mut new_node = Self::pop_right_closest(current);
        let left = current.as_mut().unwrap().left.take();
        let right = current.as_mut().unwrap().right.take();
                    
        if let Some(node) = new_node.as_mut() {
            node.left = left;
            node.right = right;
        }

        let return_val = current.take().unwrap().value;
        *current = new_node;
        return Some(return_val);
    }

    fn one_child_node_remove(from: &mut Option<Box<Node>>) -> Option<i32> {
        if from.is_none() { return None; }
        let current = from;

        if current.as_mut().unwrap().left.is_some() {
            let child = current.as_mut().unwrap().left.take();
            let return_val = current.take().unwrap().value;
            *current = child;
            return Some(return_val);
        } else {
            let child = current.as_mut().unwrap().right.take();
            let return_val = current.take().unwrap().value;
            *current = child;
            return Some(return_val); 
        }
    }

    fn pop_right_closest(from: &mut Option<Box<Node>>) -> Option<Box<Node>> {

        let mut current = if
            from.is_some() &&
            from.as_ref().unwrap().right.is_some() {
                &mut from.as_mut().unwrap().right
            } else { return None; };
        
        loop {
            if current.as_ref().unwrap().left.is_some() {
                current = &mut current.as_mut().unwrap().left;
            } else {
                break;
            }
        }

        let return_node_right_child = current.as_mut().unwrap().right.take();
        let return_node = current.take();
        *current = return_node_right_child;
        
        return_node
    }

    fn read(&mut self, key: String) -> Option<&i32> {
        let mut current = &mut self.node;

        loop {
            if current.is_none() { return None; }

            let node = current.as_mut()?;

            if key < node.key {
                current = &mut node.left;
            } else if key > node.key {
                current = &mut node.right;
            } else {
                return Some(&node.value);
            }
        }
    }


    fn pre_order(&self) -> Vec<(&String, &i32)> {
        let mut ordered: Vec<(&String, &i32)> = Vec::new();
        let mut cross_section_stack: Vec<&Box<Node>> = Vec::new();

        if self.node.is_none() { return ordered }

        let mut current = self.node.as_ref().unwrap();

        while self.size != ordered.len() {
            if current.left.is_some() && current.right.is_some() {
                cross_section_stack.push(current);
            }
            ordered.push((&current.key, &current.value));

            if current.left.is_some() {
                current = current.left.as_ref().unwrap();

            } else if current.right.is_some() {
                current = current.right.as_ref().unwrap();

            } else if let Some(backtrack_node) = cross_section_stack.pop() {
                if backtrack_node.right.is_some() {
                    current = backtrack_node.right.as_ref().unwrap();
                } else {
                    current = backtrack_node;
                }
            }
        }
        ordered
    }
}


#[derive(Debug)]
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
    let mut my_dictionary = Dictionary::new();
    my_dictionary.insert("O".to_string(), 3);
    my_dictionary.insert("J".to_string(), 3);
    my_dictionary.insert("Q".to_string(), 3);
    my_dictionary.insert("E".to_string(), 3);
    my_dictionary.insert("L".to_string(), 3);
    my_dictionary.insert("P".to_string(), 3);
    my_dictionary.insert("V".to_string(), 3);
    println!("{:?}", my_dictionary.pre_order());

    my_dictionary.remove("O".to_string());
    println!("{:?}", my_dictionary.pre_order());

}

    /*my_dictionary.insert("C".to_string(), 3);
    println!("{}", my_dictionary.size);
    println!("{:?}", my_dictionary.read("Cat".to_string()));
    println!("{:?}", my_dictionary.read("Dog".to_string()));
    my_dictionary.insert("A".to_string(), 13);
    
    println!("{:?}", my_dictionary.read("Dog".to_string()));
    my_dictionary.insert("B".to_string(), 12);
    my_dictionary.insert("E".to_string(), 12);
    my_dictionary.insert("D".to_string(), 12);
    my_dictionary.insert("G".to_string(), 12);
    my_dictionary.insert("F".to_string(), 12);
    my_dictionary.insert("H".to_string(), 12);
    my_dictionary.insert("I".to_string(), 12);
    println!("{:?}", my_dictionary.read("Dog".to_string()));
    println!("{:?}", my_dictionary.read("Dog".to_string()));
    println!("{:?}", my_dictionary.read("Cat".to_string()));
    println!("{:?}", my_dictionary.remove("Cow".to_string()));*/