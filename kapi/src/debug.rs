use crate::system_call::syscall_abc;

pub fn print_byte(c: u8) {
    unsafe {
        syscall_abc(63, 1, c as u32);
    }
}
