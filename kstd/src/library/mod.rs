use crate::str::CString;

pub enum LibraryState {
    Unloaded,
    Loaded,
}

pub struct SymbolIterator<'a> {
    export_table_pointer: *const u32,
    offset: usize,
    _phantom: core::marker::PhantomData<&'a ()>,
}

impl<'a> SymbolIterator<'a> {
    unsafe fn parse_symbol_name(&mut self) -> Result<&'a str, ()> {
        let pointer = self
            .export_table_pointer
            .add(self.offset);

        let symbol_name_pointer = 
            pointer.read() as *const u8;

        if symbol_name_pointer.is_null() {
            return Err(());
        }

        Self::read_symbol_name(
            symbol_name_pointer
        )
    }

    unsafe fn parse_symbol_pointer(&mut self) -> Result<*const (), ()> {
        let symbol_pointer = self
            .export_table_pointer
            .add(self.offset);

        // It must be impossible to happen
        if symbol_pointer.is_null() {
            return Err(());
        }

        let result = symbol_pointer.read();

        if result <= 0 {
            return Err(());
        }

        Ok(result as *const ())
    }

    unsafe fn read_symbol_name(name_pointer: *const u8) -> Result<&'a str, ()> {
        let mut offset = 0;

        loop {
            let byte = name_pointer
                .add(offset)
                .read_unaligned();
    
            if byte == 0 {
                break;
            }
    
            offset += 1
        };
    
        let bytes = core::slice::from_raw_parts(
            name_pointer,
            offset
        );
    
        match core::str::from_utf8(bytes) {
            Ok(s) => Ok(s),
            Err(_) => Err(()),
        }
    }
}

impl<'a> Iterator for SymbolIterator<'a> {
    type Item = (&'a str, *const ());

    fn next(&mut self) -> Option<Self::Item> {

        // parse symbol name

        let symbol_name_result = unsafe {
            self.parse_symbol_name()
        };

        if symbol_name_result.is_err() {
            return None;
        }
        
        self.offset += 1;

        // parse symbol pointer

        let symbol_pointer_result = unsafe {
            self.parse_symbol_pointer()
        };

        if symbol_pointer_result.is_err() {
            return None;
        }

        self.offset += 1;

        // result

        let symbol_name = symbol_name_result.unwrap();
        let symbol_pointer = symbol_pointer_result.unwrap();

        Some((symbol_name, symbol_pointer))
    }
}

pub fn load(path: &str) -> Result<SymbolIterator<'static>, ()> {
    let path_cstr = CString::new(path);

    let export_table_pointer = 
        kapi::filesystem::load_dll(path_cstr.as_str())?;

    Ok(SymbolIterator {
        export_table_pointer,
        offset: 0,
        _phantom: core::marker::PhantomData {},
    })
}
