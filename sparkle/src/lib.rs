#![crate_name = "sparkle"]
#![unstable]
#![allow(unstable)]
#![feature(slicing_syntax, unboxed_closures)]
#![feature(box_syntax)]

#[macro_use] extern crate split_access;

pub use space::Space;

pub use entity::{Entity, MetaEntity};

pub use system::{System, Filter};
pub use system::Manager as SystemManager;

pub use command::{Command, CommandSender};
pub use command::CreateEntity as CreateEntityCommand;
pub use command::RemoveEntity as RemoveEntityCommand;

pub mod entity;
pub mod component;
pub mod system;
pub mod space;
pub mod command;
pub mod blackboard;
