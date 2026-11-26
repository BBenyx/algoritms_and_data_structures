use crate::char_buffer::CharBuffer;

pub mod alloc;
pub mod char_buffer;

fn main() {
    let mut my_buff = CharBuffer::new(4);
    for char in ['h','e','y','!'].iter() {
        my_buff.push(*char);
    }
    println!("{:?}", my_buff[0..].iter().clone());
}
