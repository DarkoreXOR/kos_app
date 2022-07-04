#![no_std]
#![no_main]

#![feature(panic_info_message)]
#![feature(alloc_error_handler)]

extern crate alloc;
use core::{panic::PanicInfo, alloc::{GlobalAlloc, Layout}};

use kstd::debug_println;

mod app;

pub struct NoAllocator;

unsafe impl GlobalAlloc for NoAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let address = kapi::memory::allocate_page(layout.size())
            .expect("cannot allocate memory");

        address as *mut u8
    }

    unsafe fn dealloc(&self, ptr: *mut u8, _layout: Layout) {
        kapi::memory::deallocate_page(ptr as usize)
            .expect("cannot deallocate memory");
    }
}

#[global_allocator]
static ALLOCATOR: NoAllocator = NoAllocator;

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    debug_println!("allocation error: {:?}", layout);
    kapi::thread::exit_thread();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    debug_println!("panic: {}", info);
    kapi::thread::exit_thread();
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    kstd::initialize(app::kmain);
}
