use std::raw::TraitObject;
use std::mem;
pub use self::store::{Store, StoreMap};

pub mod deriving;
pub mod store;

pub trait Component: 'static + Send + Sync {
    fn get_type_index(_: Option<Self>) -> uint;
}

pub trait ComponentRefExt<'a> {
    unsafe fn downcast_ref<T: 'static>(self) -> &'a T;
}

impl<'a> ComponentRefExt<'a> for &'a Component {
    unsafe fn downcast_ref<T: 'static>(self) -> &'a T {
        let to: TraitObject = mem::transmute(self);
        mem::transmute(to.data)
    }
}

pub trait ComponentMutRefExt<'a> {
    unsafe fn downcast_mut<T: 'static>(self) -> &'a mut T;
}

impl<'a> ComponentMutRefExt<'a> for &'a mut Component {
    unsafe fn downcast_mut<T: 'static>(self) -> &'a mut T {
        let to: TraitObject = mem::transmute(self);
        mem::transmute(to.data)
    }
}

#[deriving(Copy)]
pub struct ComponentType;

impl ComponentType {
    #[inline]
    pub fn get_index_of<T>() -> uint where T: Component {
        Component::get_type_index(None::<T>)
    }
}