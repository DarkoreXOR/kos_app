#![no_std]

extern crate alloc;

pub mod str;
pub mod debug;
pub mod library;
pub mod console;

//

pub extern "C" fn initialize<F: FnOnce()>(main_fn: F) -> ! {
    kapi::memory::initialize_heap()
        .expect("cannot initialize heap");

    main_fn();

    kapi::thread::exit_thread();
}
