use std::io::Write;
use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering::SeqCst};

static ALLOCATED: AtomicUsize = AtomicUsize::new(0);

struct Counter;

fn print(args: std::fmt::Arguments<'_>) {
    std::io::stderr().write_fmt(args).unwrap()
}

unsafe impl GlobalAlloc for Counter {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let ret = System.alloc(layout);
        if !ret.is_null() {
            ALLOCATED.fetch_add(layout.size(), SeqCst);
            print(format_args!(
                "allocated {} bytes with align {}\n",
                layout.size(),
                layout.align(),
            ));
        }
        return ret;
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        ALLOCATED.fetch_sub(layout.size(), SeqCst);
    }
}

#[global_allocator]
static A: Counter = Counter;

fn main() {
    let before_main = ALLOCATED.load(SeqCst);
    print(format_args!(
        "allocated bytes before main: {}\n",
        before_main,
    ));

    // std::io::stdout().write(b"hello").unwrap(); // -- assign 1256 bytes
    std::io::stderr().write(b"hello").unwrap(); // -- assign 0 bytes
    // std::io::stderr().write_fmt(format_args!("{}", 1)).unwrap(); // -- assign 0 bytes
    // eprintln!("hello"); // -- assign 0 bytes
    // println!("hello"); // -- assign 1256 bytes

    print(format_args!(
        "allocated bytes for the construct: {}\n",
        ALLOCATED.load(SeqCst) - before_main,
    ));
}
