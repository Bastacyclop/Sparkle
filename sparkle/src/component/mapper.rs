use std::collections::VecMap;
use std::ops::{Deref, DerefMut};
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

pub struct ComponentRef<'a, T: 'a> {
    entity: Entity,
    inner_ref: Ref<'a, VecMap<T>>
}

impl<'a, T> Deref for ComponentRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner_ref.get(&self.entity).expect("Failed to find component")
    }
}

pub struct ComponentRefMut<'a, T: 'a> {
    entity: Entity,
    inner_mut: RefMut<'a, VecMap<T>>
}

impl<'a, T> Deref for ComponentRefMut<'a, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner_mut.get(&self.entity).expect("Failed to find component")
    }
}

impl<'a, T> DerefMut for ComponentRefMut<'a, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner_mut.get_mut(&self.entity).expect("Failed to find component")
    }
}

pub struct Mapper {
    stores: VecMap<Box<AnyStore>>
}

impl Mapper {
    pub fn new() -> Mapper {
        Mapper {
            stores: VecMap::new()
        }
    }

    pub fn insert<T>(&mut self, mentity: &mut MetaEntity, component: T)
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        mentity.components.insert(type_index);

        self.ensure::<T>();
        self.get_store_mut::<T>().unwrap().insert(mentity.entity, component);
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
    pub fn get<'a, T>(&'a self, entity: Entity) -> Option<ComponentRef<'a, T>>
        where T: Component + ComponentIndex
    {
        self.get_store::<T>().map(|store| {
            ComponentRef {
                entity: entity,
                inner_ref: store
            }
        })
    }

    #[inline]
    pub fn get_mut<'a, T>(&'a self, entity: Entity) -> Option<ComponentRefMut<'a, T>>
        where T: Component + ComponentIndex
    {
        self.get_store_mut::<T>().map(|store| {
            ComponentRefMut {
                entity: entity,
                inner_mut: store
            }
        })
    }

    #[inline]
    pub fn get_store<'a, T>(&'a self) -> Option<Ref<'a, VecMap<T>>>
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.stores.get(&type_index).map(|store| store.downcast_ref().borrow())
    }

    #[inline]
    pub fn get_store_mut<'a, T>(&'a self) -> Option<RefMut<'a, VecMap<T>>>
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        self.stores.get(&type_index).map(|store| store.downcast_ref().borrow_mut())
    }

    pub fn remove<T>(&mut self, mentity: &mut MetaEntity)
        where T: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<T>);
        mentity.components.remove(&type_index);

        self.get_store_mut::<T>().map(|mut store| store.remove(&mentity.entity));
    }

    pub fn remove_all(&mut self, mentity: &mut MetaEntity) {
        for (type_index, store) in self.stores.iter_mut() {
            mentity.components.remove(&type_index);
            store.remove(&mentity.entity);
        }
    }
}

#[doc(hidden)]
pub mod private {
    use super::Mapper;
    use entity::MetaEntity;

    pub fn forget(mapper: &mut Mapper, mentity: &MetaEntity) {
        for type_index in mentity.components.iter() {
            mapper.stores.get_mut(&type_index)
                            .map(|mut store| store.remove(&mentity.entity));
        }
    }
}
