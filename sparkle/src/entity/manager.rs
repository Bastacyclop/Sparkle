use std::collections::VecMap;
use std::cell::{Ref, RefMut};
use component::{self, Component, ComponentIndex, StoreMap};
use entity;
use entity::event;
use entity::{Entity, MetaEntityMap, GroupMap, TagMap};

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
            let mentity = self.mentities.get(entity);
            component::store::private::forget(&mut self.stores, mentity);
            entity::group::private::forget(&mut self.groups, mentity);
            entity::tag::private::forget(&mut self.tags, mentity);
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
        self.mentities.drain_events_with(|(kind, mentity)| {
            match kind {
                event::Changed => obs.notify_changed(mentity),
                event::Removed => obs.notify_removed(mentity)
            }
        });
    }
}
