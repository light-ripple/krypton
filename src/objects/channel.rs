use crate::objects::Player;

use std::string::String;
use std::sync::{Arc, RwLock};
use std::vec::Vec;

#[derive(Debug)]
pub struct Channel {
    pub name: String,
    pub description: String,
    pub autojoin: bool,
    players: RwLock<Vec<Arc<Player>>>
}

impl Channel {
    pub fn new(name: String, description: String, autojoin: bool) -> Self {
        Self {
            name,
            description,
            autojoin,
            players: RwLock::new(vec![])
        }
    }

    pub fn user_count(&self) -> u16 {
        self.players.read().unwrap().len() as u16
    }
}