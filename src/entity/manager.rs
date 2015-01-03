use std::collections::{VecMap, RingBuf};
use component::{Component, ComponentIndex, StoreMap};
use entity::{Pool, Entity, MetaEntity, Event, Observer, Queue};

struct Entities {
    pub pool: Pool,
    pub actives: VecMap<MetaEntity>,
    pub removed: RingBuf<Entity>,
}

impl Entities {
    pub fn new() -> Entities {
        Entities {
            pool: Pool::new(),
            actives: VecMap::new(),
            removed: RingBuf::new(),
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

    pub fn flush_removed(&mut self) {
        while let Some(removed) = self.entities.removed.pop_back() {
            self.entities.actives.remove(&removed).map(|mentity| self.entities.pool.put(mentity));
            self.components.detach_components(&removed);
        }
    }
}