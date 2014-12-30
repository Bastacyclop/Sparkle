use std::collections::HashSet;
use entity::Entity;
use entity::Observer as EntityObserver;
use entity::Manager as EntityManager;
pub use self::manager::Manager;
pub use self::filter::Filter;
pub use self::interval::FramerateSystem;

pub mod manager;
pub mod filter;
pub mod interval;

pub trait System: 'static + EntityObserver {
    fn process(&mut self, em: &mut EntityManager);
}

pub trait Processor: 'static {
    fn before_processing(&mut self) {}
    fn process(&mut self, em: &mut EntityManager, entities: &HashSet<Entity>);
    fn after_processing(&mut self) {}
}