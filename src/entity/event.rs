use std::collections::{VecMap, RingBuf};
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
    pub update_type: Type
}

impl Event {
    pub fn new_created(entity: Entity) -> Event {
        Event {
            entity: entity,
            update_type: Type::Created
        }
    }

    pub fn new_removed(entity: Entity) -> Event {
        Event {
            entity: entity,
            update_type: Type::Removed
        }
    }

    pub fn new_changed(entity: Entity) -> Event {
        Event {
            entity: entity,
            update_type: Type::Changed
        }
    }
}

pub trait Observer {
    fn on_created(&mut self, _entity: &MetaEntity) {}
    fn on_removed(&mut self, _entity: &MetaEntity) {}
    fn on_changed(&mut self, _entity: &MetaEntity) {}
}

pub struct Queue {
    updates: RingBuf<Event>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            updates: RingBuf::new()
        }
    }

    pub fn add(&mut self, update: Event) {
        self.updates.push_back(update);
    }

    pub fn get_update_count(&self) -> uint {
        self.updates.len()
    }
}