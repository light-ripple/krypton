use std::string::String;
use bytes::{BytesMut};

#[derive(Debug)]
pub struct Stats {
	pub rank: i32
}

impl Stats {
	pub fn new() -> Stats {
		Stats{
			rank: 0,
		}
	}
}

#[derive(Debug)]
pub struct Player {
	pub id: i32,
	pub username: String,
	pub token: u64,
	pub queue: BytesMut,
	pub country: u8,
	pub ingame_privileges: u8,
	pub mode: u8,
	pub stats: [Stats; 6],
}

impl Player {
	pub fn new(id: i32, username: String, token: u64) -> Player {
		Player {
			id,
			username,
			token,
			queue: BytesMut::with_capacity(1024),
			country: 0,
			ingame_privileges: 0,
			mode: 0,
			stats: [Stats::new(), Stats::new(), Stats::new(), Stats::new(), Stats::new(), Stats::new()],
			
		}
	}
}