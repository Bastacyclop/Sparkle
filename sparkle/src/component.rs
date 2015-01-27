//! The component related features.

use std::collections::VecMap;
use std::raw::TraitObject;
use std::mem;
use entity::{Entity, MetaEntity};

/// The trait for components.
///
/// You shouldn't implement this manually, instead use the `#[component]` macro.
pub trait Component: 'static {
    fn index_of() -> usize;
}

pub fn index_of<C>() -> usize
    where C: Component 
{
    <C as Component>::index_of()
}

// FIXME: find a better way to do this
struct StoreWrapper(Box<AnyStore>, TraitObject);

impl StoreWrapper {
    fn new<C, S>(store: S) -> StoreWrapper
        where C: Component, S: ComponentStore<C>
    {
        let boxed = Box::new(store);
        let to = unsafe { mem::transmute(&*boxed as &ComponentStore<C>) };
        StoreWrapper(boxed, to)
    }
    
    unsafe fn downcast_ref<'a, C>(&'a self) -> &'a ComponentStore<C> {
        mem::transmute(self.1)
    }
    
    unsafe fn downcast_mut<'a, C>(&'a mut self) -> &'a mut ComponentStore<C> {
        mem::transmute(self.1)
    }
}

/// A component mapper.
///
/// Basically a vector of component stores where
/// each index corresponds to a specific component type.
pub struct ComponentMapper {
    stores: VecMap<StoreWrapper>
}

impl ComponentMapper {
    /// Creates a new `ComponentMapper`.
    pub fn new() -> ComponentMapper {
        ComponentMapper {
            stores: VecMap::new()
        }
    }

    /// Attaches a component to an entity and inserts it into the mapper.
    ///
    /// If necessary, a default component store is created.
    pub fn insert<C>(&mut self, mentity: &mut MetaEntity, component: C)
        where C: Component
    {
        let type_index = index_of::<C>();
        mentity.components.insert(type_index);

        self.ensure::<C>();
        self.get_store_mut::<C>().insert(mentity.entity, component);
    }

    /// Uses the given component store for a certain type of components.
    ///
    /// This should be done when setting up the mapper, before it's actually used.
    /// Panics if another store is already used for this type of components.
    pub fn use_store<C, S>(&mut self, store: S)
        where C: Component, S: ComponentStore<C>
    {
        let type_index = index_of::<C>();

        if !self.stores.contains_key(&type_index) {
            self.stores.insert(type_index, StoreWrapper::new(store));
        } else {
            panic!("a store is already used for this type of components");
        }
    }

    /// Ensures that some store is used for this type of components.
    ///
    /// If necessary, a default component store is created.
    pub fn ensure<C>(&mut self)
        where C: Component
    {
        let type_index = index_of::<C>();

        if !self.stores.contains_key(&type_index) {
            let default = DefaultStore::<C>::new();
            self.stores.insert(type_index, StoreWrapper::new(default));
        }
    }

    /// Tries to return a reference to an entity's component, if it exists.
    #[inline]
    pub fn try_get<C>(&self, entity: Entity) -> Option<&C>
        where C: Component
    {
        self.try_get_store::<C>().and_then(|store| store.try_get(entity))
    }

    /// Returns a reference to an entity's component.
    ///
    /// Panics if the entity doesn't have the requested component.
    #[inline]
    pub fn get<C>(&self, entity: Entity) -> &C
        where C: Component
    {
        self.try_get::<C>(entity).expect("failed to get the component")
    }

    /// Tries to return a mutable reference to an entity's component, if it exists.
    #[inline]
    pub fn try_get_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
        where C: Component
    {
        self.try_get_store_mut::<C>().and_then(|store| store.try_get_mut(entity))
    }

    /// Tries to return a mutable reference to an entity's component, if it exists.
    ///
    /// Panics if the entity doesn't have the requested component.
    #[inline]
    pub fn get_mut<C>(&mut self, entity: Entity) -> &mut C
        where C: Component
    {
        self.try_get_mut::<C>(entity).expect("failed to get the component")
    }

    /// Tries to return a reference to a component store, if it exists.
    #[inline]
    pub fn try_get_store<C>(&self) -> Option<&ComponentStore<C>>
        where C: Component
    {
        let type_index = index_of::<C>();
        self.stores.get(&type_index).map(|store| unsafe {
             store.downcast_ref()
        })
    }

