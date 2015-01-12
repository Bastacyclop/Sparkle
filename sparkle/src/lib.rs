#![crate_name = "sparkle"]
#![unstable]
#![allow(unstable)]
#![feature(slicing_syntax, unboxed_closures)]
#![feature(box_syntax)]

pub use blackboard::{Blackboard, SharedBlackboard};
pub use blackboard::Entry as BlackboardEntry;
pub use space::Space;

pub use component::Mapper as ComponentMapper;

pub use entity::{Entity, MetaEntity};
pub use entity::Mapper as EntityMapper;

pub use system::{System, Filter};
pub use system::Mapper as SystemMapper;

pub use command::{Command, CommandSender};

pub mod entity;
pub mod component;
pub mod system;
pub mod space;
pub mod command;
pub mod blackboard;