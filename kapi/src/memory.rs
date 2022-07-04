use crate::system_call::{syscall_ab_a, syscall_abc_a};

pub fn initialize_heap() -> Result<u32, ()> {
    unsafe {
        match syscall_ab_a(68, 11) {
            (0,) => Err(()),
            (eax,) => Ok(eax),
        }
    }
}

pub fn allocate_page(size: usize) -> Result<u32, ()> {
    unsafe {
        match syscall_abc_a(68, 12, size as u32) {
            (0,) => Err(()),
            (eax,) => Ok(eax),
        }
    }
}

pub fn deallocate_page(address: usize) -> Result<(), ()> {
    unsafe {
        match syscall_abc_a(68, 13, address as u32) {
            (0,) => Err(()),
            (_,) => Ok(()),
        }
    }
}
