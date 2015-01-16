///! The Component related types

use std::collections::VecMap;
use std::ops::{Deref, DerefMut};
use std::cell::{RefCell, Ref, RefMut};
use std::raw::TraitObject;
use std::mem;
use entity::{Entity, MetaEntity};

/// The trait for components.
///
/// You shouldn't implement this manually, instead use the `#[sparkle_component]` macro.
pub trait Component: 'static {}

/// A Component trait extension.
///
/// You shouldn't implement this manually, instead use the `#[sparkle_component]` macro.
// FIXME: Change this to a more generic trait
pub trait ComponentIndex: Component {
    /// Returns the storage index of the component.
    fn of(_: Option<Self>) -> usize;
}

/// A store of components of the same type.
///
/// Basically a vector of components where
/// each index corresponds to an `Entity`.
pub type ComponentStore<C> = RefCell<VecMap<C>>;

/// A component mapper.
///
/// Basically a vector of component stores where
/// each index corresponds to a specific component type.
pub struct ComponentMapper {
    stores: VecMap<Box<AnyStore>>
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
        where C: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<C>);
        mentity.components.insert(type_index);

        self.ensure::<C>();
        self.get_store_mut::<C>().unwrap().insert(mentity.entity, component);
    }

    /// Ensures a component store presence.
    pub fn ensure<C>(&mut self)
        where C: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<C>);

        if !self.stores.contains_key(&type_index) {
            let empty: Box<ComponentStore<C>> = Box::new(RefCell::new(VecMap::new()));
            self.stores.insert(type_index, empty);
        }
    }

    /// Returns a reference to an entity's component, if it exists.
    #[inline]
    pub fn get<'a, C>(&'a self, entity: Entity) -> Option<ComponentRef<'a, C>>
        where C: Component + ComponentIndex
    {
        self.get_store::<C>().map(|store| {
            ComponentRef {
                entity: entity,
                inner_ref: store
            }
        })
    }

    /// Returns a mutable reference to an entity's component, if it exists.
    #[inline]
    pub fn get_mut<'a, C>(&'a self, entity: Entity) -> Option<ComponentRefMut<'a, C>>
        where C: Component + ComponentIndex
    {
        self.get_store_mut::<C>().map(|store| {
            ComponentRefMut {
                entity: entity,
                inner_mut: store
            }
        })
    }

    /// Returns a reference to a component store, if it exists.
    #[inline]
    pub fn get_store<'a, C>(&'a self) -> Option<Ref<'a, VecMap<C>>>
        where C: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<C>);
        self.stores.get(&type_index).map(|store| store.downcast_ref().borrow())
    }

    /// Returns a mutable reference to a component store, if it exists.
    #[inline]
    pub fn get_store_mut<'a, C>(&'a self) -> Option<RefMut<'a, VecMap<C>>>
        where C: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<C>);
        self.stores.get(&type_index).map(|store| store.downcast_ref().borrow_mut())
    }

    /// Detaches a component from an entity and removes it from the map.
    pub fn remove<C>(&mut self, mentity: &mut MetaEntity)
        where C: Component + ComponentIndex
    {
        let type_index = ComponentIndex::of(None::<C>);
        mentity.components.remove(&type_index);

        self.get_store_mut::<C>().map(|mut store| store.remove(&mentity.entity));
    }

    /// Detaches all components from an entity and removes them from the map.
    pub fn remove_all(&mut self, mentity: &mut MetaEntity) {
        for (type_index, store) in self.stores.iter_mut() {
            mentity.components.remove(&type_index);
            store.remove(&mentity.entity);
        }
    }
}

/// A `ComponentStore` of any component type.
trait AnyStore: 'static {
    fn get_type_index(&self) -> usize;
    fn remove(&mut self, entity: &Entity);
}

impl<C> AnyStore for ComponentStore<C>
    where C: Component + ComponentIndex
{
    fn get_type_index(&self) -> usize {
        ComponentIndex::of(None::<C>)
    }

    fn remove(&mut self, entity: &Entity) {
        self.borrow_mut().remove(entity);
    }
}

impl AnyStore {
    /// Downcasts the `AnyStore` to a reference to a 
    /// `ComponentStore` of a specific component type.
    ///
    /// The asked component type must match the original one.
    #[inline]
    pub fn downcast_ref<'a, C>(&'a self) -> &'a ComponentStore<C>
        where C: Component + ComponentIndex
    {
        debug_assert_eq!(self.get_type_index(), ComponentIndex::of(None::<C>));

        unsafe {
            let to: TraitObject = mem::transmute(self);
            mem::transmute(to.data)
        }
    }
}

/// A custom reference to a component.
pub struct ComponentRef<'a, C>
    where C: Component + ComponentIndex
{
    entity: Entity,
    inner_ref: Ref<'a, VecMap<C>>
}

impl<'a, C> Deref for ComponentRef<'a, C>
    where C: Component + ComponentIndex
{
    type Target = C;

    fn deref(&self) -> &C {
        self.inner_ref.get(&self.entity).expect("Failed to find component")
    }
}

/// A custom mutable reference to a component.
pub struct ComponentRefMut<'a, C>
    where C: Component + ComponentIndex
{
    entity: Entity,
    inner_mut: RefMut<'a, VecMap<C>>
}

impl<'a, C> Deref for ComponentRefMut<'a, C>
    where C: Component + ComponentIndex
{
    type Target = C;

    fn deref(&self) -> &C {
        self.inner_mut.get(&self.entity).expect("Failed to find component")
    }
}

impl<'a, C> DerefMut for ComponentRefMut<'a, C>
    where C: Component + ComponentIndex
{
    fn deref_mut(&mut self) -> &mut C {
        self.inner_mut.get_mut(&self.entity).expect("Failed to find component")
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
                            .map(|mut store| store.remove(&mentity.entity));
        }
    }
}
