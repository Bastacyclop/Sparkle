use std::collections::{VecMap, HashSet, BitvSet};
use std::rc::Rc;
use std::cell::RefCell;

pub use self::pool::Pool;
pub use self::event::{Event, Queue, Observer};
pub use self::tag::TagMap;
pub use self::manager::Manager;

pub mod pool;
pub mod event;
pub mod tag;
pub mod manager;

pub type Entity = uint;

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

pub struct MetaEntityMap(Rc<RefCell<VecMap<MetaEntity>>>);

impl MetaEntityMap {
    pub fn new() -> MetaEntityMap {
        MetaEntityMap(Rc::new(RefCell::new(VecMap::new())))
    }

    pub fn apply_to(&self, entity: &Entity, func: |&mut MetaEntity|) {
        self.0.borrow_mut().get_mut(entity).map(|mentity| func(mentity));
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