pub mod events;
pub mod manager;
pub mod models;
pub mod objects;
pub mod packets;

use bytes::BytesMut;
use crate::manager::Manager;
use crate::models::User;
use crate::objects::Player;

use crate::packets::PacketWriter;
use crate::packets::client::PacketType;
use crate::packets::server::{
    LoginReply,
    ChannelInfo,
    ChannelJoined
};

use bytes::{Buf, Bytes};
use std::sync::Arc;
use std::string::String;
use actix_web::{middleware, get, post, web, App, HttpServer, HttpRequest, HttpResponse, Responder};

#[get("/")]
async fn meme(manager: web::Data<Arc<Manager>>, pool: web::Data<sqlx::MySqlPool>) -> impl Responder {
    let mut count = manager.counter.lock().unwrap();
    *count += 1;
    let res: Result<(i32,), sqlx::Error> = sqlx::query_as("SELECT id FROM users LIMIT 1")
        .fetch_one(pool.get_ref()).await;

    match res {
        Ok(v) => println!("{}", v.0),
        Err(e) => println!("{:?}", e)
    }

    HttpResponse::Ok().body(format!("Connections Handled (meme): {:?}", *count))
}

use std::iter;
use rand::{Rng, thread_rng};
use rand::distributions::Alphanumeric;

#[post("/")]
async fn bancho_main(
    req: HttpRequest,
    mut body: Bytes,
    manager: web::Data<Manager>,
    pool: web::Data<sqlx::MySqlPool>
) -> impl Responder {
    let headers = req.headers();

    match headers.get("osu-token") {
        None => {
            // Login Here
            let (username, password, _client_data) = {
                let body_str = std::str::from_utf8(&body).unwrap();
                let parts: Vec<&str> = body_str.split('\n').collect();
                (parts[0].trim(), parts[1], parts[2])
            };

            let user = sqlx::query_as::<_, User>("SELECT id, username, password FROM users WHERE username = ? OR username_safe = ?")
                .bind(username)
                .bind(username)
                .fetch_one(pool.get_ref()).await;

            let mut user = match user {
                Ok(v) => v,
                Err(e) => {
                    println!("{:?}", e);
                    return HttpResponse::Ok().set_header("cho-token", "no").body(&b"\x05\x00\x00\x04\x00\x00\x00\xFF\xFF\xFF\xFF"[..]);
                }
            };

            if bcrypt::verify(password, &user.password).unwrap() {
                user.password = password.to_string();
            } else {

            }

            let mut rng = thread_rng();
            let token: String = iter::repeat(())
                .map(|()| rng.sample(Alphanumeric))
                .take(10)
                .collect();

            println!("User {:?}!", user);

            let player = Player::new(user, token.clone());
            let mut out = BytesMut::with_capacity(256);

            out.write_packet(LoginReply {
                id: player.user.id
            });
            
            manager.channels_fn(|channel| {
                if channel.autojoin {
                    out.write_packet(ChannelJoined {
                        name: channel.name.clone()
                    });
                }
                out.write_packet(ChannelInfo::from(channel))
            });

            manager.add_player(player);

            HttpResponse::Ok()
                .set_header("cho-token", token)
                .body(out)
        },
        Some(token) => {
            let p = match manager.get_player(String::from(token.to_str().unwrap())) {
                Some(x) => x,
                None => {
                    return HttpResponse::Ok().body(&b""[..]);
                }
            };

            loop {
                if body.remaining() < 7 {
                    break
                }

                let id = PacketType::from(body.get_u16_le());
                body.advance(1);
                let len = body.get_u32_le() as usize;

                if body.remaining() < len {
                    break
                }

                let mut _data = body.slice(..len);

                match id {
                    PacketType::StatusUpdate => events::status_update(&manager, &p),
                    PacketType::Ping => p.update_ping(),
                    _ => println!("Unhandled Packet {:?} with Len {}", id, len),
                }
            }

            let queue = {
                let mut queue = p.queue.lock().unwrap();
                let buf = queue.clone();
                
                queue.clear();
                buf
            };

            HttpResponse::Ok().body(queue)
        }
    }
}

use sqlx::mysql::MySqlPoolOptions;

#[actix_web::main]
async fn main() -> anyhow::Result<()> {
    println!("Hello, world!");
    println!("Welcome to krypton v2 (edition spielplatz)");

    let pool = MySqlPoolOptions::new()
        .max_connections(16)
        .connect_timeout(std::time::Duration::from_secs(5))
        .connect("mysql://root:lol123@localhost:3306/ripple").await?;

    pool.acquire().await?;

    let man = web::Data::new(Manager::new());

    println!("Now Listening on :5001");
    HttpServer::new(move || {
        App::new()
            .data(man.clone())
            .data(pool.clone())
            .wrap(middleware::Logger::default())
            .service(bancho_main)
            .service(meme)
    })
    .bind("127.0.0.1:5001")?
    .run()
    .await?;

    Ok(())
}