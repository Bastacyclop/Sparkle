use std::raw::TraitObject;
use std::mem;
pub use self::store::{Store, StoreMap};

pub mod deriving;
pub mod store;

pub trait Component: 'static + Send + Sync {}

// FIXME: Change this to a more generic trait
pub trait ComponentIndex: Component {
    fn of(_: Option<Self>) -> uint;
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