use crate::objects::Channel;

use std::string::String;
use std::sync::Mutex;
use std::vec::Vec;

#[derive(Debug)]
pub struct Player {
    pub id: i32,
    pub token: String,
    channels: Mutex<Vec<Channel>>
}

impl Player {
    pub fn new(id: i32) -> Player {
        Player{
            id,
            token: String::from("some"),
            channels: Mutex::new(vec![])
        }
    }
}