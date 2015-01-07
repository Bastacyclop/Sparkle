use std::collections::HashSet;
use entity;

pub use self::manager::Manager;

pub mod manager;
pub mod filter;

pub trait System: 'static {
    fn update(&mut self, em: &mut entity::Manager, dt: f32);
}

pub trait Processor {}