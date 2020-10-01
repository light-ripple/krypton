use crate::objects::Channel;

use std::string::String;
use std::sync::{Arc, Mutex};
use std::vec::Vec;

#[derive(Debug)]
pub struct Player {
    pub id: i32,
    pub token: String,
    channels: Mutex<Vec<Arc<Channel>>>
}

impl Player {
    pub fn new(id: i32) -> Self {
        Self {
            id,
            token: String::from("some"),
            channels: Mutex::new(vec![])
        }
    }
}