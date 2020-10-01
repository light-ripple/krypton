use bytes::{BufMut, BytesMut};

pub trait Uleb128 {
    fn put_uleb128(&mut self, value: usize);
    fn put_str(&mut self, value: &str);
    fn put_string(&mut self, value: String);
}

impl Uleb128 for BytesMut {
    fn put_uleb128(&mut self, mut value: usize) {
        while value >= 0x80 {
            self.put_u8((value | 0x80) as u8);
            value >>= 7;
        }

        self.put_u8(value as u8);
    }

    fn put_str(&mut self, value: &str) {
        self.put_string(String::from(value));
    }

    fn put_string(&mut self, value: String) {
        if value.len() == 0 {
            self.put_u8(0);
            return;
        }

        self.put_u8(11);

        self.put_uleb128(value.len());
        self.put(value.as_bytes());
    }
}