use crate::system_call::syscall_abc_a;

pub fn load_dll(path: &str) -> Result<*const u32, ()> {
    let (eax,) = unsafe {
        syscall_abc_a(
            68, 
            19, 
            path.as_ptr() as u32
        )
    };

    match eax {
        0 => Err(()),
        result => Ok(result as *const u32),
    }
}
