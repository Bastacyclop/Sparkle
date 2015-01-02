use std::collections::{VecMap, RingBuf};
use component::{Component, ComponentType, StoreMap};
use entity::{Pool, Entity, MetaEntity, Update, Observer, Record};

pub struct Manager {
    pool: Pool,
    actives: VecMap<MetaEntity>,
    removed: RingBuf<Entity>,
    components: StoreMap,
    updates_record: Record
}

impl Manager{
    pub fn new() -> Manager {
        Manager {
            pool: Pool::new(),
            actives: VecMap::new(),
            removed: RingBuf::new(),
            components: StoreMap::new(),
            updates_record: Record::new()
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        let meta_entity = self.pool.get();
        let entity = meta_entity.entity;

        self.actives.insert(entity, meta_entity);
        self.updates_record.add(Update::new_created(entity));
        entity
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        self.removed.push_back(*entity);
        self.updates_record.add(Update::new_removed(*entity));
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) where T: Component {
        let type_index = ComponentType::get_index_of::<T>();

        self.components.attach_component(entity, component);
        self.actives.get_mut(entity).map(|mentity| mentity.component_bits.insert(type_index));
        self.updates_record.add(Update::new_changed(*entity));
    }

    pub fn detach_component<T>(&mut self, entity: &Entity) where T: Component {
        let type_index = ComponentType::get_index_of::<T>();

        self.components.detach_component::<T>(entity);
        self.actives.get_mut(entity).map(|mentity| mentity.component_bits.remove(&type_index));
        self.updates_record.add(Update::new_changed(*entity));
    }

    #[inline]
    pub fn get_component<T>(&self, entity: &Entity) -> Option<&T> where T: Component {
        self.components.get_component::<T>(entity)
    }

    #[inline]
    pub fn get_mut_component<T>(&mut self, entity: &Entity) -> Option<&mut T> where T: Component {
        self.components.get_mut_component::<T>(entity)
    }

    pub fn notify_observer_and_flush<T>(&mut self, observer: &mut T) where T: Observer {
        self.updates_record.notify_and_flush(&self.actives, observer);
    }

    pub fn flush_removed(&mut self) {
        while let Some(removed) = self.removed.pop_back() {
            self.actives.remove(&removed).map(|mentity| self.pool.put(mentity));
            self.components.detach_components(&removed);
        }
    }
}