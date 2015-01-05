use std::collections::VecMap;
use component::{Component, ComponentIndex, StoreMap};
use entity::{Pool, Entity, MetaEntity};

pub struct Manager {
    pool: Pool,
    mentities: VecMap<MetaEntity>,
    components: StoreMap
}

impl Manager{
    pub fn new() -> Manager {
        Manager {
            pool: Pool::new(),
            mentities: VecMap::new(),
            components: StoreMap::new()
        }
    }

    pub fn get_mentity(&self, entity: &Entity) -> &MetaEntity {
        self.mentities.get(entity)
                      .expect(format!("There is no meta information for {}", entity)[])
    }

    pub fn get_mentity_mut(&mut self, entity: &Entity) -> &mut MetaEntity {
        self.mentities.get_mut(entity)
                      .expect(format!("There is no meta information for {}", entity)[])
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
}