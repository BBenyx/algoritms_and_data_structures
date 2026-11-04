struct GapBuffer {
    // TODO: add the fields needed to represent the buffer and its gap
}

fn initialize_buffer(size: usize) -> GapBuffer {
    // TODO
    todo!()
}

fn debug(gb: &GapBuffer) {
    // TODO
}

fn type_char(gb: &mut GapBuffer, c: char) {
    // TODO - must be O(1) amortized
}

fn insert_text(gb: &mut GapBuffer, text: &str) {
    for c in text.chars() {
        type_char(gb, c);
    }
}

fn move_left(gb: &mut GapBuffer) {
    // TODO - must be O(1)
}

fn move_right(gb: &mut GapBuffer) {
    // TODO - must be O(1)
}

fn backspace(gb: &mut GapBuffer) {
    // TODO - must be O(1)
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