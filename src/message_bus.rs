use std::time::{Duration, Instant};

use crate::config::MESSAGE_EXPIRATION_IN_SECONDS;

pub struct MessageBus {
    message: Option<String>,
    last_updated: Instant,
    expiration: Duration,
}

impl MessageBus {
    pub fn new() -> MessageBus {
        MessageBus {
            message: None,
            last_updated: Instant::now(),
            expiration: Duration::from_secs(MESSAGE_EXPIRATION_IN_SECONDS),
        }
    }

    pub fn check(&mut self) {
        if Instant::now() > (self.last_updated + self.expiration) {
            self.clear()
        }
    }

    pub fn read(&self) -> Option<&str> {
        self.message.as_deref()
    }

    pub fn send(&mut self, message: String) {
        self.message = Some(message);
        self.last_updated = Instant::now();
    }

    pub fn clear(&mut self) {
        self.message = None;
    }
}
