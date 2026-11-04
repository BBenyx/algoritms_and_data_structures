use std::alloc;

pub fn alloc_char(size: usize) -> *mut char {
    let layout = alloc::Layout::array::<char>(size).unwrap();
    unsafe { alloc::alloc(layout) as *mut char }
}

pub fn dealloc_char(ptr: *mut char, size: usize) {
    let layout = alloc::Layout::array::<char>(size).unwrap();
    unsafe { alloc::dealloc(ptr as *mut u8, layout); }
}