use std::collections::BitvSet;
pub use self::pool::Pool;
pub use self::manager::Manager;
pub use self::update::{Update, Record, Observer};

pub mod pool;
pub mod update;
pub mod manager;

pub type Entity = uint;

#[deriving(PartialEq, Eq, Clone, Hash)]
pub struct MetaEntity {
    pub entity: Entity,
    pub component_bits: BitvSet
}

impl MetaEntity {
    pub fn new(entity: Entity) -> MetaEntity {
        MetaEntity {
            entity: entity,
            component_bits: BitvSet::new()
        }
    }

    pub fn reset(mut self) -> MetaEntity {
        self.component_bits.clear();

        self
    }
}