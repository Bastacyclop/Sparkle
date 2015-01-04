use std::collections::RingBuf;
use entity::{Entity, MetaEntity};

#[derive(Hash, PartialEq, Eq, Copy)]
pub enum Type {
    Created,
    Removed,
    Changed
}

#[derive(Hash, PartialEq, Eq, Copy)]
pub struct Event {
    pub entity: Entity,
    pub event_type: Type
}

impl Event {
    pub fn new_created(entity: Entity) -> Event {
        Event {
            entity: entity,
            event_type: Type::Created
        }
    }

    pub fn new_removed(entity: Entity) -> Event {
        Event {
            entity: entity,
            event_type: Type::Removed
        }
    }

    pub fn new_changed(entity: Entity) -> Event {
        Event {
            entity: entity,
            event_type: Type::Changed
        }
    }
}

pub trait Observer {
    fn on_created(&mut self, _entity: &MetaEntity) {}
    fn on_removed(&mut self, _entity: &MetaEntity) {}
    fn on_changed(&mut self, _entity: &MetaEntity) {}
}

pub struct Queue {
    events: RingBuf<Event>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            events: RingBuf::new()
        }
    }

    pub fn add(&mut self, update: Event) {
        self.events.push_back(update);
    }

    pub fn get_update_count(&self) -> uint {
        self.events.len()
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.events.pop_back()
    }
}