    /// Returns a reference to a component store, if it exists.
    ///
    /// Panics if the store doesn't exist.
    #[inline]
    pub fn get_store<C>(&self) -> &ComponentStore<C>
        where C: Component
    {
        self.try_get_store::<C>().expect("failed to get the store")
    }

    /// Tries to return a mutable reference to a component store, if it exists.
    #[inline]
    pub fn try_get_store_mut<C>(&mut self) -> Option<&mut ComponentStore<C>>
        where C: Component
    {
        let type_index = index_of::<C>();
        self.stores.get_mut(&type_index).map(|store| unsafe {
             store.downcast_mut()
        })
    }

    /// Returns a mutable reference to a component store, if it exists.
    ///
    /// Panics if the store doesn't exist.
    #[inline]
    pub fn get_store_mut<C>(&mut self) -> &mut ComponentStore<C>
        where C: Component
    {
        self.try_get_store_mut::<C>().expect("failed to get the store")
    }

    /// Detaches a component from an entity and removes it from the mapper.
    pub fn remove<C>(&mut self, mentity: &mut MetaEntity)
        where C: Component
    {
        let type_index = index_of::<C>();
        mentity.components.remove(&type_index);

        self.get_store_mut::<C>().remove(mentity.entity);
    }

    /// Detaches all components from an entity and removes them from the mapper.
    pub fn remove_all(&mut self, mentity: &mut MetaEntity) {
        for (type_index, store) in self.stores.iter_mut() {
            mentity.components.remove(&type_index);
            store.0.remove(mentity.entity);
        }
    }
}

/// A store of components of the same type.
pub trait ComponentStore<C>: 'static
    where C: Component
{
    /// Inserts an entity's component into the store.
    fn insert(&mut self, entity: Entity, component: C);
    /// Removes an entity's component from the store.
    fn remove(&mut self, entity: Entity);
    /// Tries to return a reference to an entity's component.
    fn try_get(&self, entity: Entity) -> Option<&C>;
    /// Tries to return a mutable reference to an entity's component.
    fn try_get_mut(&mut self, entity: Entity) -> Option<&mut C>;
    
    /// Returns a reference to an entity's component.
    ///
    /// Panics if the entity doesn't have the requested component.
    #[inline]
    fn get(&self, entity: Entity) -> &C {
        self.try_get(entity).expect("failed to get component")
    }
    /// Returns a mutable reference to an entity's component.
    ///
    /// Panics if the entity doesn't have the requested component.
    #[inline]
    fn get_mut(&mut self, entity: Entity) -> &mut C {
        self.try_get_mut(entity).expect("failed to get component")
    }
}

/// A `ComponentStore` of any component type.
trait AnyStore: 'static {
    fn remove(&mut self, entity: Entity);
}

#[old_impl_check]
impl<S, C> AnyStore for S
    where C: Component, S: ComponentStore<C>
{
    fn remove(&mut self, entity: Entity) {
        self.remove(entity);
    }
}

/// The default `ComponentStore`.
///
/// Basically a vector of components where
/// each index corresponds to an `Entity`.
pub struct DefaultStore<C>(VecMap<C>) where C: Component;

impl<C> DefaultStore<C>
    where C: Component
{
    /// Creates a new `DefaultStore`.
    pub fn new() -> DefaultStore<C> {
        DefaultStore(VecMap::new())
    }
}

impl<C> ComponentStore<C> for DefaultStore<C>
    where C: Component
{
    #[inline]
    fn insert(&mut self, entity: Entity, component: C) {
        self.0.insert(entity, component);
    }

    #[inline]
    fn remove(&mut self, entity: Entity) {
        self.0.remove(&entity);
    }

    #[inline]
    fn try_get(&self, entity: Entity) -> Option<&C> {
        self.0.get(&entity)
    }

    #[inline]
    fn try_get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.0.get_mut(&entity)
    }
}

#[doc(hidden)]
pub mod private {
    use super::ComponentMapper;
    use entity::MetaEntity;

    /// Forgets an entity, removing it from the `ComponentMapper`
    /// without touching the meta entity data.
    pub fn forget(mapper: &mut ComponentMapper, mentity: &MetaEntity) {
        for type_index in mentity.components.iter() {
            mapper.stores.get_mut(&type_index)
                         .map(|mut store| store.0.remove(mentity.entity));
        }
    }
}
