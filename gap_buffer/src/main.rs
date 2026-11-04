mod alloc;

struct GapBuffer {
    cap: usize,
    prev_idx: isize,
    next_idx: usize,
    data: *mut char,
}

impl GapBuffer {
    fn resize(&mut self) {
        let new_cap = self.cap * 2;
        let new_data = alloc::alloc_char(new_cap);

        for i in 0..(self.prev_idx + 1) as usize {
            unsafe { *new_data.add(i) = *self.data.add(i); }
        }
        for i in self.next_idx..self.cap {
            unsafe { *new_data.add(i + self.cap) = *self.data.add(i); }
        }
        let right_size = self.cap - self.next_idx;
        self.next_idx = new_cap - right_size;
        self.cap = new_cap;
        self.data = new_data;
    }
}

impl Drop for GapBuffer {
    fn drop(&mut self) {
        alloc::dealloc_char(self.data, self.cap);
    }
}

fn initialize_buffer(size: usize) -> GapBuffer {
    GapBuffer {
        cap: size,
        prev_idx: -1,
        next_idx: size,
        data: alloc::alloc_char(size),
    }
}

fn debug(gb: &GapBuffer) {
    for i in 0..(gb.prev_idx + 1) as usize {
        unsafe { print!("{}", *gb.data.add(i)); }  
    }
    print!("|");
    for i in gb.next_idx..gb.cap {
        unsafe { print!("{}", *gb.data.add(i)); }  
    }
    println!();
}

fn type_char(gb: &mut GapBuffer, c: char) {
    if (gb.prev_idx + 1) as usize == gb.next_idx { gb.resize() }

    unsafe { *gb.data.add((gb.prev_idx + 1) as usize) = c; }
    gb.prev_idx += 1;
}

fn insert_text(gb: &mut GapBuffer, text: &str) {
    for c in text.chars() {
        type_char(gb, c);
    }
}

fn move_left(gb: &mut GapBuffer) {
    if gb.prev_idx < 0 { return }

    unsafe { *gb.data.add(gb.next_idx - 1) = *gb.data.add(gb.prev_idx as usize)}
    gb.next_idx -= 1;
    gb.prev_idx -= 1;
}

fn move_right(gb: &mut GapBuffer) {
    if gb.next_idx >= gb.cap { return }

    unsafe { *gb.data.add((gb.prev_idx + 1) as usize ) = *gb.data.add(gb.next_idx )}
    gb.next_idx += 1;
    gb.prev_idx += 1;
    
}

fn backspace(gb: &mut GapBuffer) {
    if gb.prev_idx < 0 { return }
    gb.prev_idx -= 1;
}

fn main() {
    let mut gb = initialize_buffer(10);
    insert_text(&mut gb, "This!");
    debug(&gb);
    move_left(&mut gb);
    debug(&gb);
    insert_text(&mut gb, " is a ni");
    debug(&gb);
    insert_text(&mut gb, "ce Rap buffer");
    debug(&gb);
    for _ in 0..9 {
        move_left(&mut gb);
        debug(&gb);
    }
    backspace(&mut gb);
    type_char(&mut gb, 'G');
    for _ in 0..10 {
        move_right(&mut gb);
        debug(&gb);
    }
    insert_text(&mut gb, " ;-) ");
    debug(&gb);
}