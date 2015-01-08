use entity;
use entity::MetaEntity;

pub use self::manager::Manager;

pub mod manager;
pub mod filter;

pub trait System: 'static {
    fn update(&mut self, em: &mut entity::Manager, dt: f32);

    fn on_entity_created(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_changed(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_removed(&mut self, _mentity: &MetaEntity) {}
}

pub trait Processor {
    fn update(&mut self, em: &mut entity::Manager, dt: f32);

    fn on_entity_created(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_changed(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_removed(&mut self, _mentity: &MetaEntity) {}
}
