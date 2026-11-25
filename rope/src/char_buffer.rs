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
            data: std::ptr::null_mut(),
        }
    }
    pub fn len(&self) -> usize {
        self.size
    }
    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn get_value(&self, at: usize) -> Option<char> {
        if at < self.size {
            unsafe { Some(*self.data.add(at)) }
        } else {
            None
        }
    }

    pub fn get_values(&self) -> Vec<char> {
        let mut vals = Vec::with_capacity(self.size);
        for i in 0..self.size {
            // TODO: maybe need clone here?
            let val = unsafe { *self.data.add(i) };
            vals.push(val);
        }
        vals
    }
    
    pub fn push(&mut self, val: char) -> Result<(), String> {
        // TODO replace string error
        if self.size == self.cap { return Err("Overflow".to_string()) }
        unsafe { *self.data.add(self.size) = val }
        self.size += 1;        
        Ok(())
        
    }

    pub fn insert(&mut self, at: usize, chars: &[char]) -> Vec<char> {
        if at > self.cap { return Vec::from(chars) }
        let mut rev_chars = Vec::new();
        if at < self.size {
            for i in (at..self.size).rev() {
                rev_chars.push(self.get_value(i).expect("Char buffer size mismatch!"));
                self.size -= 1;
            }
        }

        for char in chars.iter().rev() {
            rev_chars.push(char.clone());
        }

        for _ in 0..self.cap - self.size {
            let tmp_char = match rev_chars.pop() {
                Some(val) => val,
                None => return rev_chars,
            };
            self.push(tmp_char);
            // TODO handle this later
        }
        rev_chars.reverse();
        rev_chars
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