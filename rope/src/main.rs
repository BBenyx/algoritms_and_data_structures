use crate::rope::Rope;

pub mod alloc;
pub mod char_buffer;
pub mod rope;

fn main() {
    let mut my_rope = Rope::new(8);
    my_rope.append("Hello, world!");
    my_rope.print();
    my_rope.append("\nMy name is Benedek!");
    my_rope.print()
}
