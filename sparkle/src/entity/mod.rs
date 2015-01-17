//! The entity related features.
//!
//! ## Simple entity manipulation
//!
//! Creating and removing entities is pretty simple:
//!
//! ````ignore
//! let entity = em.create_entity();
//! // ...
//! em.remove_entity(entity);
//! ````
//!
//! ## Identification of entities using groups
//!
//! Groups are useful to identify a category of entities.
//! For example, you could have a group of allies:
//!
//! ````ignore
//! em.set_group(wise_mage, "allies");
//! em.set_group(bold_dwarf, "allies");
//! // ...
//! // And somewhere else:
//! let allies = em.get_group("allies");
//! // Though you'll probably prefer using system filters than that function.
//! ````
//! 
//! A group is referred to by a name and can contain multiple entities.
//! Likewise, an entity can belong to multiple groups.
//!
//! ## Identification of entities using tags
//!
//! Tags are useful to identify a specific entity.
//! For example, you could tag the hero of your game:
//!
//! ```ignore
//! em.set_tag(julian, "hero");
//! // ...
//! // And somewhere else:
//! let hero = em.get_tag("hero");
//! ```
//!
//! A tag is referred to by a name and can only tag one entity at a time.
//! Furthermore, an entity can only have one tag at a time.

use std::collections::{VecMap, RingBuf, HashSet, BitvSet};
use std::collections::ring_buf;

use component::ComponentMapper;

use self::group::GroupMap;
use self::tag::TagMap;

mod group;
mod tag;

/// A plain entity identifier.
pub type Entity = usize;

/// An entity and its features.
#[derive(PartialEq, Eq, Clone)]
pub struct MetaEntity {
    pub entity: Entity,
    pub is_awake: bool,
    pub tag: Option<String>,
    pub groups: HashSet<String>,
    pub components: BitvSet
}

impl MetaEntity {
    /// Creates a bare `MetaEntity`.
    fn new(entity: Entity) -> MetaEntity {
        MetaEntity {
            entity: entity,
            is_awake: true,
            tag: None,
            groups: HashSet::new(),
            components: BitvSet::new()
        }
    }

    /// Resets the `MetaEntity`.
    fn reset(mut self) -> MetaEntity {
        self.is_awake = true;
        self.components.clear();
        self.tag = None;
        self.groups.clear();

        self
    }
}

/// An entity mapper using plain `Entity` identifiers, tags and groups.
pub struct EntityMapper {
    mentities: MetaEntityMap,
    groups: GroupMap,
    tags: TagMap
}

impl EntityMapper {
    /// Creates a new `EntityMapper`.
    #[doc(hidden)]
    pub fn new() -> EntityMapper {
        EntityMapper {
            mentities: MetaEntityMap::new(),
            groups: GroupMap::new(),
            tags: TagMap::new()
        }
    }

    /// Creates a new entity.
    pub fn create_entity(&mut self) -> Entity {
        self.mentities.create()
    }

    /// Removes an entity.
    ///
    /// The removal event is recorded and will be notified before any system update.
    pub fn remove_entity(&mut self, entity: Entity) {
        {
            let mentity = self.mentities.get(entity);
            group::private::forget(&mut self.groups, mentity);
            tag::private::forget(&mut self.tags, mentity);
        }
        self.mentities.remove(entity);
    }

    /// Enables an entity
    ///
    /// The entity will be updated by systems again.
    pub fn wake_up(&mut self, entity: Entity) {
        self.mentities.set_awake(entity, true);
    }

    /// Disables an entity
    ///
    /// The entity won't be updated by systems anymore.
    pub fn put_to_sleep(&mut self, entity: Entity) {
        self.mentities.set_awake(entity, false);
    }

    /// Returns a reference to the meta entity.
    pub fn get_mentity(&self, entity: Entity) -> &MetaEntity {
        self.mentities.get(entity)
    }

    /// Returns a mutable reference to the meta entity.
    pub fn get_mentity_mut(&mut self, entity: Entity) -> &mut MetaEntity {
        self.mentities.get_mut(entity)
    }

