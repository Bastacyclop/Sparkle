use std::collections::HashSet;
use entity::Entity;
use entity::Observer as EntityObserver;
use space::SpaceProxy;
pub use self::manager::Manager;
pub use self::filter::Filter;
pub use self::interval::FramerateSystem;

pub mod manager;
pub mod filter;
pub mod interval;

pub trait System: 'static + EntityObserver {
    fn process<'a>(&mut self, em: &mut SpaceProxy<'a>, dt: f32);
}

pub trait Processor: 'static {
    fn before_processing(&mut self) {}
    fn process<'a>(&mut self, space: &mut SpaceProxy<'a>, entities: &HashSet<Entity>, dt: f32);
    fn after_processing(&mut self) {}
}