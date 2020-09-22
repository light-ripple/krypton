use crate::objects::Player;

use std::string::String;
use std::sync::Mutex;
use std::vec::Vec;

#[derive(Debug)]
pub struct Channel {
    pub name: String,
    pub desc: String,
    players: Mutex<Vec<Player>>
}