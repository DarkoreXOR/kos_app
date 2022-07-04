use alloc::string::String;

pub struct CString {
    data: String,
}

impl CString {
    pub fn new(s: &str) -> Self {
        let mut data = String::with_capacity(s.len() + 1);

        data.push_str(s);
        data.push('\0');

        Self {
            data: data
        }
    }

    #[inline]
    pub fn as_ptr(&self) -> *const u8 {
        self.data.as_ptr()
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        let bytes = self.data.as_bytes();
        &bytes[..self.data.len()]
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        &self.data[..self.data.len()]
    }
}
