use entity::{self, MetaEntity};
use component::StoreMap;

pub use self::manager::Manager;
pub use self::filter::Filter;

pub mod manager;
pub mod filter;

pub trait System: 'static {
    fn update(&mut self, _em: &mut entity::Manager, component: &mut StoreMap, _dt: f32) {}
    fn fixed_update(&mut self, _em: &mut entity::Manager, component: &mut StoreMap) {}

    fn on_entity_changed(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_removed(&mut self, _mentity: &MetaEntity) {}
}
