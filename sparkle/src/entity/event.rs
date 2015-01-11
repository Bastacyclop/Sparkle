use std::collections::RingBuf;
use entity::Entity;

pub use self::Kind::{Created, Removed};

#[derive(Copy, Show)]
pub enum Kind {
    Created,
    Removed
}

pub struct Queue {
    events: RingBuf<(Kind, Entity)>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            events: RingBuf::new()
        }
    }

    pub fn created(&mut self, entity: Entity) {
        self.events.push_back((Created, entity))
    }

    pub fn removed(&mut self, entity: Entity) {
        self.events.push_back((Removed, entity))
    }

    pub fn pop(&mut self) -> Option<(Kind, Entity)> {
        self.events.pop_front()
    }
}
