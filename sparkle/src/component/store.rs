use std::collections::VecMap;
use std::raw::TraitObject;
use std::mem;
use entity::{Entity, MetaEntity};

use component::{Component, ComponentIndex};

pub type Store<T> = VecMap<T>;

trait AnyStore: 'static {
    fn get_type_index(&self) -> uint;
    fn remove(&mut self, entity: &Entity);
}

impl<T> AnyStore for Store<T> where T: Component + ComponentIndex {
    fn get_type_index(&self) -> uint {
        ComponentIndex::of(None::<T>)
    }

    fn remove(&mut self, entity: &Entity) {
        self.remove(entity);
    }
}

impl AnyStore {
    #[inline]
    pub fn downcast_ref<'a, T>(&'a self) -> &'a Store<T>
        where T: Component + ComponentIndex
    {
        debug_assert_eq!(self.get_type_index(), ComponentIndex::of(None::<T>));

        unsafe {
            let to: TraitObject = mem::transmute(self);
            mem::transmute(to.data)
        }
    }

    #[inline]
    pub fn downcast_mut<'a, T>(&'a mut self) -> &'a mut Store<T>
        where T: Component + ComponentIndex
    {
        debug_assert_eq!(self.get_type_index(), ComponentIndex::of(None::<T>));

        unsafe {
            let to: TraitObject = mem::transmute(self);
            mem::transmute(to.data)
        }
    }
}

pub struct StoreMap {
    stores: VecMap<Box<AnyStore>>
}

impl StoreMap {
    pub fn new() -> StoreMap {
        StoreMap {
            stores: VecMap::new()
        }
    }

    pub fn attach_component<T>(&mut self, mentity: &mut MetaEntity, component: T)
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        mentity.component_bits.insert(type_index);

        if let Some(store) = self.get_store_mut::<T>() {
            store.insert(mentity.entity, component);
            return;
        }
        self.insert_new_store_with(type_index, mentity.entity, component);
    }

    fn insert_new_store_with<T>(&mut self, index: uint, entity: Entity, component: T)
        where T: Component + ComponentIndex
    {
        let mut new_store = box VecMap::new();
        new_store.insert(entity, component);

        self.stores.insert(index, new_store);
    }

    pub fn detach_component<T>(&mut self, mentity: &mut MetaEntity)
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        mentity.component_bits.remove(&type_index);

        self.get_store_mut::<T>().map(|store| store.remove(&mentity.entity));
    }

    pub fn detach_components(&mut self, mentity: &mut MetaEntity) {
        for (type_index, store) in self.stores.iter_mut() {
            mentity.component_bits.remove(&type_index);
            store.remove(&mentity.entity);
        }
    }

    #[inline]
    pub fn get_component<T>(&self, entity: Entity) -> Option<&T>
        where T: Component + ComponentIndex
    {
        self.get_store::<T>().and_then(|store| store.get(&entity))
    }

    #[inline]
    pub fn get_component_mut<T>(&mut self, entity: Entity) -> Option<&mut T>
        where T: Component + ComponentIndex
    {
        self.get_store_mut::<T>().and_then(|store| store.get_mut(&entity))
    }

    #[inline]
    pub fn get_store<T>(&self) -> Option<&Store<T>>
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.stores.get(&type_index).map(|store| store.downcast_ref())
    }

    #[inline]
    pub fn get_store_mut<T>(&mut self) -> Option<&mut Store<T>>
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.stores.get_mut(&type_index).map(|store| store.downcast_mut())
    }
}
