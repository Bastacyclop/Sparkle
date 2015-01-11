use std::collections::VecMap;
use std::cell::{Ref, RefMut};
use component::{Component, ComponentIndex, StoreMap};
use entity::event::{self, Event};
use entity::{Entity, MetaEntity, MetaEntityMap, GroupMap, TagMap};

pub struct Manager {
    mentities: MetaEntityMap,
    stores: StoreMap,
    groups: GroupMap,
    tags: TagMap
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            mentities: MetaEntityMap::new(),
            stores: StoreMap::new(),
            groups: GroupMap::new(),
            tags: TagMap::new()
        }
    }

    pub fn create(&mut self) -> Entity {
        self.mentities.create()
    }

    pub fn remove(&mut self, entity: Entity) {
        {
            let mentity = self.mentities.get_mut(entity);
            self.stores.remove_all(mentity);
            self.groups.clear_entity(mentity);
            self.tags.remove(mentity);
        }
        self.mentities.remove(entity)
    }

    pub fn attach_component<T>(&mut self, entity: Entity, component: T)
        where T: Component + ComponentIndex 
    {
        self.stores.insert(self.mentities.get_mut(entity), component);
    }

    pub fn detach_component<T>(&mut self, entity: Entity)
        where T: Component + ComponentIndex 
    {
        self.stores.remove::<T>(self.mentities.get_mut(entity));
    }

    pub fn get_store<'a, T>(&'a self) -> Option<Ref<'a, VecMap<T>>>
        where T: Component + ComponentIndex
    {
        self.stores.get::<T>()
    }

    pub fn get_store_mut<'a, T>(&'a self) -> Option<RefMut<'a, VecMap<T>>>
        where T: Component + ComponentIndex
    {
        self.stores.get_mut::<T>()
    }

    pub fn insert_group(&mut self, group: &str, entity: Entity) {
        self.groups.insert(group, self.mentities.get_mut(entity));
    }

    pub fn remove_from_group(&mut self, group: &str, entity: Entity) {
        self.groups.remove_from(group, self.mentities.get_mut(entity));
    }

    pub fn get_from_group(&self, group: &str) -> Vec<Entity> {
        self.groups.get(group)
    }

    pub fn insert_tag(&mut self, tag: &str, entity: Entity) {
        self.tags.insert(tag, self.mentities.get_mut(entity));
    }

    pub fn remove_tag(&mut self, entity: Entity) {
        self.tags.remove(self.mentities.get_mut(entity));
    }

    pub fn get_with_tag(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag)
    }

    pub fn notify_events<T>(&mut self, obs: &mut T) where T: event::Observer {
        let drain = self.mentities.drain_events();
        while let Some((kind, mentity)) = drain.next() {
            let removed = self.handle_event(kind, mentity, obs);
            if removed {
                self.mentities.clear(mentity.entity);
            }
        }
    }

    fn handle_event<T>(&mut self, kind: event::Kind, mentity: &MetaEntity, obs: &mut T) -> bool 
        where T: event::Observer 
    {
        match kind {
            event::Kind::Changed => obs.notify_changed(mentity),
            event::Kind::Removed => {
                obs.notify_removed(mentity);
                return true;
            }
        }

        false
    }
}