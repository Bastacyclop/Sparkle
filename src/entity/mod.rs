use std::collections::{HashSet, BitvSet};
pub use self::pool::Pool;
pub use self::update::{Update, Record, Observer};
pub use self::group::GroupMap;
pub use self::manager::Manager;

pub mod pool;
pub mod update;
pub mod group;
pub mod manager;

pub type Entity = uint;

#[deriving(PartialEq, Eq, Clone)]
pub struct MetaEntity {
    pub entity: Entity,
    pub groups: HashSet<String>,
    pub component_bits: BitvSet
}

impl MetaEntity {
    pub fn new(entity: Entity) -> MetaEntity {
        MetaEntity {
            entity: entity,
            groups: HashSet::new(),
            component_bits: BitvSet::new()
        }
    }

    pub fn reset(mut self) -> MetaEntity {
        self.component_bits.clear();
        self.groups.clear();

        self
    }
}

#[macro_export]
macro_rules! entity(
    ($em:expr, [$($component:expr),+]) => ({
        let entity = $em.create();
        $(
            $em.attach_component(&entity, $component);
        )+

        entity
    })
);