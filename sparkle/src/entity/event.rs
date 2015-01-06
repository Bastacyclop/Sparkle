use std::collections::{RingBuf, HashSet};
use entity::{Entity, MetaEntity};

#[derive(Hash, PartialEq, Eq, Copy)]
pub enum Kind {
    Created,
    Removed,
    Changed
}

#[derive(Hash, PartialEq, Eq, Copy)]
pub struct Event {
    pub entity: Entity,
    pub kind: Kind
}

impl Event {
    pub fn created(entity: Entity) -> Event {
        Event {
            entity: entity,
            kind: Kind::Created
        }
    }

    pub fn removed(entity: Entity) -> Event {
        Event {
            entity: entity,
            kind: Kind::Removed
        }
    }

    pub fn changed(entity: Entity) -> Event {
        Event {
            entity: entity,
            kind: Kind::Changed
        }
    }
}

pub trait Observer {
    fn on_created(&mut self, _entity: &MetaEntity) {}
    fn on_removed(&mut self, _entity: &MetaEntity) {}
    fn on_changed(&mut self, _entity: &MetaEntity) {}
}

pub struct Queue {
    dup_checker: HashSet<Event>,
    events: RingBuf<Event>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            dup_checker: HashSet::new(),
            events: RingBuf::new()
        }
    }

    pub fn add(&mut self, new_event: Event) {
        let ok = self.dup_checker.insert(new_event);
        if ok { self.events.insert(new_event.entity, new_event); }
    }

    pub fn get_update_count(&self) -> uint {
        self.events.len()
    }

    pub fn poll_event(&mut self) -> Option<Event> {
        self.events.pop_back()
    }

    pub fn reset(&mut self) {
        self.events.clear();
        self.dup_checker.clear();
    }
}