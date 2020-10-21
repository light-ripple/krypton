use spielplatz_macros::packet;
use bytes::{BufMut, BytesMut};
use crate::packets::{BSerializable, Packet, Uleb128, server::PacketType};
use crate::objects::Channel;

#[packet(PacketType::ChannelInfo)]
pub struct ChannelInfo {
    pub name: String,
    pub description: String,
    pub user_count: u16
}

impl From<&Channel> for ChannelInfo {
    fn from(c: &Channel) -> Self {
        Self {
            name: c.name.clone(),
            description: c.description.clone(),
            user_count: c.user_count()
        }
    }
}

impl BSerializable for ChannelInfo {
    fn serialize(&self) -> BytesMut {
        let mut out = BytesMut::with_capacity(32);
        out.put_string(&self.name);
        out.put_string(&self.description);
        out.put_u16_le(self.user_count);
        out
    }
}