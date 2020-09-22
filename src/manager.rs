use crate::objects::{Channel, Player};

use std::collections::HashMap;
use std::string::String;
use std::sync::{Arc, Mutex, RwLock};

#[derive(Debug)]
pub struct Manager {
    pub counter: Mutex<i32>,
    players: RwLock<HashMap<String, Arc<Player>>>,
    channels: RwLock<HashMap<String, Arc<Channel>>>
}

impl Manager {
    pub fn new() -> Self {
        Self {
            counter: Mutex::new(0),
            players: RwLock::new(HashMap::new()),
            channels: RwLock::new(HashMap::new())
        }
    }

    pub fn add_player(&self, p: Player) {
        let mut players = self.players.write().unwrap();
        players.insert(p.token.clone(), Arc::new(p));
    }

    pub fn get_player(&self, tok: String) -> Option<Arc<Player>> {
        let players = self.players.read().unwrap();
        match players.get(&tok) {
            Some(p) => Some(p.clone()),
            None => None
        }
    }

    pub fn add_channel(&self, c: Channel) {
        let mut channels = self.channels.write().unwrap();
        channels.insert(c.name.clone(), Arc::new(c));
    }

    pub fn get_channel(&self, name: String) -> Option<Arc<Channel>> {
        let channels = self.channels.read().unwrap();
        match channels.get(&name) {
            Some(c) => Some(c.clone()),
            None => None
        }
    }
}