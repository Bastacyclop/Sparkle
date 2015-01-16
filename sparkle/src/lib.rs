//! # **Sparkle Entity Component System**
//!
//! The Sparkle library provides building blocks to build games
//! with an ECS approach.
//!
//! ## **Entities**
//!
//! [Entities](entity/index.html) are simple identifiers. However, you can refer to them
//! by attaching them [tags](entity/index.html#identification-of-entities-using-tags)
//! and [groups](entity/index.html#identification-of-entities-using-groups).
//! All of these informations are contained in [MetaEntities](entity/struct.MetaEntity.html).
//!
//! ## **Components**
//!
//! [Components](component/index.html) are bags of data, they describe the state of entities 
//! in the game world.
//!
//! ## **Systems**
//!
//! [Systems](system/index.html) have the only purpose of updating entities
//! according to their components. Each system can process as many components as they want
//! and can communicate through the [commands](command/index.html)
//! and the [blackboard](blackboard/index.html).
//! 
//! ## **Space**
//! 
//! A [space](space/index.html) regroups an [EntityMapper](entity/struct.EntityMapper.html), 
//! a [ComponentMapper](component/struct.ComponentMapper.html)
//! and a [SystemMapper](system/struct.SystemMapper.html).
//! Each space represents an independent part of your game world.

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

/// The Sparkle prelude.
///
/// This prelude is useful to quickly import everything you need into your local scope:
///
/// ````ignore
/// use sparkle::prelude::*;
///
/// // Your stuff
/// ````
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
