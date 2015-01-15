//! # Sparkle Entity Component System
//!
//! The Sparkle library provides building blocks to build games
//! with an ECS approach.
//!
//! ## Entities
//!
//! Entities are some kind of objects, represented by a simple identifier.
//! They can be [tagged](entity/tag/index.html)
//! and organised in [groups](entity/group/index.html).
//!
//! ## Components
//!
//!
//!
//! ## Systems
//!
//!
//!
//! ## Other
//!
//!

#![crate_name = "sparkle"]
#![unstable]
#![allow(unstable)]
#![feature(slicing_syntax, unboxed_closures)]
#![feature(box_syntax)]

pub use blackboard::{Blackboard, SharedBlackboard};
pub use blackboard::Entry as BlackboardEntry;
pub use space::Space;

pub use component::ComponentMapper;

pub use entity::{Entity, MetaEntity, EntityMapper};

pub use system::{System, SystemMapper, Filter};

pub use command::{Command, CommandSender};

pub mod entity;
pub mod component;
pub mod system;
pub mod space;
pub mod command;
pub mod blackboard;

pub mod prelude {
    pub use {
        Blackboard, SharedBlackboard, BlackboardEntry,
        Space,
        Entity, EntityMapper,
        System, SystemMapper,
        ComponentMapper,
        Filter,
        Command
    };
}
