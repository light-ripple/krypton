use spielplatz_macros::packet;
use bytes::{BufMut, BytesMut};
use crate::packets::{BSerializable, Packet};

#[packet(5)]
pub struct LoginReply {
    pub id: i32
}

impl BSerializable for LoginReply {
    fn serialize(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(4);
        buf.put_i32_le(self.id);
        buf
    }
}