    /// Inserts an entity into a group.
    pub fn set_group(&mut self, entity: Entity, group: &str) {
        self.groups.insert_in(self.mentities.get_mut(entity), group);
    }

    /// Removes an entity from a group.
    pub fn unset_group(&mut self, entity: Entity, group: &str) {
        self.groups.remove_from(self.mentities.get_mut(entity), group);
    }
    
    /// Clears an entity groups.
    pub fn clear_entity_groups(&mut self, entity: Entity) {
        self.groups.clear_entity(self.mentities.get_mut(entity));
    }

    /// Returns an entity group as a vector.
    pub fn get_group(&mut self, group: &str) -> Vec<Entity> {
        self.groups.get(group)
    }

    /// Sets an entity tag.
    ///
    /// If the entity was already tagged, the previous tag will be overriden and returned.
    /// Panics if the tag was already used.
    pub fn set_tag(&mut self, entity: Entity, tag: &str) -> Option<String> {
        self.tags.insert(self.mentities.get_mut(entity), tag)
    }

    /// Unsets an entity tag.
    pub fn unset_tag(&mut self, entity: Entity) {
        self.tags.remove(self.mentities.get_mut(entity))
    }

    /// Returns the entity tagged by `tag` if it exists.
    ///
    /// This method returns `None` if the tag doesn't exist.
    pub fn try_get_tag(&self, tag: &str) -> Option<Entity> {
        self.tags.get(tag)
    }

    /// Returns the entity tagged by `tag`.
    ///
    /// This method panics if the tag doesn't exist.
    pub fn get_tag(&self, tag: &str) -> Entity {
        self.tags.get(tag).expect(format!("Failed to find an entity with tag {}", tag).as_slice())
    }

    /// Notify all entity events that occurred to an observer.
    #[doc(hidden)]
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

/// An event that occurred to a certain entity.
type Event = (EventKind, Entity);

/// The different kinds of entity-related events that can occur.
#[derive(Copy, PartialEq, Show)]
enum EventKind {
    Changed,
    Removed
}

/// A queue recording entity-related events.
///
/// Change events are protected against duplicates.
struct EventQueue {
    changed_set: HashSet<Entity>,
    removed_set: HashSet<Entity>,
    events: RingBuf<Event>
}

impl EventQueue {
    /// Creates an empty `EventQueue`.
    fn new() -> EventQueue {
        EventQueue {
            changed_set: HashSet::new(),
            removed_set: HashSet::new(),
            events: RingBuf::new()
        }
    }

    /// Records the change of an entity, ignoring duplicates.
    fn changed(&mut self, entity: Entity) {
        if self.changed_set.insert(entity) {
            if !self.removed_set.contains(&entity) {
                self.events.push_back((EventKind::Changed, entity))
            }
        }
    }

    /// Records the removal of an entity. 
    fn removed(&mut self, entity: Entity) {
        if self.removed_set.insert(entity) {
            self.events.push_back((EventKind::Removed, entity))
        }
    }

    /// Drains all recorded events.
    fn drain(&mut self) -> EventDrain {
        self.changed_set.clear();
        self.removed_set.clear();
        self.events.drain()
    }
}

type EventDrain<'a> = ring_buf::Drain<'a, Event>;

/// Observes entity-related events.
#[doc(hidden)]
pub trait EntityObserver {
    /// Notifies the observer that an entity was changed.
    fn notify_changed(&mut self, mentity: &MetaEntity);
    /// Notifies the observer that an entity was removed.
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

/// A map of meta entities.
struct MetaEntityMap {
    pool: Pool,
    mentities: VecMap<MetaEntity>,
    events: EventQueue
}

impl MetaEntityMap {
    /// Creates a new map of meta entities.
    fn new() -> MetaEntityMap {
        MetaEntityMap {
            pool: Pool::new(),
            mentities: VecMap::new(),
            events: EventQueue::new()
        }
    }

    /// Creates a new entity.
    fn create(&mut self) -> Entity {
        let meta_entity = self.pool.get();
        let entity = meta_entity.entity;
        self.mentities.insert(entity, meta_entity);

        self.events.changed(entity);
        entity
    }

