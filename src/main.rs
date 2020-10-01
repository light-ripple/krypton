pub mod objects;
pub mod manager;
pub mod packets;

use crate::manager::Manager;

use bytes::{Buf, Bytes};
use std::sync::Arc;
use std::string::String;
use actix_web::{middleware, get, post, web, App, HttpServer, HttpRequest, Responder};

#[get("/")]
async fn meme(manager: web::Data<Arc<Manager>>) -> impl Responder {
    let mut count = manager.counter.lock().unwrap();
    *count += 1;
    format!("Connections Handled (meme): {:?}", *count)
}

#[post("/")]
async fn bancho_main(req: HttpRequest, mut body: Bytes, manager: web::Data<Arc<Manager>>) -> impl Responder {
    let headers = req.headers();

    match headers.get("osu-token") {
        None => {
            // Login Here
            println!("Body {:?}!", body);
        },
        Some(token) => {
            let _p = match manager.get_player(String::from(token.to_str().unwrap())) {
                Some(x) => x,
                None => {
                    return String::from("xd");
                }
            };

            loop {
                if body.remaining() < 7 {
                    break
                }

                let id = body.get_u16_le();
                body.advance(1);
                let len = body.get_u32_le() as usize;

                if body.remaining() < len {
                    break
                }

                let mut _data = body.slice(..len);

                match id {
                    _ => println!("Unhandled Packet {} with Len {}", id, len),
                }
            }
        }
    };

    String::from("Not Implemented")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    println!("Welcome to krypton v2 (edition spielplatz)");

    let man = Arc::new(Manager::new());

    println!("Now Listening on :5001");

    HttpServer::new(move || {
        App::new()
            .data(man.clone())
            .wrap(middleware::Logger::default())
            .service(bancho_main)
            .service(meme)
    })
    .bind("127.0.0.1:5001")?
    .run()
    .await       
}