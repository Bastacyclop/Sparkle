use std::collections::VecMap;
use std::cell::{Ref, RefMut};
use std::ops::{Deref, DerefMut};
use split_access::Access;
use component::{self, Component, ComponentIndex, StoreMap};
use entity;
use entity::event;
use entity::{Entity, MetaEntityMap, GroupMap, TagMap};

split_access_struct! {
    view: ManagerView
    #[doc = ""]
    pub struct Manager {
        mentities: MetaEntityMap [M],
        stores: StoreMap [S],
        groups: GroupMap [G],
        tags: TagMap [T]
    }
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
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access + DerefMut<Target=MetaEntityMap>,
          S: Access,
          G: Access,
          T: Access
{
    pub fn create_entity(&mut self) -> Entity {
        self.mentities.create()
    }
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access + DerefMut<Target=MetaEntityMap>,
          S: Access,
          G: Access + DerefMut<Target=GroupMap>,
          T: Access + DerefMut<Target=TagMap>
{
    pub fn remove_entity(&mut self, entity: Entity) {
        {
            let mentity = self.mentities.get(entity);
            entity::group::private::forget(self.groups.deref_mut(), mentity);
            entity::tag::private::forget(self.tags.deref_mut(), mentity);
        }
        self.mentities.remove(entity)
    }
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access + DerefMut<Target=MetaEntityMap>,
          S: Access + DerefMut<Target=StoreMap>,
          G: Access,
          T: Access
{
    pub fn attach_component<C>(&mut self, entity: Entity, component: C)
        where C: Component + ComponentIndex
    {
        self.stores.insert(self.mentities.get_mut(entity), component);
    }

    pub fn detach_component<C>(&mut self, entity: Entity)
        where C: Component + ComponentIndex
    {
        self.stores.remove::<C>(self.mentities.get_mut(entity));
    }
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access,
          S: Access + DerefMut<Target=StoreMap>,
          G: Access,
          T: Access
{
    pub fn get_stores_mut(&'a mut self) -> (&'a mut StoreMap,
                                            ManagerView<'a, &mut <M as Access>::Spread,
                                                            (),
                                                            &mut <G as Access>::Spread,
                                                            &mut <T as Access>::Spread>) {
        (self.stores.deref_mut(),
         ManagerView {
             mentities: self.mentities.spread(),
             stores: (),
             groups: self.groups.spread(),
             tags: self.tags.spread()
         })
    }
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access,
          S: Access + Deref<Target=StoreMap>,
          G: Access,
          T: Access
{
    pub fn get_store<C>(&'a mut self) -> (Option<Ref<'a, VecMap<C>>>,
                                          ManagerView<'a, &mut <M as Access>::Spread,
                                                          &'a StoreMap,
                                                          &mut <G as Access>::Spread,
                                                          &mut <T as Access>::Spread>)
        where C: Component + ComponentIndex
    {
        (self.stores.get::<C>(),
         ManagerView {
             mentities: self.mentities.spread(),
             stores: self.stores.deref(),
             groups: self.groups.spread(),
             tags: self.tags.spread()
         })
    }

    pub fn get_store_mut<C>(&'a mut self) -> (Option<RefMut<'a, VecMap<C>>>,
                                              ManagerView<'a,
                                                          &mut <M as Access>::Spread,
                                                          &'a StoreMap,
                                                          &mut <G as Access>::Spread,
                                                          &mut <T as Access>::Spread>)
        where C: Component + ComponentIndex
    {
        (self.stores.get_mut::<C>(),
         ManagerView {
             mentities: self.mentities.spread(),
             stores: self.stores.deref(),
             groups: self.groups.spread(),
             tags: self.tags.spread()
         })
    }
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access + DerefMut<Target=MetaEntityMap>,
          S: Access,
          G: Access + DerefMut<Target=GroupMap>,
          T: Access
{
    pub fn insert_group(&mut self, group: &str, entity: Entity) {
        self.groups.insert(group, self.mentities.get_mut(entity));
    }

    pub fn remove_from_group(&mut self, group: &str, entity: Entity) {
        self.groups.remove_from(group, self.mentities.get_mut(entity));
    }
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access,
          S: Access,
          G: Access + Deref<Target=GroupMap>,
          T: Access
{
    pub fn get_group(&mut self, group: &str) -> Vec<Entity> {
        self.groups.get(group)
    }
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access + DerefMut<Target=MetaEntityMap>,
          S: Access,
          G: Access,
          T: Access + DerefMut<Target=TagMap>
{
    pub fn insert_tag(&mut self, tag: &str, entity: Entity) {
        self.tags.insert(tag, self.mentities.get_mut(entity));
    }

    pub fn remove_tag(&mut self, entity: Entity) {
        self.tags.remove(self.mentities.get_mut(entity));
    }
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access,
          S: Access,
          G: Access,
          T: Access + Deref<Target=TagMap>
{
    pub fn get_tag(&mut self, tag: &str) -> Option<Entity> {
        self.tags.get(tag)
    }
}

impl<'a, M, S, G, T> ManagerView<'a, M, S, G, T>
    where M: Access + DerefMut<Target=MetaEntityMap>,
          S: Access + DerefMut<Target=StoreMap>,
          G: Access,
          T: Access
{
    pub fn notify_events<O>(&mut self, obs: &mut O) where O: event::Observer {
        let mentities = self.mentities.deref_mut();
        let stores = self.stores.deref_mut();

        mentities.drain_events_with(|(kind, mentity)| {
            match kind {
                event::Changed => obs.notify_changed(mentity),
                event::Removed => {
                    obs.notify_removed(mentity);
                    component::store::private::forget(stores, mentity);
                }
            }
        });
    }
}
