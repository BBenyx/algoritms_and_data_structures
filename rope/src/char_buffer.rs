use std::ops::{Index, IndexMut, RangeFrom};
use crate::alloc;

pub struct CharBuffer {
    cap: usize,
    size: usize,
    data: *mut char,
}

impl CharBuffer {
    pub fn new(cap: usize) -> Self {
        Self {
            cap,
            size: 0,
            data: alloc::alloc_char(cap),
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn cap(&self) -> usize {
        self.cap
    }
    pub fn is_empty(&self) -> bool {
        self.size == 0
    }

    
    pub fn push(&mut self, val: char) {
        assert_ne!(self.cap, self.size, "CharBuffer overflown!");
        let pos = self.size;
        self[pos] = val;
        self.size += 1;
    }


    pub fn insert(&mut self, at: usize, chars: &[char]) -> Vec<char> {
        assert!(at <= self.size, "Insertion index is out of CharBuffer bound!");
        let mut char_stack = self.delete(at);
        char_stack.reverse();
        char_stack.extend(chars.iter().rev().copied());

        for _ in self.size..self.cap {
            let char = match char_stack.pop() {
                Some(val) => val,
                None => return char_stack,
            };
            self.push(char);
            self.size += 1;
        }
        char_stack.reverse();
        char_stack
    }

    pub fn delete(&mut self, from: usize) -> Vec<char> {
        assert!(from < self.cap, "From index in CharBuffer is out of bound!");
        let removed = self[from..].to_vec();
        self.size -= self.size - from;
        removed
    }
}

impl Index<usize> for CharBuffer {
    type Output = char;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < self.size, "CharBuffer index out of bound!");

        unsafe { &*self.data.add(index) }
    }
}

impl IndexMut<usize> for CharBuffer {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index <= self.size, "CharBuffer index out of bound!");

        unsafe { &mut *self.data.add(index) }
    }
}

impl Index<RangeFrom<usize>> for CharBuffer {
    type Output = [char];

    fn index(&self, index: RangeFrom<usize>) -> &Self::Output {
        assert!(index.start < self.size, "CharBuffer range out of bound!");
        unsafe {
            std::slice::from_raw_parts(self.data.add(index.start), self.size)
        }
    }
}

impl Default for CharBuffer {
    fn default() -> Self {
        CharBuffer::new(4)
    }
}

impl Drop for CharBuffer {
    fn drop(&mut self) {
        alloc::dealloc_char(self.data, self.cap);
    }
}