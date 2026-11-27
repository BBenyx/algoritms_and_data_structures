use crate::char_buffer::CharBuffer;

enum RopeNode {
    Parent(ParentNode),
    Value(CharBuffer),
}

struct ParentNode {
    //parent: *mut ParentNode,
    left_size: usize,
    left: Box<RopeNode>,
    right: Box<RopeNode>,
}

impl ParentNode {
    pub fn new(left: RopeNode, right: RopeNode) -> Self {
        Self {
            left_size: 0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }
}

pub struct Rope {
    head: Box<RopeNode>,
    buff_cap: usize,
}

impl Rope {
    pub fn new(buffer_capacity: usize) -> Self {
        Self {
            head: Box::new(RopeNode::Value(CharBuffer::new(buffer_capacity))),
            buff_cap: buffer_capacity,
        }
    }

    pub fn print(&self) {
        let mut remaining: Vec<&RopeNode> = Vec::new();
        remaining.push(&self.head);

        while let Some(current) = remaining.pop() {
            match current {
                RopeNode::Parent(node) => {
                    remaining.push(&node.right);
                    remaining.push(&node.left);
                    continue;
                }
                RopeNode::Value(buff) => {
                    if !buff.is_empty() {
                        for char in buff[0..].iter() {
                            print!("{}", char);
                        }
                    }
                }
            }
        }
        println!();
    }

    pub fn append(&mut self, with: &str) {
        let mut chars: Vec<char> = with.chars().rev().collect();
        let current = (self.head).as_mut();

        Rope::dps_append(current, &mut chars, self.buff_cap);
    }

    fn dps_append(current: &mut RopeNode, chars: &mut Vec<char>, buff_cap: usize) {
        match current {
            RopeNode::Parent(node) => {
                Rope::dps_append(node.right.as_mut(), chars, buff_cap);
            }
            RopeNode::Value(buff) => {
                for _ in buff.len()..buff.cap() {
                    let Some(char) = chars.pop() else {
                        return;
                    };
                    buff.push(char);
                }
                if chars.is_empty() {
                    return;
                }
                // Securing the filled CharBuffer before replacing its place with a new parent
                let left = std::mem::replace(buff, CharBuffer::new(buff_cap));
                let left_size = left.len();
                let right = CharBuffer::new(buff_cap);
                let mut new_parent = ParentNode::new(RopeNode::Value(left), RopeNode::Value(right));
                new_parent.left_size = left_size;

                *current = RopeNode::Parent(new_parent);
                if let RopeNode::Parent(node) = current {
                    Rope::dps_append(node.right.as_mut(), chars, buff_cap);
                }
            }
        }
    }
}
