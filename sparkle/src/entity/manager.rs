use std::collections::VecMap;
use component::{Component, ComponentIndex, Store, StoreMap};
use command::{Command, CommandSender};
use builder::{Builder, BuilderMap};
use group::GroupMap;
use tag::TagMap;
use space::Space;

use entity::{Pool, Entity, MetaEntity};

struct Entities {
    pub pool: Pool,
    pub mentities: VecMap<MetaEntity>,
    pub components: StoreMap
}
impl Entities {
    pub fn new() -> Entities {
        Entities {
            pool: Pool::new(),
            mentities: VecMap::new(),
            components: StoreMap::new()
        }
    }

    #[inline]
    pub fn create(&mut self) -> Entity {
        let meta_entity = self.pool.get();
        let entity = meta_entity.entity;
        self.mentities.insert(entity, meta_entity);

        entity
    }

    #[inline]
    pub fn remove(&mut self, entity: &Entity) {
        self.mentities.remove(entity);
    }

    #[inline]
    pub fn get_mentity_mut(&mut self, entity: &Entity) -> &mut MetaEntity {
        self.mentities.get_mut(entity)
                      .expect(format!("There is no meta information for {}", entity)[])
    }

    #[inline]
    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.components.attach_component(entity, component);
        self.mentities.get_mut(entity).map(|mentity| mentity.component_bits.insert(type_index));
    }

    #[inline]
    pub fn detach_component<T>(&mut self, entity: &Entity) 
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.components.detach_component::<T>(entity);
        self.mentities.get_mut(entity).map(|mentity| mentity.component_bits.remove(&type_index));
    }

    #[inline]
    pub fn get_component<T>(&self, entity: &Entity) -> Option<&T> 
        where T: Component + ComponentIndex 
    {
        self.components.get_component::<T>(entity)
    }

    #[inline]
    pub fn get_component_mut<T>(&mut self, entity: &Entity) -> Option<&mut T> 
        where T: Component + ComponentIndex 
    {
        self.components.get_component_mut::<T>(entity)
    }

    #[inline]
    pub fn get_store<T>(&self) -> Option<&Store<T>> 
        where T: Component + ComponentIndex
    {
        self.components.get_store::<T>()
    }

    #[inline]
    pub fn get_store_mut<T>(&mut self) -> Option<&mut Store<T>> 
        where T: Component + ComponentIndex
    {
        self.components.get_store_mut::<T>()
    }
}

pub struct Manager {
    entities: Entities,
    groups: GroupMap,
    tags: TagMap,
    builders: BuilderMap,
    cmd_sender: CommandSender<Space>
}

impl Manager {
    pub fn new(cmd_sender: CommandSender<Space>) -> Manager {
        Manager {
            entities: Entities::new(),
            groups: GroupMap::new(),
            tags: TagMap::new(),
            builders: BuilderMap::new(),
            cmd_sender: cmd_sender
        }
    }

    pub fn create(&mut self) -> Entity {
        let entity = self.entities.create();
        self.cmd_sender.send(NotifyCreated(entity));

        entity
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.cmd_sender.send(NotifyRemoved(*entity));
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        self.entities.attach_component::<T>(entity, component);
        self.cmd_sender.send(NotifyChanged(*entity));
    }

    pub fn detach_component<T>(&mut self, entity: &Entity) 
        where T: Component + ComponentIndex
    {
        self.entities.detach_component::<T>(entity);
        self.cmd_sender.send(NotifyChanged(*entity));
    }

    #[inline]
    pub fn get_component<T>(&self, entity: &Entity) -> Option<&T> 
        where T: Component + ComponentIndex 
    {
        self.entities.get_component::<T>(entity)
    }

    #[inline]
    pub fn get_component_mut<T>(&mut self, entity: &Entity) -> Option<&mut T> 
        where T: Component + ComponentIndex 
    {
        self.entities.get_component_mut::<T>(entity)
    }

    pub fn get_store<T>(&self) -> Option<&Store<T>> 
        where T: Component + ComponentIndex
    {
        self.entities.get_store::<T>()
    }

    pub fn get_store_mut<T>(&mut self) -> Option<&mut Store<T>> 
        where T: Component + ComponentIndex
    {
        self.entities.get_store_mut::<T>()
    }

    pub fn insert_group(&mut self, group: &str, entity: &Entity) {
        self.groups.insert(group, self.entities.get_mentity_mut(entity));
        self.cmd_sender.send(NotifyChanged(*entity));
    }

    pub fn remove_from_group(&mut self, group_name: &str, entity: &Entity) {
        self.groups.remove_from(group_name, self.entities.get_mentity_mut(entity));
        self.cmd_sender.send(NotifyChanged(*entity));
    }

    pub fn get_from_group(&self, group_name: &str) -> Vec<Entity> {
        self.groups.get(group_name)
    }

    pub fn insert_tag(&mut self, tag: &str, entity: &Entity) {
        self.tags.insert(tag, self.entities.get_mentity_mut(entity));
        self.cmd_sender.send(NotifyChanged(*entity));
    }

    pub fn remove_tag(&mut self, entity: &Entity) {
        self.tags.remove(self.entities.get_mentity_mut(entity));
        self.cmd_sender.send(NotifyChanged(*entity));
    }

    pub fn get_with_tag(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag)
    }

    pub fn insert_builder<T>(&mut self, name: &str, builder: T) where T: Builder {
        self.builders.insert(name, builder);
    }

    pub fn build_entity_with(&mut self, name: &str) -> Entity 
    {
        let Manager { ref mut entities, ref mut groups, ref mut tags, ref mut builders, .. } = *self;
        let entity = entities.create();
        let mentity = entities.get_mentity_mut(&entity);
        self.cmd_sender.send(NotifyCreated(entity));

        builders.get_builder_mut(name).map(|builder| {
            builder.create_entity(mentity, groups, tags)
        }).expect(format!("No template with the name {} was found.", name)[])
    }
}

struct NotifyCreated(pub Entity);
impl Command<Space> for NotifyCreated {
    fn run(&self, space: &mut Space) {
        space.sm.notify_entity_created(space.em.entities.get_mentity_mut(&self.0));
    }
}

struct NotifyChanged(pub Entity);
impl Command<Space> for NotifyChanged {
    fn run(&self, space: &mut Space) {
        space.sm.notify_entity_changed(space.em.entities.get_mentity_mut(&self.0));    
    }
}

struct NotifyRemoved(pub Entity);
impl Command<Space> for NotifyRemoved {
    fn run(&self, space: &mut Space) {
        {
            let mentity = space.em.entities.get_mentity_mut(&self.0);

            space.sm.notify_entity_removed(mentity);
            space.em.groups.clear_entity(mentity);
            space.em.tags.remove(mentity);
        }

        space.em.entities.remove(&self.0);
    }
}