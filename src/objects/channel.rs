use crate::objects::player::Player;

#[derive(Debug)]
pub struct Channel {
	name: String,
	topic: String,
	players: Vec<Player>,
}

impl Channel {
	pub fn new(name: String, topic: String) -> Channel {
		Channel {
			name,
			topic,
			players: vec![],
		}
	}
}