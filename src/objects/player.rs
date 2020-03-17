use crate::objects::channel::Channel;
use std::string::String;
use bytes::{BytesMut};
use mysql::prelude::*;

#[derive(Debug)]
pub struct Stats {
	pub ranked_score: u64,
	pub total_score: u64,
	pub playcount: i32,
	pub accuracy: f32,
	pub performance: i32,
	pub rank: i32
}

impl Stats {
	pub fn new() -> Stats {
		Stats{
			ranked_score: 0,
			total_score: 0,
			playcount: 0,
			accuracy: 0.0,
			performance: 0,
			rank: 0,
		}
	}
}

/*

range 12
pos 1, 1
coords -11, -11



*/
#[derive(Debug)]
pub struct Player {
	pub id: i32,
	pub username: String,
	pub token: u64,
	pub queue: BytesMut,
	pub country: u8,
	pub ingame_privileges: u8,
	pub mode: u8,
	pub stats: Vec<Stats>,
	pub channels: Vec<Channel>,	
	
	pub action: u8,
	pub action_text: String,
	pub action_hash: String,
	pub action_mods: i32,
	pub action_beatmap: i32,
}

const MODE_QUERIES: [&'static str; 7] = ["SELECT ranked_score_std, total_score_std, playcount_std, avg_accuracy_std/100, pp_std FROM users_stats WHERE id = ?",
	"SELECT ranked_score_taiko, total_score_taiko, playcount_taiko, avg_accuracy_taiko/100, pp_taiko FROM users_stats WHERE id = ?",
	"SELECT ranked_score_ctb, total_score_ctb, playcount_ctb, avg_accuracy_ctb/100, pp_ctb FROM users_stats WHERE id = ?",
	"SELECT ranked_score_mania, total_score_mania, playcount_mania, avg_accuracy_mania/100, pp_mania FROM users_stats WHERE id = ?",
	"SELECT ranked_score_std, total_score_std, playcount_std, avg_accuracy_std/100, pp_std FROM rx_stats WHERE id = ?",
	"SELECT ranked_score_taiko, total_score_taiko, playcount_taiko, avg_accuracy_taiko/100, pp_taiko FROM rx_stats WHERE id = ?",
	"SELECT ranked_score_ctb, total_score_ctb, playcount_ctb, avg_accuracy_ctb/100, pp_ctb FROM rx_stats WHERE id = ?"];

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
			stats: vec![],
			channels: vec![],
			
			action: 0,
			action_beatmap: 0,
			action_mods: 0,
			action_text: String::from("logging in."),
			action_hash: String::from(""),
		}
	}
	
	pub fn set_stats(&mut self, mut m: usize, rx: bool, conn: &mut mysql::PooledConn) {
		if rx {
			if m > 2 {
				return;
			}
			m += 4;
		}
		let res = conn.exec_first::<(u64, u64, i32, f32, i32), _, _>(MODE_QUERIES[m], (self.id,)).unwrap();
		if res.is_none() {
			return;
		}
		let (ranked_score, total_score, playcount, acc, pp) = res.unwrap();
		let mut s = &mut (self.stats[m]);
		s.ranked_score = ranked_score;
		s.total_score = total_score;
		s.playcount = playcount;
		s.accuracy = acc;
		s.performance = pp;
	}
}