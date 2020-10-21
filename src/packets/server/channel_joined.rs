use spielplatz_macros::packet;
use bytes::BytesMut;
use crate::packets::{BSerializable, Packet, Uleb128, server::PacketType};

#[packet(PacketType::ChannelJoined)]
pub struct ChannelJoined {
    pub name: String
}

impl BSerializable for ChannelJoined {
    fn serialize(&self) -> BytesMut {
        let mut out = BytesMut::with_capacity(4);
        out.put_string(&self.name);
        out
    }
}