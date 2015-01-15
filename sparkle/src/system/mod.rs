use entity::{self, MetaEntity};
use component;

pub use self::mapper::Mapper;
pub use self::filter::Filter;

pub mod mapper;
pub mod filter;

pub trait System: 'static {
    fn update(&mut self, _em: &mut entity::Mapper, _component: &mut component::Mapper, _dt: f32) {}
    fn fixed_update(&mut self, _em: &mut entity::Mapper, _component: &mut component::Mapper) {}

    fn on_entity_changed(&mut self, _mentity: &MetaEntity) {}
    fn on_entity_removed(&mut self, _mentity: &MetaEntity) {}
}
