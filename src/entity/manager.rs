use std::collections::{VecMap, RingBuf};
use component::{Component, ComponentIndex, StoreMap};
use entity::{TagMap, Pool, Entity, MetaEntity, Event, Observer, Queue};

struct Entities {
    pub pool: Pool,
    pub actives: VecMap<MetaEntity>,
    pub removed: RingBuf<Entity>,
    pub tags: TagMap
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            pool: Pool::new(),
            actives: VecMap::new(),
            removed: RingBuf::new(),
            tags: TagMap::new()
        }
    }
}

pub struct Manager {
    entities: Entities,
    components: StoreMap,
    events_record: Queue
}

impl Manager{
    pub fn new() -> Manager {
        Manager {
            entities: Entities::new(),
            components: StoreMap::new(),
            events_record: Queue::new()
        }
    }

    pub fn create(&mut self) -> Entity {
        let meta_entity = self.entities.pool.get();
        let entity = meta_entity.entity;

        self.entities.actives.insert(entity, meta_entity);
        self.events_record.add(Event::new_created(entity));
        entity
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.entities.removed.push_back(*entity);
        self.events_record.add(Event::new_removed(*entity));
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        let type_index = ComponentIndex::of(None::<T>);

        self.components.attach_component(entity, component);
        self.entities.actives.get_mut(entity).map(|mentity| mentity.component_bits.insert(type_index));
        self.events_record.add(Event::new_changed(*entity));
    }

    pub fn detach_component<T>(&mut self, entity: &Entity) 
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);

        self.components.detach_component::<T>(entity);
        self.entities.actives.get_mut(entity).map(|mentity| mentity.component_bits.remove(&type_index));
        self.events_record.add(Event::new_changed(*entity));
    }

    #[inline]
    pub fn get_component<T>(&self, entity: &Entity) -> Option<&T> 
        where T: Component + ComponentIndex 
    {
        self.components.get_component::<T>(entity)
    }

    #[inline]
    pub fn get_mut_component<T>(&mut self, entity: &Entity) -> Option<&mut T> 
        where T: Component + ComponentIndex 
    {
        self.components.get_mut_component::<T>(entity)
    }

    pub fn set_tag(&mut self, tag: &str, entity: &Entity) {
        self.entities.tags.set_tag(tag, entity);
    }

    pub fn unset_tag(&mut self, entity: &Entity) {
        self.entities.tags.unset_tag(entity);
    }

    pub fn get_with_tag(&mut self, tag: &str) -> Option<Entity> {
        self.entities.tags.get_with_tag(tag)
    }

    pub fn flush_removed(&mut self) {
        while let Some(removed) = self.entities.removed.pop_back() {
            self.entities.actives.remove(&removed).map(|mentity| self.entities.pool.put(mentity));
            self.entities.tags.unset_tag(&removed);
            self.components.detach_components(&removed);
        }
    }
}