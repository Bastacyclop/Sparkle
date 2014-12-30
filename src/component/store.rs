use std::collections::VecMap;
use entity::Entity;
use component::{Component, ComponentType, ComponentRefExt, ComponentMutRefExt};

pub type Store = VecMap<Box<Component>>;

pub struct StoreMap {
    stores: VecMap<Store>
}

impl StoreMap {
    pub fn new() -> StoreMap {
        StoreMap {
            stores: VecMap::new()
        }
    }

    pub fn attach_component<T>(&mut self, entity: &Entity, component: T) where T: Component {
        let type_index = ComponentType::get_index_of::<T>();
        let boxed_component = box component;

        if let Some(store) = self.stores.get_mut(&type_index) {
            store.insert(*entity, boxed_component);
            return;
        }
        self.insert_new_store_with(type_index, entity, boxed_component);
    }

    fn insert_new_store_with(&mut self, index: uint, entity: &Entity, component: Box<Component>) {
        let mut new_store = VecMap::new();
        new_store.insert(*entity, component);

        self.stores.insert(index, new_store);
    }

    pub fn detach_component<T>(&mut self, entity: &Entity) where T: Component {
        let type_index = ComponentType::get_index_of::<T>();
        self.stores.get_mut(&type_index).map(|store| store.remove(entity));
    }

    pub fn detach_components(&mut self, entity: &Entity) {
        for (_index, store) in self.stores.iter_mut() {
            store.remove(entity);
        }
    }

    pub fn has_component<T>(&self, entity: &Entity) -> bool where T: Component {
        let type_index = Component::get_type_index(None::<T>);
        self.stores.get(&type_index).map(|store| store.get(entity)).is_some()
    }

    #[inline]
    pub fn get_component<T>(&self, entity: &Entity) -> Option<&T> where T: Component {
        let type_index = ComponentType::get_index_of::<T>();

        self.stores.get(&type_index).and_then(|store| {
            store.get(entity)
        }).map(|component| unsafe { component.downcast_ref() })
    }

    #[inline]
    pub fn get_mut_component<T>(&mut self, entity: &Entity) -> Option<&mut T> where T: Component {
        let type_index = ComponentType::get_index_of::<T>();

        self.stores.get_mut(&type_index).and_then(|store| {
            store.get_mut(entity)
        }).map(|component| unsafe { component.downcast_mut() })
    }
}