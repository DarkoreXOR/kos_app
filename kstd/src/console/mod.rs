use spin::Mutex;
use crate::{str::CString, library::SymbolIterator};

type InitConsole = unsafe extern "stdcall" fn(
    window_width: u32,
    window_height: u32,
    buffer_width: u32,
    buffer_height: u32,
    title: *const u8,
);

type WriteString = unsafe extern "stdcall" fn(
    string: *const u8,
    length: u32,
);

#[derive(Debug, Clone, Copy)]
pub enum ConsoleState {
    Unloaded,
    Loaded,
    Running,
    Exited,
}

pub(crate) struct SymbolTable {
    pub init_console: Option<InitConsole>,
    pub write_string: Option<WriteString>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            init_console: None,
            write_string: None,
        }
    }
}

pub(crate) struct Console {
    symbol_table: SymbolTable,
    state: ConsoleState,
    title: Option<CString>,
}

impl Console {
    fn new() -> Self {
        Self {
            symbol_table: SymbolTable::new(),
            state: ConsoleState::Unloaded,
            title: None,
        }
    }

    fn load(&mut self, library_path: &str) -> Result<ConsoleState, ()> {
        if let ConsoleState::Unloaded = self.state {
            let load_result = crate::library::load(library_path);

            if let Ok(symbol_iterator) = load_result {
                load_symbols(
                    &mut self.symbol_table,
                    symbol_iterator
                );

                self.state = ConsoleState::Loaded;

                return Ok(self.state);
            }
        }

        Err(())
    }

    fn open(&mut self, title: &str) -> Result<ConsoleState, ()> {
        if let ConsoleState::Loaded = self.state {
            if let Some(f) = self.symbol_table.init_console {
                // does console.obj copy the title string to itself or use pointer?
                // don't destroy our string (just in case)
                self.title = Some(CString::new(title));

                let title_ptr = self.title
                    .as_ref()
                    .unwrap()
                    .as_ptr();

                unsafe {
                    f(80, 25, 80, 2500, title_ptr);
                }

                self.state = ConsoleState::Running;

                return Ok(self.state);
            }
        }

        Err(())
    }

    fn write_string(&self, string: &str) -> Result<(), ()> {
        if let ConsoleState::Running = self.state {
            if let Some(f) = self.symbol_table.write_string {
                unsafe {
                    f(string.as_ptr(), string.len() as u32)
                }

                return Ok(());
            }
        }

        Err(())
    }
}

impl core::fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        match self.write_string(s) {
            Ok(_) => Ok(()),
            Err(_) => Err(core::fmt::Error)
        }
    }
}

lazy_static::lazy_static! {
    pub(crate) static ref CONSOLE: Mutex<Console> =
        Mutex::new(Console::new());
}

pub fn load() -> Result<ConsoleState, ()> {
    let path = "/sys/lib/console.obj";

    CONSOLE.lock().load(path)
}

pub fn open(title: &str) -> Result<ConsoleState, ()> {
    CONSOLE.lock().open(title)
}

pub fn _write_string(args: core::fmt::Arguments) {
    use core::fmt::Write;

    CONSOLE
        .lock()
        .write_fmt(args)
        .expect("cannot write string to console")
}

// macros

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::console::_write_string(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

// utils

fn load_symbols(table: &mut SymbolTable, symbol_iterator: SymbolIterator) {
    for (name, pointer) in symbol_iterator {
        unsafe {
            match name {
                "con_init" => {
                    table.init_console = Some(
                        core::mem::transmute::<*const (), InitConsole>(pointer)
                    );
                }

                "con_write_string" => {
                    table.write_string = Some(
                        core::mem::transmute::<*const (), WriteString>(pointer)
                    );
                }

                _ => {}
            }
        }
    }
}
