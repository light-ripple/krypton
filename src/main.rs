mod objects;
mod packets;

extern crate rand;

use crate::packets::{Packet, Packets};
use objects::player as player;

use rand::Rng;
use std::str::FromStr;
use std::str;
use bytes::{Bytes, BytesMut, BufMut};
use actix_web::{web, App, HttpServer, HttpResponse, HttpRequest};

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

//#[get("/")]
async fn bmain(req: HttpRequest, body: Bytes) -> HttpResponse {
	println!("Request");
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
		let _password = v[1];
		let _client_data = v[2];
		// BCrypt stuff here
		
		// Failed Login
		
		let id = 1337;
		
		let mut rng = rand::thread_rng();
		let token: u64 = rng.gen();
		
		let mut p = player::Player::new(id, String::from(username), token);
		let mut buf = BytesMut::with_capacity(MAX_BPACKET);
		buf.int_packet(5, p.id);
		buf.string_packet(24, format!("Hello, {}!", p.username));
		unsafe {
			let presence_packet = Packet::presence_packet(&mut p);
			buf.write_packet(&presence_packet);
			for i in 0..PLAYERS.len() {
				PLAYERS[i].queue.write_packet(&presence_packet);
				//PLAYERS[i].queue.stats_packet(&mut p);
				buf.write_packet(&Packet::presence_packet(&mut PLAYERS[i]));
				//buf.stats_packet(&mut PLAYERS[i]);
			}
			PLAYERS.push(p);
		}
		HttpResponse::Ok().set_header("cho-token", format!("{}", token)).body(buf)
	} else {
		let p = get_player(u64::from_str(token.unwrap().to_str().unwrap()).unwrap());
		
		if p.is_none() {
			return HttpResponse::Ok().body(&b"\x05\x00\x00\x04\x00\x00\x00\xFB\xFF\xFF\xFF"[..]);
		}
		let p = p.unwrap();
		
		let mut buf = BytesMut::with_capacity(1024);
		buf.put(&p.queue[..]);
		unsafe {
			p.queue.set_len(0);
		}
		println!("Request from {}", p.username);
		println!("Body {:?}!", body);
		HttpResponse::Ok()
        .body(buf)
	}
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
	// PLAYERS = HashMap::new();
	// assert!(false);
	println!("Starting Server");
    HttpServer::new(|| App::new().service(web::resource("/").route(web::post().to(bmain))))
        .bind("127.0.0.1:5001")?
        .run()
        .await
}