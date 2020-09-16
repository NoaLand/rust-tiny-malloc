use std::alloc::{GlobalAlloc, Layout};

struct NullAllocator;

unsafe impl GlobalAlloc for NullAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        std::ptr::null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("won't deallocate: we never allocate!");
    }
}

#[global_allocator]
static A: NullAllocator = NullAllocator;

fn main() {
}
