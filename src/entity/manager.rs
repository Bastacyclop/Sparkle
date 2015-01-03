use std::collections::{VecMap, RingBuf};
use component::{Component, ComponentIndex, StoreMap};
use entity::{Pool, Entity, MetaEntityMap};

pub struct Manager {
    pool: Pool,
    mentities: MetaEntityMap,
    removed: RingBuf<Entity>,
    components: StoreMap
}

impl Manager{
    pub fn new() -> Manager {
        Manager {
            pool: Pool::new(),
            mentities: MetaEntityMap::new(),
            removed: RingBuf::new(),
            components: StoreMap::new()
        }
    }

    pub fn create(&mut self) -> Entity {
        let meta_entity = self.pool.get();
        let entity = meta_entity.entity;

        self.mentities.0.borrow_mut().insert(entity, meta_entity);
        entity
    }

    pub fn remove(&mut self, entity: &Entity) {
        self.removed.push_back(*entity);
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) 
        where T: Component + ComponentIndex 
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.components.attach_component(entity, component);

        self.mentities.apply_to(entity, |mentity| { 
            mentity.component_bits.insert(type_index); 
        });
    }

    pub fn detach_component<T>(&mut self, entity: &Entity) 
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.components.detach_component::<T>(entity);

        self.mentities.apply_to(entity, |mentity| {
            mentity.component_bits.remove(&type_index);
        });
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
        while let Some(removed) = self.removed.pop_back() {
            self.mentities.0.borrow_mut().remove(&removed).map(|mentity| self.pool.put(mentity));
            self.components.detach_components(&removed);
        }
    }
}