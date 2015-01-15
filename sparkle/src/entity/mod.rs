use std::collections::{VecMap, RingBuf, HashSet, BitvSet};
use std::collections::ring_buf;

use component::ComponentMapper;

pub use self::group::GroupMap;
pub use self::tag::TagMap;

pub mod group;
pub mod tag;

pub type Entity = usize;

#[derive(PartialEq, Eq, Clone)]
pub struct MetaEntity {
    pub entity: Entity,
    pub tag: Option<String>,
    pub groups: HashSet<String>,
    pub components: BitvSet
}

impl MetaEntity {
    pub fn new(entity: Entity) -> MetaEntity {
        MetaEntity {
            entity: entity,
            tag: None,
            groups: HashSet::new(),
            components: BitvSet::new()
        }
    }

    pub fn reset(mut self) -> MetaEntity {
        self.components.clear();
        self.tag = None;
        self.groups.clear();

        self
    }
}

pub struct EntityMapper {
    mentities: MetaEntityMap,
    groups: GroupMap,
    tags: TagMap
}

impl EntityMapper {
    pub fn new() -> EntityMapper {
        EntityMapper {
            mentities: MetaEntityMap::new(),
            groups: GroupMap::new(),
            tags: TagMap::new()
        }
    }

    pub fn create_entity(&mut self) -> Entity {
        self.mentities.create()
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        {
            let mentity = self.mentities.get(entity);
            group::private::forget(&mut self.groups, mentity);
            tag::private::forget(&mut self.tags, mentity);
        }
        self.mentities.remove(entity);
    }

    pub fn get_mentity(&self, entity: Entity) -> &MetaEntity {
        self.mentities.get(entity)
    }

    pub fn get_mentity_mut(&mut self, entity: Entity) -> &mut MetaEntity {
        self.mentities.get_mut(entity)
    }

    pub fn set_group(&mut self, entity: Entity, group: &str) {
        self.groups.insert_in(self.mentities.get_mut(entity), group);
    }

    pub fn unset_group(&mut self,  entity: Entity, group: &str) {
        self.groups.remove_from(self.mentities.get_mut(entity), group);
    }

    pub fn get_group(&mut self, group: &str) -> Vec<Entity> {
        self.groups.get(group)
    }

    pub fn set_tag(&mut self, tag: &str, entity: Entity) {
        self.tags.insert(self.mentities.get_mut(entity), tag);
    }

    pub fn unset_tag(&mut self, entity: Entity) {
        self.tags.remove(self.mentities.get_mut(entity))
    }

    pub fn get_tag(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag)
    }

    pub fn notify_events<O>(&mut self, cm: &mut ComponentMapper, obs: &mut O) where O: EntityObserver {
        let EntityMapper { ref mut mentities, .. } = *self;

        mentities.drain_events_with(|(kind, mentity)| {
            match kind {
                EventKind::Changed => obs.notify_changed(mentity),
                EventKind::Removed => {
                    obs.notify_removed(mentity);
                    ::component::private::forget(cm, mentity);
                }
            }
        });
    }
}

type Event = (EventKind, Entity);

#[derive(Copy, Show)]
enum EventKind {
    Changed,
    Removed
}

struct EventQueue {
    changed_set: HashSet<Entity>,
    events: RingBuf<Event>
}

impl EventQueue {
    fn new() -> EventQueue {
        EventQueue {
            changed_set: HashSet::new(),
            events: RingBuf::new()
        }
    }

    fn changed(&mut self, entity: Entity) {
        if self.changed_set.insert(entity) {
            self.events.push_back((EventKind::Changed, entity))
        }
    }

    fn removed(&mut self, entity: Entity) {
        self.events.push_back((EventKind::Removed, entity))
    }

    fn drain(&mut self) -> EventDrain {
        self.changed_set.clear();
        self.events.drain()
    }
}

type EventDrain<'a> = ring_buf::Drain<'a, Event>;

pub trait EntityObserver {
    fn notify_changed(&mut self, mentity: &MetaEntity);
    fn notify_removed(&mut self, mentity: &MetaEntity);
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

struct MetaEntityMap {
    pool: Pool,
    mentities: VecMap<MetaEntity>,
    events: EventQueue
}

impl MetaEntityMap {
    fn new() -> MetaEntityMap {
        MetaEntityMap {
            pool: Pool::new(),
            mentities: VecMap::new(),
            events: EventQueue::new()
        }
    }

    fn create(&mut self) -> Entity {
        let meta_entity = self.pool.get();
        let entity = meta_entity.entity;
        self.mentities.insert(entity, meta_entity);

        self.events.changed(entity);
        entity
    }

    fn remove(&mut self, entity: Entity) {
        self.events.removed(entity);
    }

    fn get(&self, entity: Entity) -> &MetaEntity {
        get_mentity!(self.mentities, entity)
    }

    fn get_mut(&mut self, entity: Entity) -> &mut MetaEntity {
        self.events.changed(entity);
        get_mentity_mut!(self.mentities, entity)
    }

    fn drain_events_with<'a, F>(&'a mut self, mut func: F)
        where F: for<'b> FnMut((EventKind, &'b MetaEntity))
    {
        let MetaEntityMap {ref mut events, ref mut mentities, ref mut pool} = *self;
        for (kind, entity) in events.drain() {
            func((kind, get_mentity!(mentities, entity)));
            if let EventKind::Removed = kind {
                pool.put(mentities.remove(&entity).unwrap());
            }
        }
    }
}

struct Pool {
    available: Vec<MetaEntity>,
    next_id: usize
}

impl Pool {
    fn new() -> Pool {
        Pool {
            available: Vec::new(),
            next_id: 0
        }
    }

    fn get(&mut self) -> MetaEntity {
        self.available.pop().unwrap_or_else(|| {
            let id = self.next_id;
            self.next_id += 1;

            MetaEntity::new(id)
        }).reset()
    }

    fn put(&mut self, entity: MetaEntity) {
        self.available.push(entity);
    }
}
