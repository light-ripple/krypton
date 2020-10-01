// Kinda shitty but its to implement the macro lool
pub trait Packet {
    // Only defines the Bancho sided ID rn
    fn get_id(&self) -> u16;
}