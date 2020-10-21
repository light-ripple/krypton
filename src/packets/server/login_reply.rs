use spielplatz_macros::packet;
use bytes::{BufMut, BytesMut};
use crate::packets::{BSerializable, Packet, server::PacketType};

#[packet(PacketType::LoginReply)]
pub struct LoginReply {
    pub id: i32
}

impl BSerializable for LoginReply {
    fn serialize(&self) -> BytesMut {
        let mut out = BytesMut::with_capacity(4);
        out.put_i32_le(self.id);
        out
    }
}