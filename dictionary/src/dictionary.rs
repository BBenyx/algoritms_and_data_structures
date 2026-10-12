pub struct Dictionary<K: PartialOrd,V> {
    node: Option<Box<Node<K,V>>>,
    pub size: usize,
}

impl<K: PartialOrd,V> Dictionary<K,V> {
    pub fn new() -> Self {
        Self {
            node: None,
            size: 0, //TODO: Make size function
        }
    }

    pub fn insert(&mut self, key: K, val: V) {
        let mut current = &mut self.node;
        loop {
            match current {
                None => {
                    *current = Some(Box::new(Node::new(key, val)));
                    self.size += 1;
                    return;
                },
                Some(node) => {
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
    }

    pub fn remove(&mut self, key: K) -> Option<V> {

        let mut current = &mut self.node;
        loop {
            match current {
                None => return None,
                Some(node) => {
                    if node.key == key {

                        if node.left.is_none() && node.right.is_none() {
                            self.size -= 1;
                            return Some(current.take().unwrap().value);

                        } else if node.left.is_some() && node.right.is_some() {
                            self.size -= 1;
                            return Self::two_child_node_remove(current);

                        } else {
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
        }
    }

    pub fn read(&mut self, key: K) -> Option<&V> {
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

    /* ---------------------------------- */
    /* Ordering functions --------------- */

    pub fn pre_order(&self) -> Vec<(&K, &V)> {
        let mut ordered: Vec<(&K, &V)> = Vec::new();
        if self.node.is_none() { return ordered }

        let mut cross_section: Vec<&Box<Node<K,V>>> = Vec::new();
        let mut current = self.node.as_ref().unwrap();

        while self.size > ordered.len() {
            if current.left.is_some() && current.right.is_some() {
                cross_section.push(current);
            }
            ordered.push((&current.key, &current.value));

            if current.left.is_some() {
                current = current.left.as_ref().unwrap();

            } else if current.right.is_some() {
                current = current.right.as_ref().unwrap();

            } else if let Some(backtrack_node) = cross_section.pop() {
                current = backtrack_node.right.as_ref().unwrap();
            }
        }
        ordered
    }

    /* ---------------------------------- */
    /* Helper functions from remove ----- */

    fn two_child_node_remove(from: &mut Option<Box<Node<K,V>>>) -> Option<V> {
        let node = from.as_ref()?;
        if node.left.is_none() || node.right.is_none() { return None; }

        let mut new_node = Self::pop_right_closest(from)?;
        let mut old_node = from.take()?;

        new_node.left = old_node.left.take();
        new_node.right = old_node.right.take();

        *from = Some(new_node);
        Some(old_node.value)
    }

    fn one_child_node_remove(from: &mut Option<Box<Node<K,V>>>) -> Option<V> {
        let node = from.as_mut()?;
        if node.left.is_none() && node.right.is_none() { return None; }

        if node.left.is_some() && node.right.is_none() {
            let child = node.left.take();
            let return_val = from.take()?.value;
            *from = child;

            return Some(return_val);

        } else if node.left.is_none() && node.right.is_some() {
            let child = node.right.take();
            let return_val = from.take()?.value;
            *from = child;

            return Some(return_val); 
        }

        None
    }

    fn pop_right_closest(from: &mut Option<Box<Node<K,V>>>) -> Option<Box<Node<K,V>>> {
        let mut current = &mut from.as_mut()?.right;
        loop {
            if current.as_ref()?.left.is_some() {
                current = &mut current.as_mut()?.left;
            } else {
                let mut closest = current.take()?;
                *current = closest.right.take();

                return Some(closest);
            }
        }
    }

}


#[derive(Debug)]
struct Node<K: PartialOrd, V> {
    key: K,
    value: V,
    left: Option<Box<Node<K,V>>>,
    right: Option<Box<Node<K,V>>>,
}

impl<K: PartialOrd, V> Node<K, V> {

    fn new(key: K, value: V) -> Self {
        Self {
            key,
            value,
            left: None,
            right: None,
        }
    }
}