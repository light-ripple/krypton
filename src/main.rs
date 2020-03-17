mod objects;
mod packets;
mod query;

extern crate rand;
extern crate bcrypt;

use crate::packets::{Packet, Packets};
use objects::player as player;

use rand::Rng;
use std::str::FromStr;
use std::str;
use bytes::{Bytes, BytesMut, BufMut};
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};
use mysql::*;
use mysql::prelude::*;

const MAX_BPACKET: usize = 8192;

static mut PLAYERS: Vec<player::Player> = Vec::new();

fn get_player<'a>(tok: u64) -> Option<&'a mut player::Player> {
	unsafe {
		for i in 0..PLAYERS.len() {
			if PLAYERS[i].token == tok {
				return Some(&mut PLAYERS[i]);
			}
		}
	}
	None
}

/*#[get("/")]
async fn fmain() -> HttpResponse {
	HttpResponse::Ok().body(format!("Hello there!\nUsers Online: {}", PLAYERS.len()))
}*/

fn event_get_stats(mut p: &mut player::Player) {
	p.queue.write_packet(&Packet::stats_packet(p));
}

//#[get("/")]
async fn bmain(req: HttpRequest, body: Bytes, pool: web::Data<Pool>) -> HttpResponse {
	let header = req.headers().get("User-Agent");
	if header.is_none() {
		return HttpResponse::BadRequest().body("Bad Request")
	}
	
	let res = header.unwrap().to_str().unwrap();
	if res != "osu!" {
		unsafe {
			return HttpResponse::Ok().body(format!("Hello there!\nUsers Online: {}\n\n{}", PLAYERS.len(), res));
		}
	}
	
	let token = req.headers().get("osu-token");
	if token.is_none() {
		// Login
		println!("Body {:?}!", body);
		let strd = str::from_utf8(&body).unwrap();
		let v: Vec<&str> = strd.split('\n').collect(); 
		if v.len() < 3 {
			return HttpResponse::BadRequest().body("Bad Request")
		}
		let username = v[0];
		let password = v[1];
		let _client_data = v[2];
		// SQL stuff here
		let mut conn = pool.get_conn().unwrap();
		let res = match conn.exec_first::<(i32, String, String), _, _>("SELECT id, username, password_md5 FROM users WHERE username = ? OR username_safe = ?", (username, username)) {
			Ok(v) => v,
			Err(e) => panic!(e),
		};
		if res.is_none() {
			return HttpResponse::Ok().set_header("cho-token", "no").body(&b"\x05\x00\x00\x04\x00\x00\x00\xFF\xFF\xFF\xFF"[..])
		}
		
		let (id, username, db_password) = res.unwrap();
		// BCrypt stuff here
		let verified = bcrypt::verify(password, &db_password).unwrap();
		// Failed Login
		if !verified {
			return HttpResponse::Ok().set_header("cho-token", "no").body(&b"\x05\x00\x00\x04\x00\x00\x00\xFF\xFF\xFF\xFF"[..]);
		}
		
		let mut rng = rand::thread_rng();
		let token: u64 = rng.gen();
		
		let mut p = player::Player::new(id, String::from(username), token);
		let mut buf = BytesMut::with_capacity(MAX_BPACKET);
		buf.int_packet(5, p.id);
		buf.string_packet(24, format!("Hello, {}!", p.username));
		for i in 0..4 {
			p.stats.push(player::Stats::new());
			p.set_stats(i, false, &mut conn);
		}
		for i in 0..3 {
			p.stats.push(player::Stats::new());
			p.set_stats(i, true, &mut conn);
		}
		unsafe {
			let presence_packet = Packet::presence_packet(&mut p);
			let stats_packet = Packet::stats_packet(&mut p);
			buf.write_packet(&presence_packet);
			buf.write_packet(&stats_packet);
			for i in 0..PLAYERS.len() {
				PLAYERS[i].queue.write_packet(&presence_packet);
				PLAYERS[i].queue.write_packet(&stats_packet);
				buf.write_packet(&Packet::presence_packet(&mut PLAYERS[i]));
				buf.write_packet(&Packet::stats_packet(&mut PLAYERS[i]));
			}
			buf.write_packet(&Packet::channel_exists(String::from("#osu"), String::from("Main osu channel"), 0));
			PLAYERS.push(p);
		}
		HttpResponse::Ok().set_header("cho-token", format!("{}", token)).body(buf)
	} else {
		let p = get_player(u64::from_str(token.unwrap().to_str().unwrap()).unwrap());
		
		if p.is_none() {
			return HttpResponse::Forbidden().body(&b"\x05\x00\x00\x04\x00\x00\x00\xFB\xFF\xFF\xFF"[..]);
		}
		let mut p = p.unwrap();
		println!("Request from {}", p.username);
		let mut i = 0;
		loop {
			if i + 6 > body.len() {
				break
			}
			let id: u16;
			let len: u32;
			unsafe {
				id = *(&body[i] as *const _ as *const u16);
				i += 3;
				len = *(&body[i] as *const _ as *const u32);
				i += 4;
				i += len as usize;
			}
			
			match id {
				3 => event_get_stats(p),
				_ => println!("Got Packet {} with Len {}", id, len),
			}
			p.queue.write_packet(&Packet::message_packet(format!("Packet {} with {}", id, len), String::from("#osu"), String::from("Bot1"), 1339));
		}
		
		let mut buf = BytesMut::with_capacity(1024);
		buf.put(&p.queue[..]);
		unsafe {
			p.queue.set_len(0);
		}
		
		HttpResponse::Ok()
        .body(buf)
	}
}

const DB_URL: &'static str = "mysql://root:lol123@localhost:3306/ripplef";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	let pool = Pool::new(DB_URL).unwrap();
	// PLAYERS = HashMap::new();
	// assert!(false);
	println!("Starting Server");
    HttpServer::new(move || App::new()
		.data(pool.clone())
		.service(web::resource("/").route(web::post().to(bmain))))
        .bind("127.0.0.1:5001")?
        .run()
        .await
}
