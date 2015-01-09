#![crate_name = "sparkle"]
#![unstable]
#![allow(unstable)]
#![feature(slicing_syntax, unboxed_closures)]


pub use space::Space;

pub use entity::{Entity, MetaEntity};
pub use entity::Manager as EntityManager;
pub use entity::Builder as EntityBuilder;

pub use system::{Filter, Processor};
pub use system::Manager as SystemManager;
pub use system::Framerate as FramerateSystem;
pub use system::FixedRate as FixedRateSystem;

pub use command::{Command, CommandSender};
pub use command::CreateEntity as CreateEntityCommand;
pub use command::RemoveEntity as RemoveEntityCommand;


pub mod entity;
pub mod component;
pub mod system;
pub mod space;
pub mod command;
