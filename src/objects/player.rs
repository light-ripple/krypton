use crate::objects::Channel;
use crate::models::User;

use bytes::BytesMut;

use std::string::String;
use std::sync::{Arc, Mutex};
use std::vec::Vec;
use std::time::SystemTime;

#[derive(Debug)]
pub struct Player {
    pub user: User,
    pub token: String,
    pub ping_time: Mutex<SystemTime>,
    channels: Mutex<Vec<Arc<Channel>>>,
    pub queue: Mutex<BytesMut>
}

impl Player {
    pub fn new(user: User, token: String) -> Self {
        Self {
            user,
            token,
            ping_time: Mutex::new(SystemTime::now()),
            channels: Mutex::new(vec![]),
            queue: Mutex::new(BytesMut::with_capacity(128))
        }
    }

    pub fn update_ping(&self) {
        *self.ping_time.lock().unwrap() = SystemTime::now();
    }

    pub fn add_channel(&mut self, channel: &Arc<Channel>) {
        self.channels.lock().unwrap().push(channel.clone());
    }
}