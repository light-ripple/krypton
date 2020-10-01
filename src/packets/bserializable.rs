use bytes::{BytesMut};

pub trait BSerializable {
    fn serialize(&self) -> BytesMut;
}

pub trait BDeserializable {
    fn deserialize(data: BytesMut) -> Self;
}