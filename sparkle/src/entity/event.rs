use std::collections::{RingBuf, HashSet};
use std::collections::ring_buf;
use entity::{Entity, MetaEntity};

pub use self::Kind::{Changed, Removed};

#[derive(Copy, Show)]
pub enum Kind {
    Changed,
    Removed
}

pub type Event = (Kind, Entity);
pub type Drain<'a> = ring_buf::Drain<'a, (Kind, Entity)>;

pub struct Queue {
    changed_set: HashSet<Entity>,
    events: RingBuf<Event>
}

impl Queue {
    pub fn new() -> Queue {
        Queue {
            changed_set: HashSet::new(),
            events: RingBuf::new()
        }
    }

    pub fn changed(&mut self, entity: Entity) {
        if self.changed_set.insert(entity) {
            self.events.push_back((Changed, entity))
        }
    }

    pub fn removed(&mut self, entity: Entity) {
        self.events.push_back((Removed, entity))
    }

    pub fn drain(&mut self) -> Drain {
        self.changed_set.clear();
        self.events.drain()
    }
}

pub trait Observer {
    fn notify_changed(&mut self, mentity: &MetaEntity);
    fn notify_removed(&mut self, mentity: &MetaEntity);
}