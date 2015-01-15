//! # **Sparkle Entity Component System**
//!
//! The Sparkle library provides building blocks to build games
//! with an ECS approach.
//!
//! ## **Entities**
//!
//! [Entities](entity/index.html) are simple identifiers. However you can refer to them
//! by attaching them [tags](entity/tag/index.html) and [groups](entity/group/index.html).
//! All of these informations are contained in [MetaEntities](entity/struct.MetaEntity.html).
//!
//! ## **Components**
//!
//! [Components](component/index.html) are bags of data, they describe the state of entities 
//! in the game world.
//!
//! ## **Systems**
//!
//! [Systems](system/index.html) have the only purpose of updating entities according to their components. Each system
//! can process as many as components as they want and can communicate through 
//! [commands](command/index.html) and [blackboard](blackboard/index.html).
//! 
//! ## **Space**
//! 
//! A [space](space/index.html) regroup an [EntityMapper](entity/struct.EntityMapper.html), 
//! [ComponentMapper](entity/struct.ComponentMapper.html) and a [SystemMapper](system/struct.SystemMapper.html).
//! Each space represent a independant part of your game world.

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

/// The Sparkle Prelude
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