    /// Removes an entity.
    ///
    /// The removal event is recorded and will be treated later.  
    /// The entity effective removal is delayed until then.
    fn remove(&mut self, entity: Entity) {
        self.events.removed(entity);
    }

    /// Enable or disable an entity.
    fn set_awake(&mut self, entity: Entity, awake: bool) {
        self.get_mut(entity).is_awake = awake;
    }

    /// Returns a reference to a meta entity.
    fn get(&self, entity: Entity) -> &MetaEntity {
        get_mentity!(self.mentities, entity)
    }

    /// Returns a mutable reference to a meta entity.
    fn get_mut(&mut self, entity: Entity) -> &mut MetaEntity {
        self.events.changed(entity);
        get_mentity_mut!(self.mentities, entity)
    }

    /// Drains the entity-related events, applying `func` for each event.
    ///
    /// In case of a removal event, this is where the effective removal occurs.
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

/// A pool from where we can draw bare entities.
///
/// A drawn entity can be put back in to be recycled.
struct Pool {
    available: Vec<MetaEntity>,
    next_id: usize
}

impl Pool {
    /// Creates an empty `Pool`.
    fn new() -> Pool {
        Pool {
            available: Vec::new(),
            next_id: 0
        }
    }

    /// Retrieves a bare entity from the pool.
    fn get(&mut self) -> MetaEntity {
        self.available.pop().unwrap_or_else(|| {
            let id = self.next_id;
            self.next_id += 1;

            MetaEntity::new(id)
        }).reset()
    }

    /// Puts an entity back in the pool.
    fn put(&mut self, entity: MetaEntity) {
        self.available.push(entity);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::{EventQueue, EventKind, Pool};
    
    #[test]
    fn event_queue_changed() {
        let mut queue = EventQueue::new();
        
        queue.changed(0);
        assert_eq!(queue.events.pop_back(), Some((EventKind::Changed, 0)));
    }
    
    #[test]
    fn event_queue_changed_dup() {
        let mut queue = EventQueue::new();
        
        queue.changed(0);
        queue.changed(0);
        assert_eq!(queue.events.pop_back(), Some((EventKind::Changed, 0)));
        assert_eq!(queue.events.pop_back(), None);
    }
    
    #[test]
    fn event_queue_removed() {
        let mut queue = EventQueue::new();
        
        queue.removed(0);
        assert_eq!(queue.events.pop_back(), Some((EventKind::Removed, 0)));
    }
    
    #[test]
    fn event_queue_changed_when_removed() {
        let mut queue = EventQueue::new();
        
        queue.removed(0);
        queue.changed(0);
        assert_eq!(queue.events.pop_back(), Some((EventKind::Removed, 0)));
        assert_eq!(queue.events.pop_back(), None);
    }
    
    #[test]
    fn event_queue_drain() {
        let mut queue = EventQueue::new();
        
        queue.changed(0);
        queue.removed(1);
        queue.changed(2);
        queue.changed(3);
        
        let expected = [
            (EventKind::Changed, 0),
            (EventKind::Removed, 1),
            (EventKind::Changed, 2),
            (EventKind::Changed, 3)
        ];
        
        let drained: Vec<(EventKind, Entity)> = queue.drain().collect();
        assert_eq!(drained, expected);
        assert_eq!(queue.events.len(), 0);
        assert_eq!(queue.changed_set.len(), 0);
        assert_eq!(queue.removed_set.len(), 0);
    }
    
    
    
    #[test]
    fn pool_regular_get() {
        let mut pool = Pool::new();
        for i in 0..10 {
            assert_eq!(pool.get().entity, i);
        }
    }
    
    #[test]
    fn pool_put_and_get_recycled() {
        let mut pool = Pool::new();
        let recycled = pool.get();
        
        assert_eq!(recycled.entity, 0);
        assert_eq!(pool.get().entity, 1);
        pool.put(recycled);
        assert_eq!(pool.get().entity, 0);
        assert_eq!(pool.get().entity, 2);
    }
}
