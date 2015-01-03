#![crate_name = "sparkle"]
#![feature(plugin_registrar, macro_rules, unboxed_closures)]
#![unstable]

extern crate rustc;
extern crate syntax;

pub use entity::{Entity, MetaEntity};
pub use entity::Manager as EntityManager;
pub use system::{System, Processor, FramerateSystem};
pub use system::Manager as SystemManager;
pub use space::Space;

pub mod entity;
pub mod component;
pub mod group;
pub mod system;
pub mod space;