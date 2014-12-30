use std::collections::{VecMap, HashSet};
use entity::{Entity, MetaEntity};

#[deriving(Hash, PartialEq, Eq, Copy)]
pub enum Type {
    Created,
    Removed,
    Changed
}

#[deriving(Hash, PartialEq, Eq, Copy)]
pub struct Update {
    pub entity: Entity,
    pub update_type: Type
}

impl Update {
    pub fn new_created(entity: Entity) -> Update {
        Update {
            entity: entity,
            update_type: Type::Created
        }
    }

    pub fn new_removed(entity: Entity) -> Update {
        Update {
            entity: entity,
            update_type: Type::Removed
        }
    }

    pub fn new_changed(entity: Entity) -> Update {
        Update {
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

pub struct Record {
    updates: HashSet<Update>
}

impl Record {
    pub fn new() -> Record {
        Record {
            updates: HashSet::new()
        }
    }

    pub fn add(&mut self, update: Update) {
        self.updates.insert(update);
    }

    pub fn notify_and_flush<T>(&mut self, mentities: &VecMap<MetaEntity>, observer: &mut T) where T: Observer {
        for update in self.updates.drain() {
            mentities.get(&update.entity).map(|mentity| Record::notify_with(mentity, update, observer));
        }
    }

    fn notify_with<T>(mentity: &MetaEntity, update: Update, observer: &mut T) where T: Observer {
        match update.update_type {
            Type::Created => observer.on_created(mentity),
            Type::Removed => observer.on_removed(mentity),
            Type::Changed => observer.on_removed(mentity)
        }
    }
}