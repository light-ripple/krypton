use crate::objects::player::Player as player;

use bytes::{BytesMut, BufMut};

pub struct Packet {
	id: u16,
	buffer: BytesMut,
}

impl Packet {
	pub fn message_packet(msg: String, target: String, sender: &player) -> Packet {
		let mut p = Packet {
			id: 7,
			buffer: BytesMut::with_capacity(512),
		};
		
		p.buffer.put_str(format!("{}!", sender.username));
		p.buffer.put_str(msg);
		p.buffer.put_str(target);
		p.buffer.put_i32_le(sender.id);
		
		p
	}

	pub fn presence_packet(p: &player) -> Packet {
		let mut pc = Packet {
			id: 83,
			buffer: BytesMut::with_capacity(64),
		};
		
		pc.buffer.put_i32(p.id);
		pc.buffer.put_str(format!("{}!", p.username));
		pc.buffer.put_u8(24); 
		pc.buffer.put_u8(p.country);
		pc.buffer.put_u8(((p.ingame_privileges & 0x1f) | ((p.mode & 0x7) << 5)) as u8);
		pc.buffer.put_f32(0.0);
		pc.buffer.put_f32(0.0);
		pc.buffer.put_i32(p.stats[p.mode as usize].rank);
		
		pc
	}
	
	pub fn stats_packet(p :&player) -> Packet {
		let mut pc = Packet {
			id: 11,
			buffer: BytesMut::with_capacity(256),
		};
		
		pc.buffer.put_i32_le(p.id);
		
		pc
	}
}

pub trait Packets {
	fn int_packet(&mut self, i: u16, v: i32);
	fn put_str(&mut self, v: String);
	fn string_packet(&mut self, i: u16, v: String);
	fn write_packet(&mut self, p: &Packet);
}

impl Packets for BytesMut {
	fn int_packet(&mut self, i: u16, v:i32) {
		self.put_u16_le(i);
		self.put_u8(0);
		self.put_u32_le(4);
		self.put_i32_le(v);
	}
	
	fn put_str(&mut self, v: String) {
		let mut strlen = v.len();
		if strlen == 0 {
			self.put_u8(0);
			return;
		}
		self.put_u8(11);
		while strlen >= 0x80 {
			self.put_u8((strlen | 0x80) as u8);
			strlen >>= 7;
		}
		self.put_u8(strlen as u8);
		self.put(v.as_bytes());
	}
	
	fn string_packet(&mut self, i: u16, v: String) {
		self.put_u16_le(i);
		self.put_u8(0);
		let mut strlen = v.len();
		if strlen == 0 {
			self.put_u32_le(1);
			self.put_u8(0);
		}
		self.put_i32_le((strlen + (strlen / 128) + 2) as i32);
		self.put_u8(11);
		while strlen >= 0x80 {
			self.put_u8((strlen | 0x80) as u8);
			strlen >>= 7;
		}
		self.put_u8(strlen as u8);
		self.put(v.as_bytes());
	}
	
	fn write_packet(&mut self, p: &Packet) {
		self.put_u16_le(p.id);
		self.put_u8(0);
		self.put_i32_le(p.buffer.len() as i32);
		self.put(&p.buffer[..]);
	}
}

