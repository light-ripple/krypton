#[repr(u16)]
pub enum PacketType {
    LoginReply = 5,
    ChannelJoined = 64,
    ChannelInfo = 65,
}