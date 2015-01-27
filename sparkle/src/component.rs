//! The component related features.

use std::collections::VecMap;
use std::raw::TraitObject;
use std::mem;
use entity::{Entity, MetaEntity};

/// The trait for components.
///
/// You shouldn't implement this manually, instead use the `#[sparkle_component]` macro.
pub trait Component: 'static {
    fn index_of() -> usize;
}

pub fn index_of<C>() -> usize
    where C: Component 
{
    <C as Component>::index_of()
}

// FIXME: must find a better way to do this
type StoreWrapper = (Box<AnyStore>, TraitObject);

/// A component mapper.
///
/// Basically a vector of component stores where
/// each index corresponds to a specific component type.
pub struct ComponentMapper {
    stores: VecMap<StoreWrapper>
}

impl ComponentMapper {
    /// Creates a new `ComponentMapper`.
    #[doc(hidden)]
    pub fn new() -> ComponentMapper {
        ComponentMapper {
            stores: VecMap::new()
        }
    }

    /// Attaches a component to an entity and inserts it into the map.
    ///
    /// If necessary, a new component store is created.
    pub fn insert<C>(&mut self, mentity: &mut MetaEntity, component: C)
        where C: Component
    {
        let type_index = index_of::<C>();
        mentity.components.insert(type_index);

        self.ensure::<C>();
        self.get_store_mut::<C>().insert(mentity.entity, component);
    }

    pub fn use_store<C: Component, S: ComponentStore<C>>(&mut self, store: S) {
        let type_index = index_of::<C>();

        if !self.stores.contains_key(&type_index) {
            let boxed = Box::new(store);
            let to: TraitObject = unsafe { mem::transmute(&*boxed as &ComponentStore<C>) };
            self.stores.insert(type_index, (boxed, to));
        } else {
            panic!("The store is already assigned");
        }
    }

    /// Ensures a default component store presence.
    pub fn ensure<C>(&mut self)
        where C: Component
    {
        let type_index = index_of::<C>();

        if !self.stores.contains_key(&type_index) {
            let boxed = Box::new(DefaultStore::new());
            let to: TraitObject = unsafe { mem::transmute(&*boxed as &ComponentStore<C>) };
            self.stores.insert(type_index, (boxed, to));
        }
    }

    /// Try to returns a reference to an entity's component, if it exists.
    #[inline]
    pub fn try_get<C>(&self, entity: Entity) -> Option<&C>
        where C: Component
    {
        self.try_get_store::<C>().and_then(|store| store.try_get(entity))
    }

    /// Returns a reference to an entity's component.
    ///
    /// This method panic if the entity doesn't have the requested component.
    #[inline]
    pub fn get<C>(&self, entity: Entity) -> &C
        where C: Component
    {
        self.try_get::<C>(entity).expect("Failed to get the component")
    }

    /// Try to returns a mutable reference to an entity's component, if it exists.
    #[inline]
    pub fn try_get_mut<C>(&mut self, entity: Entity) -> Option<&mut C>
        where C: Component
    {
        self.try_get_store_mut::<C>().and_then(|store| store.try_get_mut(entity))
    }

    /// Try to returns a mutable reference to an entity's component, if it exists.
    ///
    /// This method panic if the entity doesn't have the requested component.
    #[inline]
    pub fn get_mut<C>(&mut self, entity: Entity) -> &mut C
        where C: Component
    {
        self.try_get_mut::<C>(entity).expect("Failed to get the component")
    }

    /// Try to returns a reference to a component store, if it exists.
    #[inline]
    pub fn try_get_store<C>(&self) -> Option<&ComponentStore<C>>
        where C: Component
    {
        let type_index = index_of::<C>();
        self.stores.get(&type_index).map(|store| {
            let to = store.1;
            unsafe { mem::transmute(to) }
        })
    }

    /// Returns a reference to a component store, if it exists.
    ///
    /// This method panic if the store doesn't exist.
    #[inline]
    pub fn get_store<C>(&self) -> &ComponentStore<C>
        where C: Component
    {
        self.try_get_store::<C>().expect("Failed to get the store")
    }

    /// Try to returns a mutable reference to a component store, if it exists.
    #[inline]
    pub fn try_get_store_mut<C>(&mut self) -> Option<&mut ComponentStore<C>>
        where C: Component
    {
        let type_index = index_of::<C>();
        self.stores.get_mut(&type_index).map(|store| {
            let to = store.1;
            unsafe { mem::transmute(to) }
        })
    }

    /// Returns a mutable reference to a component store, if it exists.
    ///
    /// This method panic if the store doesn't exist.
    #[inline]
    pub fn get_store_mut<C>(&mut self) -> &mut ComponentStore<C>
        where C: Component
    {
        self.try_get_store_mut::<C>().expect("Failed to get the store")
    }

    /// Detaches a component from an entity and removes it from the map.
    pub fn remove<C>(&mut self, mentity: &mut MetaEntity)
        where C: Component
    {
        let type_index = index_of::<C>();
        mentity.components.remove(&type_index);

        self.get_store_mut::<C>().remove(mentity.entity);
    }

    /// Detaches all components from an entity and removes them from the map.
    pub fn remove_all(&mut self, mentity: &mut MetaEntity) {
        for (type_index, store) in self.stores.iter_mut() {
            mentity.components.remove(&type_index);
            store.0.remove(mentity.entity);
        }
    }
}

pub trait ComponentStore<C>: 'static
    where C: Component
{
    fn insert(&mut self, entity: Entity, component: C);
    fn remove(&mut self, entity: Entity);

    fn get(&self, component: Entity) -> &C;
    fn get_mut(&mut self, entity: Entity) -> &mut C;
    fn try_get(&self, entity: Entity) -> Option<&C>;
    fn try_get_mut(&mut self, entity: Entity) -> Option<&mut C>;
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

/// A store of components of the same type.
///
/// Basically a vector of components where
/// each index corresponds to an `Entity`.
pub struct DefaultStore<C>(VecMap<C>) where C: Component;

impl<C> DefaultStore<C>
    where C: Component
{
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
    fn get(&self, entity: Entity) -> &C {
        self.try_get(entity).expect("Failed to get component")
    }

    #[inline]
    fn try_get_mut(&mut self, entity: Entity) -> Option<&mut C> {
        self.0.get_mut(&entity)
    }

    #[inline]
    fn get_mut(&mut self, entity: Entity) -> &mut C {
        self.try_get_mut(entity).expect("Failed to get component")
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
