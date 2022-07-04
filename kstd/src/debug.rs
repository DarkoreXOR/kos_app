use spin::Mutex;

pub struct DebugWriter;

impl DebugWriter {
    pub fn write_byte(&mut self, byte: u8) {
        kapi::debug::print_byte(byte);
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.as_bytes() {
            self.write_byte(*byte);
        }
    }
}

impl core::fmt::Write for DebugWriter {
    fn write_str(&mut self, s: &str) -> alloc::fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static::lazy_static! {
    pub static ref DEBUG_WRITER: Mutex<DebugWriter> = 
        Mutex::new(DebugWriter);
}

pub fn _print(args: core::fmt::Arguments) {
    use core::fmt::Write;

    DEBUG_WRITER
        .lock()
        .write_fmt(args)
        .unwrap(); // always Result::Ok(...)
}

#[macro_export]
macro_rules! debug_print {
    ($($arg:tt)*) => ($crate::debug::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! debug_println {
    () => ($crate::debug_print!("\n"));
    ($($arg:tt)*) => ($crate::debug_print!("{}\n", format_args!($($arg)*)));
}
