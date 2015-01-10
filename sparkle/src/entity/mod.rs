use std::collections::{HashSet, BitvSet};

pub use self::manager::Manager;
pub use self::builder::Builder;

pub mod pool;
pub mod manager;
pub mod group;
pub mod tag;
pub mod builder;
pub mod event;

pub type Entity = usize;

#[derive(PartialEq, Eq, Clone)]
pub struct MetaEntity {
    pub entity: Entity,
    pub tag: Option<String>,
    pub groups: HashSet<String>,
    pub component_bits: BitvSet
}

impl MetaEntity {
    pub fn new(entity: Entity) -> MetaEntity {
        MetaEntity {
            entity: entity,
            tag: None,
            groups: HashSet::new(),
            component_bits: BitvSet::new()
        }
    }

    pub fn reset(mut self) -> MetaEntity {
        self.component_bits.clear();
        self.tag = None;
        self.groups.clear();

        self
    }
}
