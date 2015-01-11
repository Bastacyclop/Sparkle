use std::collections::{VecMap, HashSet, BitvSet};
use self::pool::Pool;

pub use self::manager::Manager;
pub use self::group::GroupMap;
pub use self::tag::TagMap;

pub mod pool;
pub mod manager;
pub mod group;
pub mod tag;
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

macro_rules! get_mentity {
    ($mentities:expr, $entity:expr) => (
        $mentities.get(&$entity)
                  .expect(format!("There is no meta information for {}", $entity).as_slice())
    )
}

macro_rules! get_mentity_mut {
    ($mentities:expr, $entity:expr) => (
        $mentities.get_mut(&$entity)
                  .expect(format!("There is no meta information for {}", $entity).as_slice())
    )
}

pub struct MetaEntityMap {
    pool: Pool,
    mentities: VecMap<MetaEntity>,
    events: event::Queue
}

impl MetaEntityMap {
    pub fn new() -> MetaEntityMap {
        MetaEntityMap {
            pool: Pool::new(),
            mentities: VecMap::new(),
            events: event::Queue::new()
        }
    }

    pub fn create(&mut self) -> Entity {
        let meta_entity = self.pool.get();
        let entity = meta_entity.entity;
        self.mentities.insert(entity, meta_entity);

        self.events.changed(entity);
        entity
    }

    pub fn remove(&mut self, entity: Entity) {
        self.events.removed(entity);
    }

    pub fn get(&self, entity: Entity) -> &MetaEntity {
        get_mentity!(self.mentities, entity)
    }

    pub fn get_mut(&mut self, entity: Entity) -> &mut MetaEntity {
        self.events.changed(entity);
        get_mentity_mut!(self.mentities, entity)
    }

    pub fn drain_events_with<'a, W>(&'a mut self, mut last_wish: W)
        where W: for<'b> FnMut((event::Kind, &'b MetaEntity))
    {
        let MetaEntityMap {ref mut events, ref mut mentities, ref mut pool} = *self;
        for (kind, entity) in events.drain() {
            last_wish((kind, get_mentity!(mentities, entity)));
            pool.put(mentities.remove(&entity).unwrap());
        }
    }
}

pub trait Observer {
    fn notify_changed(&mut self, mentity: &MetaEntity);
    fn notify_removed(&mut self, mentity: &MetaEntity);
}
