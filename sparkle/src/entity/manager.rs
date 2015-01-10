use std::collections::VecMap;
use std::collections::vec_map::IterMut;
use std::iter::Filter;
use component::{Component, ComponentIndex, Store, StoreMap};
use entity::builder::{Builder, BuilderMap};
use entity::group::GroupMap;
use entity::tag::TagMap;
use entity::pool::Pool;
use entity::{Entity, MetaEntity};
use entity::event;

struct Entities {
    pool: Pool,
    mentities: VecMap<MetaEntity>,
    components: StoreMap
}

macro_rules! get_mentity_mut {
    ($entities:expr, $entity:expr) => (
        $entities.mentities.get_mut(&$entity)
                           .expect(format!("There is no meta information for {}", $entity).as_slice())
    )
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            pool: Pool::new(),
            mentities: VecMap::new(),
            components: StoreMap::new()
        }
    }
}

pub struct Manager {
    entities: Entities,
    groups: GroupMap,
    tags: TagMap,
    builders: BuilderMap,
    events: event::Queue
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            entities: Entities::new(),
            groups: GroupMap::new(),
            tags: TagMap::new(),
            builders: BuilderMap::new(),
            events: event::Queue::new()
        }
    }

    pub fn create(&mut self) -> Entity {
        let meta_entity = self.entities.pool.get();
        let entity = meta_entity.entity;
        self.entities.mentities.insert(entity, meta_entity);

        self.events.created(entity);
        entity
    }

    pub fn remove(&mut self, entity: Entity) {
        {
            let mentity = get_mentity_mut!(self.entities, entity);
            self.entities.components.remove_all(mentity);
            self.groups.clear_entity(mentity);
            self.tags.remove(mentity);
        }
        self.entities.mentities.remove(&entity);
        self.events.removed(entity);
    }

    pub fn get_mentity(&mut self, entity: Entity) -> &MetaEntity {
        self.entities.mentities.get(&entity)
                               .expect(format!("There is no meta information for {}", entity).as_slice())
    }

    pub fn filter<'a, P>(&'a mut self, filter: P) -> Filter<(usize, &mut MetaEntity), IterMut<'a, MetaEntity>, P>
        where P: FnMut(&(usize, &mut MetaEntity)) -> bool
    {
        self.entities.mentities.iter_mut().filter(filter)
    }

    pub fn attach_component<T>(&mut self, entity: Entity, component: T)
        where T: Component + ComponentIndex
    {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.entities.components.insert::<T>(mentity, component);
    }

    pub fn detach_component<T>(&mut self, entity: Entity)
        where T: Component + ComponentIndex
    {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.entities.components.remove::<T>(mentity);
    }

    #[inline]
    pub fn get_component<T>(&self, entity: Entity) -> Option<&T>
        where T: Component + ComponentIndex
    {
        self.entities.components.get_component::<T>(entity)
    }

    #[inline]
    pub fn get_component_mut<T>(&mut self, entity: Entity) -> Option<&mut T>
        where T: Component + ComponentIndex
    {
        self.entities.components.get_component_mut::<T>(entity)
    }

    pub fn get_store<T>(&self) -> Option<&Store<T>>
        where T: Component + ComponentIndex
    {
        self.entities.components.get_store::<T>()
    }

    pub fn get_store_mut<T>(&mut self) -> Option<&mut Store<T>>
        where T: Component + ComponentIndex
    {
        self.entities.components.get_store_mut::<T>()
    }

    pub fn insert_group(&mut self, group: &str, entity: Entity) {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.groups.insert(group, mentity);
    }

    pub fn remove_from_group(&mut self, group_name: &str, entity: Entity) {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.groups.remove_from(group_name, mentity);
    }

    pub fn get_from_group(&self, group_name: &str) -> Vec<Entity> {
        self.groups.get(group_name)
    }

    pub fn insert_tag(&mut self, tag: &str, entity: Entity) {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.tags.insert(tag, mentity);
    }

    pub fn remove_tag(&mut self, entity: Entity) {
        let mentity = get_mentity_mut!(self.entities, entity);

        self.tags.remove(mentity);
    }

    pub fn get_with_tag(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag)
    }

    pub fn insert_builder<T>(&mut self, name: &str, builder: T) where T: Builder {
        self.builders.insert(name, builder);
    }

    pub fn build_entity_with(&mut self, name: &str) -> Entity
    {
        let entity = self.create();
        let Manager { ref mut entities, ref mut groups, ref mut tags, ref mut builders, .. } = *self;

        let mentity = get_mentity_mut!(entities, entity);
        builders.get_builder_mut(name).map(|builder| {
            builder.create_entity(mentity, groups, tags)
        }).expect(format!("No template with the name {} was found.", name).as_slice())
    }

    pub fn pop_event(&mut self) -> Option<(event::Kind, Entity)> {
        self.events.pop()
    }
}
