use crate::system_call::syscall_a;



pub fn exit_thread() -> ! {
    unsafe {
        // u32::MAX == unsigned 0xFFFFFFFF == signed -1
        syscall_a(u32::MAX);
    }

    unreachable!("cannot exit thread");
}
