use std::collections::VecMap;
use component::StoreMap;
use entity::builder::{Builder, BuilderMap};
use entity::group::GroupMap;
use entity::tag::TagMap;
use entity::pool::Pool;
use entity::{Entity, MetaEntity};
use entity::event;

macro_rules! get_mentity {
    ($metas:expr, $entity:expr) => (
        $metas.get(&$entity)
              .expect(format!("There is no meta information for {}", $entity).as_slice())
    )
}

macro_rules! get_mentity_mut {
    ($metas:expr, $entity:expr) => (
        $metas.get_mut(&$entity)
              .expect(format!("There is no meta information for {}", $entity).as_slice())
    )
}

pub struct Manager {
    pool: Pool,
    metas: VecMap<MetaEntity>,
    components: StoreMap,
    groups: GroupMap,
    tags: TagMap,
    builders: BuilderMap,
    events: event::Queue
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            pool: Pool::new(),
            metas: VecMap::new(),
            components: StoreMap::new(),
            groups: GroupMap::new(),
            tags: TagMap::new(),
            builders: BuilderMap::new(),
            events: event::Queue::new()
        }
    }

    pub fn create(&mut self) -> Entity {
        let meta_entity = self.pool.get();
        let entity = meta_entity.entity;
        self.metas.insert(entity, meta_entity);

        self.events.changed(entity);
        entity
    }

    pub fn remove(&mut self, entity: Entity) {
        let mut mentity = self.metas.remove(&entity)
                              .expect(format!("There is no meta information for {}", entity).as_slice());
        self.components.remove_all(&mut mentity);
        self.groups.clear_entity(&mut mentity);
        self.tags.remove(&mut mentity);
        self.pool.put(mentity);

        self.events.removed(entity);
    }

    pub fn insert_group(&mut self, group: &str, entity: Entity) {
        let mentity = get_mentity_mut!(self.metas, entity);

        self.groups.insert(group, mentity);
    }

    pub fn remove_from_group(&mut self, group_name: &str, entity: Entity) {
        let mentity = get_mentity_mut!(self.metas, entity);

        self.groups.remove_from(group_name, mentity);
    }

    pub fn get_from_group(&self, group_name: &str) -> Vec<Entity> {
        self.groups.get(group_name)
    }

    pub fn insert_tag(&mut self, tag: &str, entity: Entity) {
        let mentity = get_mentity_mut!(self.metas, entity);

        self.tags.insert(tag, mentity);
    }

    pub fn remove_tag(&mut self, entity: Entity) {
        let mentity = get_mentity_mut!(self.metas, entity);

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
        let Manager { ref mut metas, ref mut groups, ref mut tags, ref mut builders, .. } = *self;

        let mentity = get_mentity_mut!(metas, entity);
        builders.get_builder_mut(name).map(|builder| {
            builder.create_entity(mentity, groups, tags)
        }).expect(format!("No template with the name {} was found.", name).as_slice())
    }

    pub fn drain_events<'a>(&'a mut self) -> Box<Iterator<Item=(event::Kind, &MetaEntity)>> {
        let Manager {ref mut events, ref metas, ..} = *self;
        Box::new(events.drain().map(move |(kind, entity)| (kind, get_mentity!(metas, entity))))
    }
}
