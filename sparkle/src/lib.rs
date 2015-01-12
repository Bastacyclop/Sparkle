#![crate_name = "sparkle"]
#![unstable]
#![allow(unstable)]
#![feature(slicing_syntax, unboxed_closures)]
#![feature(box_syntax)]

#[macro_use] extern crate split_access;

pub use blackboard::Blackboard;
pub use space::Space;

pub use entity::{Entity, MetaEntity};
pub use entity::Manager as EntityManager;

pub use system::{System, Filter};
pub use system::Manager as SystemManager;

pub use command::{Command, CommandSender};

pub mod entity;
pub mod component;
pub mod system;
pub mod space;
pub mod command;
pub mod blackboard;