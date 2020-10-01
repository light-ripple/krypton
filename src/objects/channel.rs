use crate::objects::Player;

use std::string::String;
use std::sync::{Arc, RwLock};
use std::vec::Vec;

#[derive(Debug)]
pub struct Channel {
    pub name: String,
    pub desc: String,
    players: RwLock<Vec<Arc<Player>>>
}

impl Channel {
    pub fn new(name: String, desc: String) -> Self {
        Self {
            name,
            desc,
            players: RwLock::new(vec![])
        }
    }
}