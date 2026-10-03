const LAYOUT: std::alloc::Layout = std::alloc::Layout::new::<Node>();

struct Node {
    data: u32,
    next: *mut Node,
    prev: *mut Node,
}

impl Node {
    fn new(data: u32, next: *mut Node, prev: *mut Node) -> *mut Node {

        unsafe {
            let ptr = std::alloc::alloc(LAYOUT) as *mut Node;

            std::ptr::write(
                ptr,
                Node {
                    data,
                    next,
                    prev
                }
            );

            ptr
        }
    }
}

struct RDLL {
    head: *mut Node,
}

impl RDLL {
    pub fn new() -> Self {
        RDLL { head : std::ptr::null_mut() }
    }

    pub fn size(&self) -> usize {
        if self.head.is_null() { return 0; }

        let mut current = self.head;
        let mut count = 0;
        //do while
        loop {
            count += 1;
            unsafe { current = (*current).next; }

            if current == self.head {
                break;
            }
        }
        count
    }

    pub fn empty(&self) -> bool {
        self.head.is_null()
    }

    pub fn push_front(&mut self, value:u32) {
        if self.head.is_null() {
            let ptr = Node::new(value, std::ptr::null_mut(), std::ptr::null_mut());

            unsafe {
                (*ptr).next = ptr;
                (*ptr).prev = ptr;
            }

            self.head = ptr;

        } else {
            let prev_ptr = unsafe { (*self.head).prev };
            let ptr = Node::new(value, self.head, prev_ptr);

            unsafe {
                (*prev_ptr).next = ptr;
                (*self.head).prev = ptr;
            }

            self.head = ptr;
        }
    }

    pub fn push_back(&mut self, value:u32) {
        if self.head.is_null() {
            self.push_front(value);

        } else {
            unsafe {
                let old_prev_ptr = (*self.head).prev;
                let ptr = Node::new(value, self.head, old_prev_ptr);

                (*old_prev_ptr).next = ptr;
                (*self.head).prev = ptr;
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<u32> {
        if self.head.is_null() { return None }

        let return_data = unsafe { (*self.head).data };

        if unsafe { (*self.head).next == (*self.head).prev } {
            self.head = std::ptr::null_mut();

        } else {
            let old_ptr = self.head;
            
            unsafe {
                let next = (*self.head).next;
                let prev = (*self.head).prev;

                (*prev).next = next;
                (*next).prev = prev;

                self.head = next;
                std::alloc::dealloc(old_ptr as *mut u8, LAYOUT);
            }
        }
        Some(return_data)
    }

    pub fn pop_back(&mut self) -> Option<u32> {
        if self.head.is_null() { return None }

        let return_data = unsafe {
            let last_elem = (*self.head).prev;
            (*last_elem).data
        };
        if unsafe { (*self.head).next == (*self.head).prev } {
            self.head = std::ptr::null_mut();

        } else {
            unsafe {
                let old_ptr = (*self.head).prev;
                let next = self.head;
                let prev = (*old_ptr).prev;

                (*prev).next = next;
                (*next).prev = prev;

                std::alloc::dealloc(old_ptr as *mut u8, LAYOUT);
            }
        }
        Some(return_data)
    }

}

impl Drop for RDLL {
    fn drop(&mut self) {

        while !self.head.is_null() {
            self.pop_front();
        }
    }
}

impl std::fmt::Display for RDLL {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let mut current = self.head;        
        write!(f, "HEAD({:p}) -> ", self.head)?;
        unsafe {
            loop {
                write!(f, "[{}, {:p}] -> ", (*current).data, (*current).next)?;
                current = (*current).next;

                if self.head == current {
                    break;
                }
            }
        } 
        write!(f, "HEAD({:p})", self.head)
    }
}

fn main() {
    let mut rdll1 = RDLL::new();

    println!("\nThe doubly linked list is {}", if rdll1.empty() { "empty!" } else {"not empty!"});

    rdll1.push_front(1);
    rdll1.push_front(22);
    rdll1.push_front(333);
    rdll1.push_back(4444);

    println!("\nSize: {}", rdll1.size());
    println!("\n{}", rdll1);


    println!("\nPop_front: {}", rdll1.pop_front().unwrap());
    println!("Pop_back: {}", rdll1.pop_back().unwrap());

    println!("\nSize: {}", rdll1.size());
    println!("\n{}", rdll1);
    println!("\nThe doubly linked list is {}", if rdll1.empty() { "empty!" } else {"not empty!"});


}