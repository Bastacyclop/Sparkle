use std::collections::VecMap;
use std::cell::{RefCell, Ref, RefMut};
use std::raw::TraitObject;
use std::mem;
use entity::{Entity, MetaEntity};

use component::{Component, ComponentIndex};

pub type Store<T> = RefCell<VecMap<T>>;

trait AnyStore: 'static {
    fn get_type_index(&self) -> usize;
    fn remove(&mut self, entity: &Entity);
}

impl<T> AnyStore for Store<T> where T: Component + ComponentIndex {
    fn get_type_index(&self) -> usize {
        ComponentIndex::of(None::<T>)
    }

    fn remove(&mut self, entity: &Entity) {
        self.borrow_mut().remove(entity);
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

    pub fn insert<T>(&mut self, mentity: &mut MetaEntity, component: T)
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        mentity.component_bits.insert(type_index);

        self.ensure::<T>();
        if let Some(mut store) = self.get_mut::<T>() {
            store.insert(mentity.entity, component);
            return;
        }
    }

    pub fn ensure<T>(&mut self)
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);

        if !self.stores.contains_key(&type_index) {
            let empty: Box<Store<T>> = Box::new(RefCell::new(VecMap::new()));
            self.stores.insert(type_index, empty);
        }
    }

    #[inline]
    pub fn get<'a, T>(&'a self) -> Option<Ref<'a, VecMap<T>>>
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.stores.get(&type_index).map(|store| store.downcast_ref().borrow())
    }

    #[inline]
    pub fn get_mut<'a, T>(&'a self) -> Option<RefMut<'a, VecMap<T>>>
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.stores.get(&type_index).map(|store| store.downcast_ref().borrow_mut())
    }

    pub fn remove<T>(&mut self, mentity: &mut MetaEntity)
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        mentity.component_bits.remove(&type_index);

        self.get_mut::<T>().map(|mut store| store.remove(&mentity.entity));
    }

    pub fn remove_all(&mut self, mentity: &mut MetaEntity) {
        for (type_index, store) in self.stores.iter_mut() {
            mentity.component_bits.remove(&type_index);
            store.remove(&mentity.entity);
        }
    }
}

#[doc(hidden)]
pub mod private {
    use super::StoreMap;
    use entity::MetaEntity;

    pub fn forget(store_map: &mut StoreMap, mentity: &MetaEntity) {
        for type_index in mentity.component_bits.iter() {
            store_map.stores.get_mut(&type_index)
                            .map(|mut store| store.remove(&mentity.entity));
        }
    }
}
