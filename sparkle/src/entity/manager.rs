use std::collections::VecMap;
use component::{Component, ComponentIndex, Store, StoreMap};
use group::GroupMap;
use tag::TagMap;
use builder::{Builder, BuilderMap};
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

    pub fn create(&mut self) -> Entity {
        let meta_entity = self.pool.get();
        let entity = meta_entity.entity;
        self.mentities.insert(entity, meta_entity);

        entity
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.mentities.remove(entity);
    }

    pub fn get_mentity(&self, entity: &Entity) -> &MetaEntity {
        self.mentities.get(entity)
                      .expect(format!("There is no meta information for {}", entity)[])    
    }

    pub fn get_mentity_mut(&mut self, entity: &Entity) -> &mut MetaEntity {
        self.mentities.get_mut(entity)
                      .expect(format!("There is no meta information for {}", entity)[])
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.components.attach_component(entity, component);
        self.mentities.get_mut(entity).map(|mentity| mentity.component_bits.insert(type_index));
    }

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

    pub fn get_store<T>(&self) -> Option<&Store<T>> 
        where T: Component + ComponentIndex
    {
        self.components.get_store::<T>()
    }

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
    builders: BuilderMap
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            entities: Entities::new(),
            groups: GroupMap::new(),
            tags: TagMap::new(),
            builders: BuilderMap::new()
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.entities.create()
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.entities.remove(entity)
    }

    pub fn get_mentity(&self, entity: &Entity) -> &MetaEntity {
        self.entities.get_mentity(entity)
    }

    pub fn get_mentity_mut(&mut self, entity: &Entity) -> &mut MetaEntity {
        self.entities.get_mentity_mut(entity)
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        self.entities.attach_component::<T>(entity, component);
    }

    pub fn detach_component<T>(&mut self, entity: &Entity) 
        where T: Component + ComponentIndex
    {
        self.entities.detach_component::<T>(entity);
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
    }

    pub fn remove_from_group(&mut self, group_name: &str, entity: &Entity) {
        self.groups.remove_from(group_name, self.entities.get_mentity_mut(entity))
    }

    pub fn clear_entity_groups(&mut self, entity: &Entity) {
        self.groups.clear_entity(self.entities.get_mentity_mut(entity));
    }

    pub fn get_from_group(&self, group_name: &str) -> Vec<Entity> {
        self.groups.get(group_name)
    }

    pub fn insert_tag(&mut self, tag: &str, entity: &Entity) {
        self.tags.insert(tag, self.entities.get_mentity_mut(entity));
    }

    pub fn remove_tag(&mut self, entity: &Entity) {
        self.tags.remove(self.entities.get_mentity_mut(entity));
    }

    pub fn get_with_tag(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag)
    }

    pub fn insert_builder<T>(&mut self, name: &str, builder: T) where T: Builder {
        self.builders.insert(name, builder);
    }

    pub fn build_entity_with(&mut self, name: &str) -> Entity 
    {
        let Manager { ref mut entities, ref mut groups, ref mut tags, ref mut builders } = *self;
        let entity = entities.create();
        let mentity = entities.get_mentity_mut(&entity);

        builders.get_builder_mut(name).map(|builder| {
            builder.create_entity(mentity, groups, tags)
        }).expect(format!("No template with the name {} was found.", name)[])
    }
}