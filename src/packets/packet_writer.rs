use bytes::{BufMut, BytesMut};
use crate::packets::{BSerializable, Packet};

pub trait PacketWriter {
    fn write_packet<T>(&mut self, data: T) where T: BSerializable, T: Packet;
}

impl PacketWriter for BytesMut {
    fn write_packet<T>(&mut self, data: T) where T: BSerializable, T: Packet {
        self.put_u16_le(data.get_id()); // ID
        self.put_u8(0);

        let pdata = data.serialize();
        self.put_u32_le(pdata.len() as u32);
        self.put(pdata);
    }
}